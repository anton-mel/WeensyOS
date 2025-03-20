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

    /* Single elements on heap of varying sizes */
    for(int i = 1; i < 512; ++i) {
        void *ptr = malloc(i);
        assert(ptr != NULL);

        /* Check that we can write */
        memset(ptr, 'A', i);
        free(ptr);
    }

    TEST_PASS();
}
