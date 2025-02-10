// Bindigns to x86_64.h: Rust-C code to interface with x86 hardware and CPU.
//
//   Contents:
//   - Memory and interrupt constants.
//   - x86_registers: Used in process descriptors to store x86 registers.
//   - x86 functions: C function wrappers for useful x86 instructions.
//   - Hardware structures: C structures and constants for initializing
//     x86 hardware, including the interrupt descriptor table.

use core::arch::asm;
use core::ffi::c_uint;
use core::ffi::c_int;
use core::assert;

use crate::bindings::bindings_kernel::P_FREE;
use crate::bindings::bindings_kernel::Procstate;

pub type X86_64PageentryT = u64;
pub type ProcstateT = c_uint;
pub type PidT = c_int;

pub const PTE_FLAGS_MASK: u64 = 0xFFF;
pub const PAGE_OFF_MASK: u64 = PAGESIZE - 1;

// Paged memory constants
pub const PAGEOFFBITS: u64 = 12;                          // # bits in page offset
pub const PAGESIZE: u64 = 1 << PAGEOFFBITS;               // Size of page in bytes
pub const PAGEINDEXBITS: u64 = 9;                         // # bits in a page index level
pub const NPAGETABLEENTRIES: u64 = 1 << PAGEINDEXBITS;    // # entries in page table page

// Functions for page number and address
#[inline]
pub fn page_number(pa: u64) -> c_int {
    (pa >> PAGEOFFBITS) as c_int
}

#[inline]
pub fn page_address(pn: c_int) -> u64 {
    (pn as u64) << PAGEOFFBITS
}

#[inline]
pub fn pageindex(addr: usize, level: usize) -> usize {
    (addr >> (PAGEOFFBITS as usize + (3 - level) * PAGEINDEXBITS as usize)) & 0x1FF
}

// Macros for page index levels
pub fn l1pageindex(addr: usize) -> usize {
    pageindex(addr, 3)
}

pub fn l2pageindex(addr: usize) -> usize {
    pageindex(addr, 2)
}

pub fn l3pageindex(addr: usize) -> usize {
    pageindex(addr, 1)
}

pub fn l4pageindex(addr: usize) -> usize {
    pageindex(addr, 0)
}

// Page offset mask and offset function
pub const PAGEOFFMASK: u64 = PAGESIZE - 1;

#[inline]
pub fn page_offset(addr: u64) -> u64 {
    addr & PAGEOFFMASK
}

// The physical address contained in a page table entry
#[inline]
pub fn pte_addr(pageentry: X86_64PageentryT) -> X86_64PageentryT {
    pageentry & !0xFFF
}

// Page table entry flags
#[inline]
pub fn pte_flags(pageentry: X86_64PageentryT) -> X86_64PageentryT {
    pageentry & 0xFFF
}

// Page table entry flags
// - Permission flags: define whether page is accessible
pub const PTE_P: X86_64PageentryT = 1;      // entry is Present
pub const PTE_W: X86_64PageentryT = 2;      // entry is Writeable
pub const PTE_U: X86_64PageentryT = 4;      // entry is User-accessible
// - Accessed flags: automatically turned on by processor
pub const PTE_A: X86_64PageentryT = 32;     // entry was Accessed (read/written)
pub const PTE_D: X86_64PageentryT = 64;     // entry was Dirtied (written)
pub const PTE_PS: X86_64PageentryT = 128;   // entry has a large Page Size
// - There are other flags too!

// Page fault error flags
// These bits are stored in x86_registers::reg_err after a page fault trap.
pub const PFERR_PRESENT: u8 = 0x1;   // Fault happened due to a protection violation (rather than due to a missing page)
pub const PFERR_WRITE: u8 = 0x2;     // Fault happened on a write
pub const PFERR_USER: u8 = 0x4;      // Fault happened in an application (user mode) (rather than kernel)

#[repr(C)]
#[repr(align(4096))]
#[derive(Debug, Copy, Clone)]
pub struct x86_64_pagetable {
    pub entry: [X86_64PageentryT; 512usize],
}

impl x86_64_pagetable {
    pub fn new() -> Self {
        x86_64_pagetable {
            entry: [0; 512],
        }
    }
}

