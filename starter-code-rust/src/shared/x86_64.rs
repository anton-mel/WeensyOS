#![allow(dead_code)]
use core::arch::asm;
use x86_64::instructions::port::Port;

// x86.h: C code to interface with x86 hardware and CPU.
//
//   Contents:
//   - Memory and interrupt constants.
//   - x86_registers: Used in process descriptors to store x86 registers.
//   - x86 functions: C function wrappers for useful x86 instructions.
//   - Hardware structures: C structures and constants for initializing
//     x86 hardware, including the interrupt descriptor table.

// Paged memory constants
pub const PAGEOFFBITS: usize = 12;                          // # bits in page offset
pub const PAGESIZE: usize = 1 << PAGEOFFBITS;               // Size of page in bytes
pub const PAGEINDEXBITS: usize = 9;                         // # bits in a page index level
pub const NPAGETABLEENTRIES: usize = 1 << PAGEINDEXBITS;    // # entries in page table page

pub fn pagernumber(ptr: *const u8) -> usize {
    (ptr as usize) >> PAGEOFFBITS
}

pub fn pageaddress(pn: usize) -> usize {
    pn << PAGEOFFBITS
}

pub type X86_64PageEntry = u64;

#[repr(align(4096))]
pub struct X86_64PageTable {
    entry: [X86_64PageEntry; NPAGETABLEENTRIES],
}

pub fn pageindex(addr: usize, level: u32) -> usize {
    assert!(level <= 3);
    (addr >> (PAGEOFFBITS + ((3 - level) as usize) * PAGEINDEXBITS)) & 0x1FF
}

pub const PAGEOFFMASK: usize = PAGESIZE - 1;

pub fn pageoffset(addr: usize) -> usize {
    addr & PAGEOFFMASK
}

pub fn pte_addr(pageentry: X86_64PageEntry) -> usize {
    (pageentry & !0xFFF) as usize
}

pub fn pte_flags(pageentry: X86_64PageEntry) -> X86_64PageEntry {
    pageentry & 0xFFF
}

// Page table entry flags
pub const PTE_P: X86_64PageEntry = 1;    // entry is Present
pub const PTE_W: X86_64PageEntry = 2;    // entry is Writeable
pub const PTE_U: X86_64PageEntry = 4;    // entry is User-accessible
pub const PTE_A: X86_64PageEntry = 32;   // entry was Accessed (read/written)
pub const PTE_D: X86_64PageEntry = 64;   // entry was Dirtied (written)
pub const PTE_PS: X86_64PageEntry = 128; // entry has a large Page Size

// Page fault error flags
pub const PFERR_PRESENT: u32 = 0x1;     // Fault happened due to a protection violation (rather than due to a missing page)
pub const PFERR_WRITE: u32 = 0x2;       // Fault happened on a write
pub const PFERR_USER: u32 = 0x4;        // Fault happened in an application (user mode) (rather than kernel)

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

#[repr(C)]
pub struct X86_64Registers {
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
    pub reg_padding2: [u16; 3],
    pub reg_rflags: u64,
    pub reg_rsp: u64,
    pub reg_ss: u16,
    pub reg_padding3: [u16; 3],
}

pub fn breakpoint() {
    unsafe {
        asm!("int3");
    }
}

pub fn inb(port: u16) -> u8 {
    let mut port = Port::new(port);
    unsafe { port.read() }
}

pub fn insb(port: u16, addr: *mut u8, cnt: usize) {
    unsafe {
        let mut cur_addr = addr;

        for _ in 0..cnt {
            let byte = inb(port);
            cur_addr.write(byte);
            cur_addr = cur_addr.offset(1);
        }
    }
}

pub fn inw(port: u16) -> u16 {
    let mut data: u16;
    unsafe {
        asm!("inw %dx, %ax", in("dx") port, out("ax") data);
    }
    data
}

pub fn insw(port: u16, addr: *mut u16, cnt: usize) {
    unsafe {
        asm!("cld; repne; insw", in("dx") port, in("rdi") addr, in("rcx") cnt, options(nostack, preserves_flags));
    }
}

pub fn inl(port: u16) -> u32 {
    let mut data: u32;
    unsafe {
        asm!("inl %dx, %eax", in("dx") port, out("eax") data);
    }
    data
}

pub fn insl(port: u16, addr: *mut u32, cnt: usize) {
    unsafe {
        asm!("cld; repne; insl", in("dx") port, in("rdi") addr, in("rcx") cnt, options(nostack, preserves_flags));
    }
}

