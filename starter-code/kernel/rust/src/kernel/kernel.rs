// kernel.c
//
//    This is the kernel.

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

#[allow(dead_code)]
const PROC_SIZE: usize = 0x40000;   // initial state only
const HZ: u32 = 100;                // timer interrupt frequency (interrupts/sec)

// PAGEINFO
//
//    The pageinfo[] array keeps track of information about each physical page.
//    There is one entry per physical page.
//    `pageinfo[pn]` holds the information for physical page number `pn`.
//    You can get a physical page number from a physical address `pa` using
//    `page_number(pa)`. (This also works for page table entries.)
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

    for i in 0..NPROC {
        processes[i].p_pid = i as i32;
        processes[i].p_state = P_FREE;
    }

    match command {
        Some(ptr) => {
            let cmd = ptr.as_ptr() as *const core::ffi::c_char;

            if strcmp(cmd, b"malloc\0".as_ptr() as *const i8) == 0 {
                process_setup(1, 1);
            } else if strcmp(cmd, b"alloctests\0".as_ptr() as *const i8) == 0 {
                process_setup(1, 2);
            } else if strcmp(cmd, b"test\0".as_ptr() as *const i8) == 0 {
                process_setup(1, 3);
            } else if strcmp(cmd, b"test2\0".as_ptr() as *const i8) == 0 {
                for i in 1..=2 {
                    process_setup(i, 3);
                }
            } else {
                process_setup(1, 0);
            }
        }
        None => {
            process_setup(1, 0)
        }
    }
    
    run(&mut processes[1]);
}
