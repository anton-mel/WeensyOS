#include "x86-64.h"
#include "elf.h"
#include "lib.h"
#include "kernel.h"

// k-loader.c
//
//    Load a weensy application into memory from a RAM image.

#define SECTORSIZE 512

extern uint8_t _binary_obj_p_allocator_start[];
extern uint8_t _binary_obj_p_allocator_end[];
extern uint8_t _binary_obj_p_malloc_start[];
extern uint8_t _binary_obj_p_malloc_end[];
extern uint8_t _binary_obj_p_alloctests_start[];
extern uint8_t _binary_obj_p_alloctests_end[];
extern uint8_t _binary_obj_p_test_start[];
extern uint8_t _binary_obj_p_test_end[];

struct ramimage
{
    void *begin;
    void *end;
} ramimages[] = {
    {_binary_obj_p_allocator_start, _binary_obj_p_allocator_end},
    {_binary_obj_p_malloc_start, _binary_obj_p_malloc_end},
    {_binary_obj_p_alloctests_start, _binary_obj_p_alloctests_end},
    {_binary_obj_p_test_start, _binary_obj_p_test_end}};

extern int program_load_segment(proc *p, const elf_program *ph,
                                const uint8_t *src,
                                x86_64_pagetable *(*allocator)(void));

// program_load(p, programnumber)
//    Load the code corresponding to program `programnumber` into the process
//    `p` and set `p->p_registers.reg_rip` to its entry point. Calls
//    `assign_physical_page` to as required. Returns 0 on success and
//    -1 on failure (e.g. out-of-memory). `allocator` is passed to
//    `virtual_memory_map`.

int program_load(proc *p, int programnumber,
                 x86_64_pagetable *(*allocator)(void))
{
    // is this a valid program?
    int nprograms = sizeof(ramimages) / sizeof(ramimages[0]);
    assert(programnumber >= 0 && programnumber < nprograms);
    elf_header *eh = (elf_header *)ramimages[programnumber].begin;
    assert(eh->e_magic == ELF_MAGIC);

    // load each loadable program segment into memory
    elf_program *ph = (elf_program *)((const uint8_t *)eh + eh->e_phoff);
    for (int i = 0; i < eh->e_phnum; ++i)
    {
        if (ph[i].p_type == ELF_PTYPE_LOAD)
        {
            const uint8_t *pdata = (const uint8_t *)eh + ph[i].p_offset;
            if (program_load_segment(p, &ph[i], pdata, allocator) < 0)
            {
                return -1;
            }
        }
    }

    // set the entry point from the ELF header
    p->p_registers.reg_rip = eh->e_entry;
    return 0;
}
