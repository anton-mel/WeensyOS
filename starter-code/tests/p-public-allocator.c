#include "lib.h"
#include "malloc.h"


#define ALLOC_SLOWDOWN 100
#define MAX_ALLOC 100
extern uint8_t end[];

uint8_t *heap_top;
uint8_t *heap_bottom;
uint8_t *stack_bottom;



void process_main(void) {
    pid_t p = getpid();
    srand(p);
    heap_bottom = heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);
    stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    while(heap_top < heap_bottom + 1024*1024) {

        void * ret = sbrk(PAGESIZE);
        assert(ret != (void *)-1);

        *heap_top = p;      /* check we have write access to new page */
        heap_top = (uint8_t *)ret + PAGESIZE;
    }

    TEST_PASS();
}
