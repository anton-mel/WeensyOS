use crate::vm::*;

// lookup_l1pagetable(pagetable, va, perm)
//    Helper function to find the last level of `va` in `pagetable`
//
//    Returns an x86_64_pagetable pointer to the last level pagetable
//    if it exists and can be accessed with the given permissions
//    Returns NULL otherwise

#[no_mangle]
#[allow(unused_variables)] // must be removed
pub unsafe extern "C" fn lookup_l1pagetable(
    pagetable: *mut x86_64_pagetable,
    va: usize,
    perm: i32,
) -> *mut x86_64_pagetable {
    let mut pt = pagetable;

    // We find the L1 pagetable by doing the following three steps for each level
    // 1. Find index to the next pagetable entry using the `va`
    // 2. Check if this entry has the appropriate requested permissions
    // 3. Repeat the steps till you reach the L1 pagetable (i.e., thrice)
    // 4. Return the pagetable address

    for i in 0..=2 {
        // TODO
        // find page entry by finding `ith` level index of va to index pagetable entries of `pt`
        // you should read x86-64.h to understand relevant structs and macros to make this part easier
        let pe: u64 = 0; // replace this

        if (pe & PTE_P as u64) == 0
        { // address of next level should be present AND PTE_P should be set, error otherwise
            // HERE NEED TO ADD THE LOGPRINTF
            if (perm & PTE_P as i32) == 0 {
                return core::ptr::null_mut();
            }
            // HERE NEED TO ADD THE LOGPRINTF
            return core::ptr::null_mut();
        }

        // sanity-check page entry and permissions
        assert!(pte_addr(pe) < MEMSIZE_PHYSICAL as u64); // at sensible address
        if (perm & PTE_W as i32) != 0 {
            assert!((pe & PTE_W as u64) != 0); // if requester wants PTE_W, entry must allow PTE_W
        }
        if (perm & PTE_U as i32) != 0 {
            assert!((pe & PTE_U as u64) != 0); // if requester wants PTE_U, entry must allow PTE_U
        }

        // TODO
        // set pt to physical address to next pagetable using `pe`
        pt = core::ptr::null_mut(); // replace this
    }

    pt
}

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
