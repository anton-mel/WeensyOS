#include "kernel.h"	// DO NOT EDIT!!!
#include "lib.h"	// DO NOT EDIT!!!

// external declarations
extern void (*sys_int_handlers[])(void);
extern void default_int_handler(void);
extern void gpf_int_handler(void);
extern void pagefault_int_handler(void);
extern void timer_int_handler(void);

// NOTE
// Read x86-64.h for some useful functions and macros relevant here!

// virtual_memory_init
//    Initialize the virtual memory system, including an initial page table
//    `kernel_pagetable`.

static x86_64_pagetable kernel_pagetables[5];
x86_64_pagetable* kernel_pagetable;


void virtual_memory_init(void) {
    kernel_pagetable = &kernel_pagetables[0];
    memset(kernel_pagetables, 0, sizeof(kernel_pagetables));

    // connect the pagetable pages
    kernel_pagetables[0].entry[0] =
        (x86_64_pageentry_t) &kernel_pagetables[1] | PTE_P | PTE_W | PTE_U;
    kernel_pagetables[1].entry[0] =
        (x86_64_pageentry_t) &kernel_pagetables[2] | PTE_P | PTE_W | PTE_U;
    kernel_pagetables[2].entry[0] =
        (x86_64_pageentry_t) &kernel_pagetables[3] | PTE_P | PTE_W | PTE_U;
    kernel_pagetables[2].entry[1] =
        (x86_64_pageentry_t) &kernel_pagetables[4] | PTE_P | PTE_W | PTE_U;

    // identity map the page table
    virtual_memory_map(kernel_pagetable, (uintptr_t) 0, (uintptr_t) 0,
                       MEMSIZE_PHYSICAL, PTE_P | PTE_W | PTE_U);

    // check if kernel is identity mapped
    for(uintptr_t addr = 0 ; addr < MEMSIZE_PHYSICAL ; addr += PAGESIZE){
        vamapping vmap = virtual_memory_lookup(kernel_pagetable, addr);
        // this assert will probably fail initially!
        // have you implemented virtual_memory_map and lookup_l1pagetable ?
        assert(vmap.pa == addr);
        assert((vmap.perm & (PTE_P|PTE_W)) == (PTE_P|PTE_W));
    }

    // set pointer to this pagetable in the CR3 register
    // set_pagetable also does several checks for a valid pagetable
    set_pagetable(kernel_pagetable);
}


// set_pagetable
//    Change page directory. lcr3() is the hardware instruction;
//    set_pagetable() additionally checks that important kernel procedures are
//    mappable in `pagetable`, and calls panic() if they aren't.

void set_pagetable(x86_64_pagetable* pagetable) {
    assert(PAGEOFFSET(pagetable) == 0); // must be page aligned
    // check for kernel space being mapped in pagetable
    assert(virtual_memory_lookup(pagetable, (uintptr_t) default_int_handler).pa
           == (uintptr_t) default_int_handler);
    assert(virtual_memory_lookup(kernel_pagetable, (uintptr_t) pagetable).pa
           == (uintptr_t) pagetable);
    assert(virtual_memory_lookup(pagetable, (uintptr_t) kernel_pagetable).pa
           == (uintptr_t) kernel_pagetable);
    assert(virtual_memory_lookup(pagetable, (uintptr_t) virtual_memory_map).pa
           == (uintptr_t) virtual_memory_map);
    lcr3((uintptr_t) pagetable);
}

// lookup_l1pagetable(pagetable, va, perm)
//    Helper function to find the last level of `va` in `pagetable`
//
//    Returns an x86_64_pagetable pointer to the last level pagetable
//    if it exists and can be accessed with the given permissions
//    Returns NULL otherwise
static x86_64_pagetable* lookup_l1pagetable(x86_64_pagetable* pagetable,
                 uintptr_t va, int perm);

extern int virtual_memory_map(x86_64_pagetable* pagetable, uintptr_t va,
                       uintptr_t pa, size_t sz, int perm);

// lookup_l1pagetable(pagetable, va, perm)
//    Helper function to find the last level of `va` in `pagetable`
//
//    Returns an x86_64_pagetable pointer to the last level pagetable
//    if it exists and can be accessed with the given permissions
//    Returns NULL otherwise
static x86_64_pagetable* lookup_l1pagetable(x86_64_pagetable* pagetable,
                 uintptr_t va, int perm) {
    x86_64_pagetable* pt = pagetable;

    // we find the lq pagetable by doing the following three steps for each level
    // 1. Find index to the next pagetable entry using the `va`
    // 2. Check if this entry has the appropriate requested permissions
    // 3. Repeat the steps till you reach the l1 pagetable (i.e thrice)
    // 4. return the pagetable address

    for (int i = 0; i <= 2; ++i) {
        // TODO
        // find page entry by finding `ith` level index of va to index pagetable entries of `pt`
        // you should read x86-64.h to understand relevant structs and macros to make this part easier
        x86_64_pageentry_t pe = 0; // replace this

        if (!(pe & PTE_P)) { // address of next level should be present AND PTE_P should be set, error otherwise
            log_printf("[Kern Info] Error looking up l1pagetable: Pagetable address: 0x%x perm: 0x%x."
                    " Failed to get level (%d)\n",
                    PTE_ADDR(pe), PTE_FLAGS(pe), (i+2));
            if (!(perm & PTE_P)) {
                return NULL;
            }
            log_printf("[Kern Info] failed to find pagetable address at " __FILE__ ": %d\n", __LINE__);
            return NULL;
        }

        // sanity-check page entry and permissions
        assert(PTE_ADDR(pe) < MEMSIZE_PHYSICAL); // at sensible address
        if (perm & PTE_W) {       // if requester wants PTE_W,
            assert(pe & PTE_W);   //   entry must allow PTE_W
        }
        if (perm & PTE_U) {       // if requester wants PTE_U,
            assert(pe & PTE_U);   //   entry must allow PTE_U
        }

        // TODO
        // set pt to physical address to next pagetable using `pe`
        pt = 0; // replace this
    }
    return pt;
}


// virtual_memory_lookup(pagetable, va)
//    Returns information about the mapping of the virtual address `va` in
//    `pagetable`. The information is returned as a `vamapping` object.

vamapping virtual_memory_lookup(x86_64_pagetable* pagetable, uintptr_t va) {
    x86_64_pagetable* pt = pagetable;
    x86_64_pageentry_t pe = PTE_W | PTE_U | PTE_P;
    for (int i = 0; i <= 3 && (pe & PTE_P); ++i) {
        pe = pt->entry[PAGEINDEX(va, i)] & ~(pe & (PTE_W | PTE_U));
        pt = (x86_64_pagetable*) PTE_ADDR(pe);
    }
    vamapping vam = { -1, (uintptr_t) -1, 0 };
    if (pe & PTE_P) {
        vam.pn = PAGENUMBER(pe);
        vam.pa = PTE_ADDR(pe) + PAGEOFFSET(va);
        vam.perm = PTE_FLAGS(pe);
    }
    return vam;
}

