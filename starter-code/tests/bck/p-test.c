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

    void *ptr = malloc(7);
    assert(ptr != NULL);
    heap_top = sbrk(0);
    free(ptr);

    /* free block reuse */
    for(int i = 1; i < 100000; ++i) {
        ptr = malloc(7);
        assert(ptr != NULL);

        /* Check that we reuse a free block */
        assert(sbrk(0) == heap_top);

        /* Check that we can write */
        memset(ptr, 'A', 7);
        free(ptr);
    }

    TEST_PASS();
}
