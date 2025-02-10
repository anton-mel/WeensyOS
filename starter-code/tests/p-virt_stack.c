#include "process.h"
#include "lib.h"

#define N 30
#define MEMSIZE_VIRTUAL 0x300000
extern uint8_t end[];

void process_main(void) {
    pid_t p = sys_getpid();
    srand(p);

    // lets first check where stack is in VM
    uint8_t * stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    // we want it to be at the end of VM, i.e. 0x300000
    assert((uintptr_t)stack_bottom == (MEMSIZE_VIRTUAL - PAGESIZE));

    // Now, lets check how its allocated
    vamapping smap;
    sys_mapping((uintptr_t) stack_bottom, &smap);

    if(smap.pa == (uintptr_t)stack_bottom){
        // This case shouldn't take place now that we checked stack is at end
        // Consider ghostly interference
        panic("Error, stack is not allocated virtually");
    }

    // No need to check perm, otherwise nothing will work
    int i = p;
    assert(i == p);
    TEST_PASS();
}
