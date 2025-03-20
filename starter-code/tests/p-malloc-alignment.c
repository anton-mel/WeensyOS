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

    void *ptr;

    /* Single elements on heap of varying sizes */
    for(int i = 0; i < 512; ++i) {
        ptr = malloc(i);
        assert(ptr != NULL || i == 0);
        assert((uintptr_t)ptr % 8 == 0);
        free(ptr);
    }

    /* Many things allocated at the same time */
    static char *ptrs[512];

    for(size_t i = 0; i < sizeof(ptrs)/sizeof(ptrs[0]); ++i) {
        ptrs[i] = (char *) malloc(i+1);
        assert(ptrs[i] != NULL);
        assert((uintptr_t)ptrs[i] % 8 == 0);
    }

    for(size_t i = 0; i < sizeof(ptrs)/sizeof(ptrs[0]); ++i) {
        free((void *)ptrs[i]);
    }

    /* Single elements on heap of varying sizes,
     * in reverse size order, leading to small splitting of free blocks. */
    for(size_t i = 4096; i > 0; --i) {
        ptr = malloc(i);
        assert(ptr != NULL);
        assert((uintptr_t)ptr % 8 == 0);

        /* Check that we can write */
        free(ptr);
    }

    ptr = malloc(25);
    assert(ptr != NULL);
    assert((uintptr_t)ptr % 8 == 0);

    ptr = realloc(ptr, 25000);
    assert(ptr != NULL);
    assert((uintptr_t)ptr % 8 == 0);

    free(ptr);

    ptr = calloc(10,10);
    assert(ptr != NULL);
    assert((uintptr_t)ptr % 8 == 0);

    TEST_PASS();
}
