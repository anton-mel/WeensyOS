use crate::vm::*;

// virtual_memory_map(pagetable, va, pa, sz, perm)
//    Map virtual address range `[va, va+sz)` in `pagetable`.
//    When `X >= 0 && X < sz`, the new pagetable will map virtual address
//    `va+X` to physical address `pa+X` with permissions `perm`.
//
//    Precondition: `va`, `pa`, and `sz` must be multiples of PAGESIZE
//    (4096).
//
//    Typically `perm` is a combination of `PTE_P` (the memory is Present),
//    `PTE_W` (the memory is Writable), and `PTE_U` (the memory may be
//    accessed by User applications). If `!(perm & PTE_P)`, `pa` is ignored.
//
//    Returns 0 if the map succeeds, -1 if it fails (because a required
//    page table was not allocated).

#[no_mangle]
pub unsafe fn virtual_memory_map(
    pagetable: *mut x86_64_pagetable, // Pointer to the page table
    va: usize,                        // Virtual address
    pa: usize,                        // Physical address
    sz: usize,                        // Size
    perm: i32,                        // Permissions
) -> i32 {    
    // sanity checks for virtual address, size, and permisions
    assert!(va % PAGESIZE as usize == 0);        // virtual address is page-aligned
    assert!(sz % PAGESIZE as usize == 0);        // size is a multiple of PAGESIZE
    assert!(va + sz >= va || va + sz == 0);      // va range does not wrap
    if perm & PTE_P as i32 != 0 {
        assert!(pa % PAGESIZE as usize == 0);            // physical addr is page-aligned
        assert!(pa + sz >= pa);                          // physical address range does not wrap
        assert!(pa + sz <= MEMSIZE_PHYSICAL as usize);   // physical addresses exist
    }
    assert!(perm >= 0 && perm < 0x1000);         // `perm` makes sense (perm can only be 12 bits)
    assert!((pagetable as usize) % PAGESIZE as usize == 0); // `pagetable` page-aligned

    let mut last_index123 = -1 as i32;
    
    #[allow(unused_mut)]
    let mut l1pagetable: *mut x86_64_pagetable = core::ptr::null_mut();

    #[allow(unused_variables)]
    let (mut va, mut pa, mut sz) = (va, pa, sz); // clone
    // for each page-aligned address, set the appropriate page entry
    while sz != 0 {
        let cur_index123 = (va >> (PAGEOFFBITS + PAGEINDEXBITS)) as i32;
        if cur_index123 != last_index123 {
            // TODO
            // find pointer to last level pagetable for current va


            
            last_index123 = cur_index123;
        }

        if perm & PTE_P as i32 != 0 && !l1pagetable.is_null() {
            // TODO
            // map `pa` at appropriate entry with permissions `perm`
        } else if !l1pagetable.is_null() {
            // TODO
            // map to address 0 with `perm`
        } else if perm & PTE_P as i32 != 0 {
            // error, no allocated l1 page found for va
            return -1;
        }

        va += PAGESIZE as usize;
        pa += PAGESIZE as usize;
        sz -= PAGESIZE as usize;
    }
    return 0;
}
