
use core::mem::transmute;
use core::arch::global_asm;
use crate::shared::elf::{ElfHeader, ElfProgram, ELF_MAGIC};
use crate::shared::x86_64::{inb, outb, insl};

// boot.rs
//
//   WeensyOS boot loader. Loads the kernel at address 0x40000 from
//   the first IDE hard disk.
//
//   A BOOT LOADER is a tiny program that loads an operating system into
//   memory. It has to be tiny because it can contain no more than 510 bytes
//   of instructions: it is stored in the disk's first 512-byte sector.
//
//   When the CPU boots it loads the BIOS into memory and executes it. The
//   BIOS intializes devices and CPU state, reads the first 512-byte sector of
//   the boot device (hard drive) into memory at address 0x7C00, and jumps to
//   that address.
//
//   The boot loader is contained in bootstart.S and boot.c. Control starts
//   in bootstart.S, which initializes the CPU and sets up a stack, then
//   transfers here. This code reads in the kernel image and calls the
//   kernel.
//
//   The main kernel is stored as an ELF executable image starting in the
//   disk's sector 1.

const SECTORSIZE: usize = 512;
const ELFHDR: *mut ElfHeader = 0x10000 as *mut ElfHeader;

// Panic
//    Required by cargo compiler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Print _info via vga_buffer if needed
    loop {}
}

// boot
//    Load the kernel and jump to it.
#[no_mangle]
pub unsafe extern "C" fn boot() -> ! {
    // read 1st page off disk (should include programs as well as header)
    // and check validity
    boot_readseg(ELFHDR as usize, 1, SECTORSIZE, SECTORSIZE);
    while (*ELFHDR).e_magic != ELF_MAGIC {
        /* do nothing */
    }

    // load each program segment
    let ph = (ELFHDR as usize + (*ELFHDR).e_phoff as usize) as *const ElfProgram;
    let eph = ph.add((*ELFHDR).e_phnum as usize);
    let mut ph_ptr = ph;
    while ph_ptr < eph {
        let ph_val = *ph_ptr;
        boot_readseg(
            ph_val.p_va as usize,
            ph_val.p_offset as u32 / SECTORSIZE as u32 + 1,
            ph_val.p_filesz as usize,
            ph_val.p_memsz as usize,
        );
        ph_ptr = ph_ptr.add(1);
    }

    // jump to the kernel
    let kernel_entry: fn() -> ! = transmute((*ELFHDR).e_entry);
    kernel_entry();
}

// boot_readseg(dst, src_sect, filesz, memsz)
//    Load an ELF segment at virtual address `dst` from the IDE disk's sector
//    `src_sect`. Copies `filesz` bytes into memory at `dst` from sectors
//    `src_sect` and up, then clears memory in the range
//    `[dst+filesz, dst+memsz)`.
fn boot_readseg(ptr: usize, src_sect: u32, filesz: usize, memsz: usize) {
    let end_ptr = ptr + filesz;
    let mut ptr = ptr & !(SECTORSIZE - 1);

    let mut src_sect = src_sect;
    while ptr < end_ptr {
        boot_readsect(ptr, src_sect);
        ptr += SECTORSIZE;
        src_sect += 1;
    }

    while ptr < memsz {
        unsafe {
            *(ptr as *mut u8) = 0;
        }
        ptr += 1;
    }
}

// boot_readsect(dst, src_sect)
//    Read disk sector number `src_sect` into address `dst`.
fn boot_readsect(dst: usize, src_sect: u32) {
    // Wait for the disk to be ready.
    boot_waitdisk();

    // Read disk sector number `src_sect` into address `dst`.
    outb(0x1F2, 1);
    outb(0x1F3, src_sect.try_into().unwrap());
    outb(0x1F4, (src_sect >> 8) as u8);
    outb(0x1F5, (src_sect >> 16) as u8);
    outb(0x1F6, ((src_sect >> 24) | 0xE0) as u8);
    outb(0x1F7, 0x20);

    // Then move the data into memory.
    boot_waitdisk();
    insl(0x1F0, dst as *mut u32, SECTORSIZE / 4);
}

// boot_waitdisk
//    Wait for the disk to be ready.
fn boot_waitdisk() {
    // Wait until the ATA status register says ready (0x40 is on)
    // & not busy (0x80 is off)
    while inb(0x1F7) & 0xC0 != 0x40 {
        /* do nothing */
    }
}
