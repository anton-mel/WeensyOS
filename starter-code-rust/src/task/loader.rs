/* use x86_64::structures::paging::{PageTableFlags, Mapper, Page, Size4KiB};
use x86_64::{VirtAddr, PhysAddr};
use crate::allocator::ALLOCATOR;
use alloc::vec::Vec;
use core::slice;
use core::ptr;

const ELF_MAGIC: u32 = 0x464C_457F; // "\x7FELF" in little-endian
const ELF_PTYPE_LOAD: u32 = 1;

#[repr(C)]
struct ElfHeader {
    magic: u32,
    _other: [u8; 12], // Unused fields
    ph_offset: u64,
    ph_entry_size: u16,
    ph_count: u16,
    entry: u64,
}

#[repr(C)]
struct ElfProgramHeader {
    ptype: u32,
    flags: u32,
    offset: u64,
    va: u64,
    pa: u64,
    filesz: u64,
    memsz: u64,
    align: u64,
}

// Define a Process structure (simplified)
struct Process {
    registers: Registers,
    mapper: dyn Mapper<Size4KiB>, // Add the correct generic type here
}

// Define a Registers structure (simplified)
struct Registers {
    rip: u64,
}

// Define a custom FrameAllocator trait
pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysAddr>;
} */

pub fn load_program(_program_number: &str) {
    
}

// Example usage
/* pub fn load_program(p: &mut Process, program_number: usize) -> Result<(), &'static str> {
    let mut allocator = ALLOCATOR.lock();
    let ram_images = get_ram_images();  // This should return a collection of your ELF binaries
    let nprograms = ram_images.len();

    if program_number >= nprograms {
        return Err("Invalid program number");
    }

    let elf_header = unsafe { &*(ram_images[program_number].begin as *const ElfHeader) };
    if elf_header.magic != ELF_MAGIC {
        return Err("Invalid ELF magic number");
    }

    for i in 0..elf_header.ph_count {
        let ph_offset = elf_header.ph_offset + (i as u64) * (elf_header.ph_entry_size as u64);
        let ph = unsafe { &*(ram_images[program_number].begin.offset(ph_offset as isize) as *const ElfProgramHeader) };
        if ph.ptype == ELF_PTYPE_LOAD {
            let pdata = unsafe { slice::from_raw_parts((ram_images[program_number].begin as usize + ph.offset as usize) as *const u8, ph.filesz as usize) };
            load_program_segment(p, ph, pdata, allocator)?;
        }
    }

    p.registers.rip = elf_header.entry;
    Ok(())
}

fn load_program_segment(p: &mut Process, ph: &ElfProgramHeader, src: &[u8], allocator: &mut dyn FrameAllocator) -> Result<(), &'static str> {
    let va = VirtAddr::new(ph.va);
    let end_file = va + ph.filesz;
    let end_mem = va + ph.memsz;

    for addr in (va.as_u64()..end_mem.as_u64()).step_by(4096) {
        let frame = allocator.allocate_frame().ok_or("Out of memory")?;
        let page = Page::containing_address(VirtAddr::new(addr));
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            p.mapper.map_to(page, frame, flags, allocator)?.flush();
        }
    }

    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), va.as_mut_ptr(), src.len());
        if ph.memsz > ph.filesz {
            let zero_start = va.as_u64() + ph.filesz;
            let zero_length = (ph.memsz - ph.filesz) as usize;
            ptr::write_bytes(zero_start as *mut u8, 0, zero_length);
        }
    }

    Ok(())
}

// Example function to get the RAM images (This is a placeholder)
fn get_ram_images() -> Vec<RamImage> {
    use alloc::vec;
    vec![]
}

// Example structure for RamImage
struct RamImage {
    begin: *const u8,
    end: *const u8,
}
 */