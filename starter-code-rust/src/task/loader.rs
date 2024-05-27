
/* use x86_64::structures::paging::{PageTable, PageTableFlags, Mapper};
use x86_64::{VirtAddr, PhysAddr};
use alloc::vec::Vec; */

pub fn load_program(_allocator: &str) -> Result<(), &'static str> {
    /* let nprograms = ram_images.len();
    assert!(program_number < nprograms);

    let elf_header = unsafe { &*(ram_images[program_number].begin as *const ElfHeader) };
    assert_eq!(elf_header.magic, ELF_MAGIC);

    for ph in &elf_header.program_headers {
        if ph.ptype == ELF_PTYPE_LOAD {
            let pdata = unsafe { core::slice::from_raw_parts((elf_header as *const _ as usize + ph.offset) as *const u8, ph.filesz as usize) };
            load_program_segment(p, ph, pdata, allocator)?;
        }
    }

    p.registers.rip = elf_header.entry;
    */
    Ok(())
}

/* fn load_program_segment(p: &mut Process, ph: &ElfProgramHeader, src: &[u8], allocator: &mut FrameAllocator) -> Result<(), &'static str> {
    let va = VirtAddr::new(ph.va as u64);
    let end_file = va + ph.filesz as u64;
    let end_mem = va + ph.memsz as u64;

    for addr in (va..end_mem).step_by(4096) {
        let frame = allocator.allocate_frame().ok_or("Out of memory")?;
        p.mapper.map_to(Page::containing_address(addr), frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, allocator)?;
    }

    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), va.as_mut_ptr(), src.len());
        core::ptr::write_bytes(end_file.as_mut_ptr(), 0, (end_mem - end_file) as usize);
    }

    Ok(())
}
 */