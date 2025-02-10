#include "process.h"
#include "lib.h"

#define ALLOC_SLOWDOWN 100
#define MAX_ALLOC 100

#ifndef KERNEL_ADDR
#define KERNEL_ADDR 0x10000
#endif

#ifndef MEMSIZE_VIRTUAL
#define MEMSIZE_VIRTUAL 0x300000
#endif
extern uint8_t end[];

uint8_t* heap_top;
uint8_t* stack_bottom;

// Program that checks some of sys_page_alloc conditions

void process_main(void) {
    pid_t p = sys_getpid();
    srand(p);
    // The heap starts on the page right after the 'end' symbol,
    // whose address is the first address not allocated to process code
    // or data.
    heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);

    // Test for alignment
    int x = sys_page_alloc((void *) (end + 0x10));
    if(x != -1){
        panic("Error, sys_page_alloc doesn't check for alignment!");
    }
    // Test for accessing beyond size limits
    x = sys_page_alloc((void *) MEMSIZE_VIRTUAL + PAGESIZE);
    if(x != -1){
        panic("Error, sys_page_alloc doesn't check for VM bounds!");
    }

    TEST_PASS();
}
