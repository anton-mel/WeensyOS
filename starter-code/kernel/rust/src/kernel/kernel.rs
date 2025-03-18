// kernel.c
//
//    This is the kernel.

use crate::*;
use crate::kernel::*;
use core::ptr::NonNull;
use core::ops::AddAssign;

// INITIAL PHYSICAL MEMORY LAYOUT
//
//  +-------------- Base Memory --------------+
//  v                                         v
// +-----+--------------------+----------------+--------------------+---------/
// |     | Kernel      Kernel |       :    I/O | App 1        App 1 | App 2
// |     | Code + Data  Stack |  ...  : Memory | Code + Data  Stack | Code ...
// +-----+--------------------+----------------+--------------------+---------/
// 0  0x40000              0x80000 0xA0000 0x100000             0x140000
//                                             ^
//                                             | \___ PROC_SIZE ___/
//                                      PROC_START_ADDR

const PROC_SIZE: usize = 0x40000;   // initial state only
const HZ: u32 = 100;                // timer interrupt frequency (interrupts/sec)

// PAGEINFO
//
//    The pageinfo[] array keeps track of information about each physical page.
//    There is one entry per physical page.
//    `pageinfo[pn]` holds the information for physical page number `pn`.
//    You can get a physical page number from a physical address `pa` using
//    `PAGENUMBER(pa)`. (This also works for page table entries.)
//    To change a physical page number `pn` into a physical address, use
//    `PAGEADDRESS(pn)`.
//
//    pageinfo[pn].refcount is the number of times physical page `pn` is
//      currently referenced. 0 means it's free.
//    pageinfo[pn].owner is a constant indicating who owns the page.
//      PO_KERNEL means the kernel, PO_RESERVED means reserved memory (such
//      as the console), and a number >=0 means that process ID.
//
//    pageinfo_init() sets up the initial pageinfo[] state.

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PhysicalPageInfo {
    pub owner: i8,
    pub refcount: i8,
}

impl AddAssign<i8> for PhysicalPageInfo {
    fn add_assign(&mut self, other: i8) {
        self.refcount += other;
    }
}

#[repr(i8)]
#[allow(unused)]
#[derive(PartialEq, Clone)]
pub enum PageOwner {
    PoFree = 0,         // this page is free
    PoReserved = -1,    // this page is reserved memory
    PoKernel = -2,      // this page is used by the kernel
}

// kernel(command)
//    Initialize the hardware and processes and start running. The `command`
//    string is an optional string passed from the boot loader.

#[no_mangle]
pub unsafe extern "sysv64" fn kernel(command: Option<NonNull<u8>>) {
    hardware_init();
    pageinfo_init();
    console_clear();
    timer_init(HZ);

    let proc_ptr = processes.as_mut_ptr();
    let proc_size = NPROC * core::mem::size_of::<Proc>();
    core::ptr::write_bytes(proc_ptr as *mut u8, 0, proc_size);
    let _console = console.as_ptr() as usize; // optional

    for i in 0..NPROC {
        processes[i].p_pid = i as i32;
        processes[i].p_state = P_FREE;
    }

    match command {
        Some(ptr) => {
            let cmd = ptr.as_ptr() as *const core::ffi::c_char;

            if strcmp(cmd, b"fork\0".as_ptr() as *const i8) == 0 {
                process_setup(1, 4);
            } else if strcmp(cmd, b"forkexit\0".as_ptr() as *const i8) == 0 {
                process_setup(1, 5);
            } else if strcmp(cmd, b"test\0".as_ptr() as *const i8) == 0 {
                process_setup(1, 6);
            } else if strcmp(cmd, b"test2\0".as_ptr() as *const i8) == 0 {
                for i in 1..=2 {
                    process_setup(i, 6);
                }
            } else {
                for i in 1..=4 {
                    process_setup(i, i - 1);
                }
            }
        }
        None => {
            for i in 1..=4 {
                process_setup(i, i - 1);
            }
        }
    }

    run(&mut processes[1]);
}

// process_setup(pid, program_number)
//    Load application program `program_number` as process number `pid`.
//    This loads the application's code and data into memory, sets its
//    %rip and %rsp, gives it a stack page, and marks it as runnable.