pub fn outb(port: u16, data: u8) {
    let mut port = Port::new(port);
    unsafe { port.write(data) }
}

pub fn outsb(port: u16, addr: *const u8, cnt: usize) {
    unsafe {
        asm!("cld; repne; outsb", in("dx") port, in("rsi") addr, in("rcx") cnt, options(nostack, preserves_flags));
    }
}

pub fn outw(port: u16, data: u16) {
    unsafe {
        asm!("outw %ax, %dx", in("ax") data, in("dx") port);
    }
}

pub fn outsw(port: u16, addr: *const u16, cnt: usize) {
    unsafe {
        asm!("cld; repne; outsw", in("dx") port, in("rsi") addr, in("rcx") cnt, options(nostack, preserves_flags));
    }
}

pub fn outl(port: u16, data: u32) {
    unsafe {
        asm!("outl %eax, %dx", in("eax") data, in("dx") port);
    }
}

pub fn outsl(port: u16, addr: *const u32, cnt: usize) {
    unsafe {
        asm!("cld; repne; outsl", in("dx") port, in("rsi") addr, in("rcx") cnt, options(nostack, preserves_flags));
    }
}

pub fn invlpg(addr: *const u8) {
    unsafe {
        asm!("invlpg ({})", in(reg) addr, options(nostack, preserves_flags));
    }
}

pub fn lidt(p: *const u8) {
    unsafe {
        asm!("lidt ({})", in(reg) p, options(nostack, preserves_flags));
    }
}

pub fn lldt(sel: u16) {
    unsafe {
        asm!("lldt {0:x}", in(reg) sel);
    }
}

pub fn ltr(sel: u16) {
    unsafe {
        asm!("ltr {0:x}", in(reg) sel);
    }
}

pub fn lcr0(val: u32) {
    unsafe {
        asm!("mov cr0, {0:x}", in(reg) val);
    }
}

pub fn rcr0() -> u32 {
    let val: u32;
    unsafe {
        asm!("mov {0:x}, cr0", out(reg) val);
    }
    val
}

pub fn rcr2() -> usize {
    let val: usize;
    unsafe {
        asm!("mov {}, cr2", out(reg) val);
    }
    val
}

pub fn lcr3(val: usize) {
    unsafe {
        asm!("mov cr3, {}", in(reg) val);
    }
}

pub fn rcr3() -> usize {
    let val: usize;
    unsafe {
        asm!("mov {}, cr3", out(reg) val);
    }
    val
}

pub fn lcr4(val: u32) {
    unsafe {
        asm!("mov cr4, {0:x}", in(reg) val);
    }
}

pub fn rcr4() -> u32 {
    let val: u32;
    unsafe {
        asm!("mov {0:x}, cr4", out(reg) val);
    }
    val
}

pub fn read_rbp() -> usize {
    let val: usize;
    unsafe {
        asm!("mov {}, rbp", out(reg) val);
    }
    val
}

pub fn read_rsp() -> usize {
    let val: usize;
    unsafe {
        asm!("mov {}, rsp", out(reg) val);
    }
    val
}

// use raw_cpuid::CpuId;
// pub fn cpuid(info: u32, eaxp: &mut u32, ebxp: &mut u32, ecxp: &mut u32, edxp: &mut u32) {
//     let cpuid = CpuId::new();
//     if let Some(result) = cpuid.get_processor_brand_string() {
//         *eaxp = result.as_bytes()[0..4].try_into().unwrap();
//         *ebxp = result.as_bytes()[4..8].try_into().unwrap();
//         *ecxp = result.as_bytes()[8..12].try_into().unwrap();
//         *edxp = result.as_bytes()[12..16].try_into().unwrap();
//     } else {
//         *eaxp = 0;
//         *ebxp = 0;
//         *ecxp = 0;
//         *edxp = 0;
//     }
// }


pub fn rdtsc() -> u64 {
    let rax: u32;
    let rdx: u32;
    unsafe {
        asm!("rdtsc", out("eax") rax, out("edx") rdx);
    }
    ((rdx as u64) << 32) | (rax as u64)
}

pub fn read_eflags() -> u64 {
    let val: u64;
    unsafe {
        asm!("pushfq; pop {}", out(reg) val);
    }
    val
}

pub fn write_eflags(eflags: u64) {
    unsafe {
        asm!("push {}; popfq", in(reg) eflags);
    }
}