unsafe impl Send for x86_64_pagetable {}
unsafe impl Sync for x86_64_pagetable {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x86_64_registers {
    pub reg_rax: u64,
    pub reg_rcx: u64,
    pub reg_rdx: u64,
    pub reg_rbx: u64,
    pub reg_rbp: u64,
    pub reg_rsi: u64,
    pub reg_rdi: u64,
    pub reg_r8: u64,
    pub reg_r9: u64,
    pub reg_r10: u64,
    pub reg_r11: u64,
    pub reg_r12: u64,
    pub reg_r13: u64,
    pub reg_r14: u64,
    pub reg_r15: u64,
    pub reg_fs: u64,
    pub reg_gs: u64,
    pub reg_intno: u64,
    pub reg_err: u64,
    pub reg_rip: u64,
    pub reg_cs: u16,
    pub reg_padding2: [u16; 3usize],
    pub reg_rflags: u64,
    pub reg_rsp: u64,
    pub reg_ss: u16,
    pub reg_padding3: [u16; 3usize],
}

impl Default for x86_64_registers {
    fn default() -> Self {
        x86_64_registers {
            reg_rax: 0,
            reg_rcx: 0,
            reg_rdx: 0,
            reg_rbx: 0,
            reg_rbp: 0,
            reg_rsi: 0,
            reg_rdi: 0,
            reg_r8: 0,
            reg_r9: 0,
            reg_r10: 0,
            reg_r11: 0,
            reg_r12: 0,
            reg_r13: 0,
            reg_r14: 0,
            reg_r15: 0,
            reg_fs: 0,
            reg_gs: 0,
            reg_intno: 0,
            reg_err: 0,
            reg_rip: 0,
            reg_cs: 0,
            reg_padding2: [0; 3],
            reg_rflags: 0,
            reg_rsp: 0,
            reg_ss: 0,
            reg_padding3: [0; 3],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Proc {
    pub p_pid: PidT,
    pub p_registers: x86_64_registers,
    pub p_state: ProcstateT,
    pub p_pagetable: *mut x86_64_pagetable,
    pub display_status: u8,
}

unsafe impl Send for Proc {}
unsafe impl Sync for Proc {}

impl Default for Proc {
    fn default() -> Self {
        Proc {
            p_pid: 0,
            p_registers: x86_64_registers::default(),
            p_state: P_FREE,
            p_pagetable: core::ptr::null_mut(),
            display_status: 0,
        }
    }
}

impl Proc {
    pub fn new(pid: PidT, state: Procstate) -> Self {
        let mut proc = Proc::default();
        proc.p_pid = pid;
        proc.p_state = state;
        proc
    }
}

// struct vamapping object
// used to store mapping information by virtual_memory_lookup and
// other kernel functions
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VAMapping {
    pub pn: i32,
    pub pa: u64,
    pub perm: i32,
}

impl VAMapping {
    pub fn default() -> Self {
        VAMapping {
            pn: -1,         // physical page number; -1 if unmapped
            pa: u64::MAX,   // physical address; (uintptr_t) -1 if unmapped
            perm: 0,        // permissions; 0 if unmapped
        }
    }
}

// Interrupt numbers
pub const INT_DIVIDE: u32 = 0x0;        // Divide error
pub const INT_DEBUG: u32 = 0x1;         // Debug exception
pub const INT_BREAKPOINT: u32 = 0x3;    // Breakpoint
pub const INT_OVERFLOW: u32 = 0x4;      // Overflow
pub const INT_BOUNDS: u32 = 0x5;        // Bounds check
pub const INT_INVALIDOP: u32 = 0x6;     // Invalid opcode
pub const INT_DOUBLEFAULT: u32 = 0x8;   // Double fault
pub const INT_INVALIDTSS: u32 = 0xa;    // Invalid TSS
pub const INT_SEGMENT: u32 = 0xb;       // Segment not present
pub const INT_STACK: u32 = 0xc;         // Stack exception
pub const INT_GPF: u32 = 0xd;           // General protection fault
pub const INT_PAGEFAULT: u32 = 0xe;     // Page fault

pub const INT_SYS: u32 = 48;
pub const INT_SYS_PANIC: u32 = 48;
pub const INT_SYS_GETPID: u32 = 49;
pub const INT_SYS_YIELD: u32 = 50;
pub const INT_SYS_PAGE_ALLOC: u32 = 51;
pub const INT_SYS_FORK: u32 = 52;
pub const INT_SYS_EXIT: u32 = 53;
pub const INT_SYS_MAPPING: u32 = 54;
pub const INT_SYS_MEM_TOG: u32 = 56;
pub const INT_SYS_BRK: u32 = 57;
pub const INT_SYS_SBRK: u32 = 58;


/// Set the CR3 register (page table base register).
pub fn lcr3(val: usize) {
    use x86_64::structures::paging::PhysFrame;
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::Size4KiB;
    // SAFETY: Writing to CR3 is inherently unsafe
    // because it directly affects memory translation.
    unsafe {
        let frame = PhysFrame::<Size4KiB>::containing_address(x86_64::PhysAddr::new(val as u64));
        Cr3::write(frame, x86_64::registers::control::Cr3Flags::empty());
    }
}

#[macro_export]
macro_rules! cpos {
    ($row:expr, $col:expr) => {
        (($row) * 80 + ($col))
    };
}
