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
    heap_bottom = heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);
    stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    /* move the break forward by 21KB -> ~5 pages */
    assert(sbrk(1024*21) == heap_bottom);

    /* get the new break */
    heap_top = (uint8_t *)sbrk(0);

    /* force the pages to be allocated */
    for(size_t i = 0; i < (uintptr_t)(heap_top - heap_bottom); ++i) {
        heap_bottom[i] = 'A';
        assert(heap_bottom[i] == 'A');
    }

    /* Break unmodied after optimistic allocation, move it back 21KB. */
    assert(sbrk(-1024*21) == heap_top);

    /* check that pages were deallocated */
    for(uintptr_t va = (uintptr_t)heap_bottom; va < (uintptr_t)heap_top; va += 4096) {
        vamapping map;
        mapping(va, &map);
	assert(!(map.perm & PTE_P));
    }

    TEST_PASS();
}
