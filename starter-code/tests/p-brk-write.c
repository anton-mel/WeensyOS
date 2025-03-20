#include "lib.h"
#include "malloc.h"


#define ALLOC_SLOWDOWN 100
#define MAX_ALLOC 100
extern uint8_t end[];

uint8_t * heap_top;
uint8_t * heap_bottom;
uint8_t * stack_bottom;


void process_main(void) {
    pid_t p = getpid();
    srand(p);
    heap_bottom  = ROUNDUP((uint8_t*) end, PAGESIZE);
    stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    heap_top = heap_bottom + 21*1024;
    assert(brk((void *)heap_top) == 0);

    for(ssize_t i = 0; i < (intptr_t)heap_top - (intptr_t)heap_bottom; ++i) {
        heap_bottom[i] = 'A';
        assert(heap_bottom[i] == 'A');
    }

    assert(brk((void *)heap_bottom) == 0);

    /* check that pages were deallocated */
    for(uintptr_t va = (uintptr_t)heap_bottom; va < (uintptr_t)heap_top; va += 4096) {
        vamapping map;
        mapping(va, &map);
	assert(!(map.perm & PTE_P));
    }

    TEST_PASS();
}
