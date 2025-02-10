#include "process.h"
#include "lib.h"

#define N 30
extern uint8_t end[];

// program that checks if pages on the heap are allocated without one-to-one
// va->pa mapping

void process_main(void) {
    pid_t p = sys_getpid();
    srand(p);

    vamapping pmap;
    uint8_t * heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);

    // test N times
    for(int i = 0 ; i < N ; i++){
        int x = sys_page_alloc(heap_top);
        if(x != 0)
            panic("Error, sys_page_alloc failed!");
        // lets make sure we write to the page and are able to read from it
        *heap_top = p;
        assert(*heap_top == p);
        sys_mapping((uintptr_t)heap_top, &pmap);

        if(pmap.pa == (uintptr_t)heap_top)
            panic("Error, sys page alloc not virtualized!");

        heap_top += PAGESIZE;
    }

    TEST_PASS();
}