pub unsafe fn process_setup(pid: usize, pn: usize) {
    process_init(&mut processes[pid], 0);
    processes[pid].p_pagetable = kernel_pagetable;
    pageinfo[page_number(kernel_pagetable as u64) as usize].refcount += 1; //increase refcount since kernel_pagetable was used

    let r = program_load(&mut processes[pid], pn as i32, core::ptr::null());
    assert!(r >= 0); 

    processes[pid].p_registers.reg_rsp = PROC_START_ADDR + (PROC_SIZE * pid) as u64;
    let stack_page = (processes[pid].p_registers.reg_rsp - PAGESIZE) as usize;
    assign_physical_page(stack_page, pid as i8);
    virtual_memory_map(processes[pid].p_pagetable, stack_page, stack_page, 
                PAGESIZE as usize, (PTE_P | PTE_W | PTE_U) as i32);
    processes[pid].p_state = P_RUNNABLE;
}

// exception(reg)
//    Exception handler (for interrupts, traps, and faults).
//
//    The register values from exception time are stored in `reg`.
//    The processor responds to an exception by saving application state on
//    the kernel's stack, then jumping to kernel assembly code (in
//    k-exception.S). That code saves more registers on the kernel's stack,
//    then calls exception().
//
//    Note that hardware interrupts are disabled whenever the kernel is running.

#[no_mangle]
pub unsafe fn exception(reg: &mut x86_64_registers) {    
    // Copy the saved registers into the `current` process descriptor
    // and always use the kernel's page table.
    (*current).p_registers = *reg;
    set_pagetable(kernel_pagetable);

    // It can be useful to log events using `log_printf`.
    // Events logged this way are stored in the host's `log.txt` file.
    /*log_printf("proc %d: exception %d\n", current->p_pid, reg->reg_intno);*/

    // Show the current cursor location and memory state
    // (unless this is a kernel fault).
    console_show_cursor(cursorpos);
    if (reg.reg_intno != INT_PAGEFAULT as u64 && reg.reg_intno != INT_GPF as u64) // no error due to pagefault or general fault
        || (reg.reg_err & PFERR_USER as u64) != 0 // pagefault error in user mode
    {
        check_virtual_memory();
        if disp_global != 0 {
            memshow_physical();
            memshow_virtual_animate();
        }
    }

    // If Control-C was typed, exit the virtual machine.
    check_keyboard();
    
    // Handle the exception based on the interrupt number.
    match reg.reg_intno as u32 {
        INT_SYS_PANIC => {
            // rdi stores pointer for msg string
            let addr = (*current).p_registers.reg_rdi;
            if addr == 0 {
                panic!("(exception) current process has not been set yet");
            } else {
                let map = virtual_memory_lookup((*current).p_pagetable, addr as usize);
                let mut msg = [0u8; 160];
                memcpy(
                    &mut msg as *mut [u8; 160] as *mut core::ffi::c_void, 
                    map.pa as *const core::ffi::c_void, 
                    160
                );
                panic!("{:?}", msg);
                /* will not be reached */
            }
        }
        INT_SYS_GETPID => {
            (*current).p_registers.reg_rax = (*current).p_pid as u64;       
        }
        INT_SYS_YIELD => {
            schedule();
            /* will not be reached */
        }
        INT_SYS_PAGE_ALLOC => {
            let addr = (*current).p_registers.reg_rdi;
            let r = assign_physical_page(
                addr as usize, 
                (*current).p_pid as i8,
            );
            if r >= 0 {
                virtual_memory_map(
                    (*current).p_pagetable, 
                    addr as usize,
                    addr as usize,
                    PAGESIZE as usize,
                    (PTE_P | PTE_W | PTE_U) as i32,
                );
            }
            (*current).p_registers.reg_rax = r as u64;
        }
        INT_SYS_MAPPING => {
            syscall_mapping(&mut *current);
        }
        INT_SYS_MEM_TOG => {
            syscall_mem_tog(&mut *current);
        }
        INT_TIMER => {
            ticks += 1;
            schedule();
            /* will not be reached */
        }
        INT_PAGEFAULT => {
            // Analyze faulting address and access type.
            let addr = asm_rcr2();
            let operation = if reg.reg_err & PFERR_WRITE as u64 != 0 { "write" } else { "read" };
            let problem = if reg.reg_err & PFERR_PRESENT as u64 != 0 { "protection problem" } else { "missing page" };
            
            if reg.reg_err & PFERR_USER as u64 == 0 {
                panic!("Kernel page fault for {:?} ({} {}, rip={:?})!",
                    addr, operation, problem, reg.reg_rip);
            }

            // TODO
            console_printf(cpos!(24, 0), 0x0C00, 
                "Process page fault!".as_ptr() as *const u8);

            (*current).p_state = P_BROKEN;
        }
        _ => {
            default_exception(&mut *current);
            /* will not be reached */
        }
    }

    // Return to the current process (or run something else).
    if (*current).p_state == P_RUNNABLE {
        run(&mut *current);
    } else {
        schedule();
    }
}
