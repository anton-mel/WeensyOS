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
    for(int i = 1; i < 64; ++i) {
        for(int j = 1; j < 64; ++j) {
            void *ptr = calloc(i,j);
            assert(ptr != NULL);

            for(int k = 0; k < i*j; ++k) {
                assert(((char *)ptr)[k] == 0);
            }

            free(ptr);
        }
	defrag();
    }

    TEST_PASS();
}
