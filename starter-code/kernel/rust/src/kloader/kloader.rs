// k-loader.c
//
//    Load a weensy application into memory from a RAM image.

use crate::*;
use crate::kloader::*;

// program_load_segment(p, ph, src, allocator)
//    Load an ELF segment at virtual address `ph->p_va` in process `p`. Copies
//    `[src, src + ph->p_filesz)` to `dst`, then clears
//    `[ph->p_va + ph->p_filesz, ph->p_va + ph->p_memsz)` to 0.
//    Calls `assign_physical_page` to allocate pages and `virtual_memory_map`
//    to map them in `p->p_pagetable`. Returns 0 on success and -1 on failure.

#[no_mangle]
pub unsafe fn program_load_segment(
    p: *mut Proc,
    ph: *const ElfProgram,
    src: *const u64
) -> i32 {
    let mut va: u64 = (*ph).p_va;
    let end_file: u64 = va + (*ph).p_filesz;
    let end_mem: u64 = va + (*ph).p_memsz;
    va &= !(PAGESIZE - 1); // round to page boundary
    
    // allocate memory
    let mut addr = va;
    while addr < end_mem {
        let pa = palloc((*p).p_pid);

        if pa.is_null() || virtual_memory_map(
            (*p).p_pagetable,
            addr as usize,
            pa as usize,
            PAGESIZE as usize,
            (PTE_W | PTE_P | PTE_U) as i32) < 0
        {
            // Unfortunately, rust does not support %d, %p etc.
            // Use this generate_msg helper function for convenience if required.
            let msg = generate_msg(
                "program_load_segment(pid ".as_ptr() as *const i8, 
                (*p).p_pid,
                "): can't assign address ".as_ptr() as *const i8, 
                addr
            );
            console_printf(
                cpos!(22, 0), 0xC000, msg as *const u8
            );
            return -1;
        }
        addr += PAGESIZE;
    }

    // ensure new memory mappings are active
    set_pagetable((*p).p_pagetable);
    
    // copy data from executable image into process memory
    (va as *mut u8).copy_from_nonoverlapping(src as *const u8, (end_file - va) as usize);
    core::ptr::write_bytes(end_file as *mut u8, 0, (end_mem - end_file) as usize);

    // restore the kernel pagetable
    set_pagetable(kernel_pagetable);

    if ((*ph).p_flags & ELF_PFLAG_WRITE) == 0 {
        let mut addr = va;
        while addr < end_mem {
            let mapping = virtual_memory_lookup((*p).p_pagetable, addr as usize);
    
            virtual_memory_map(
                (*p).p_pagetable, 
                addr as usize, 
                mapping.pa as usize, 
                PAGESIZE as usize,
                (PTE_P | PTE_U) as i32);
    
            addr += PAGESIZE;
        }
    }

    // TODO : Add code here
    0 // success
}
