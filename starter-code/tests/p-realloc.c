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

    void *ptr = malloc(25);
    assert(ptr != NULL);

    memset(ptr, 'A', 25);

    ptr = realloc(ptr, 25000);
    assert(ptr != NULL);

    /* check that memory was copied */
    for(size_t i = 0; i < 25; ++i) {
        assert(((char *)ptr)[i] == 'A');
    }
    memset(ptr, 'A', 25000);


    void *ptr2 = malloc(1024);
    memset(ptr2, 'B', 1024);

    for(size_t i = 512; i > 0; --i) {
        ptr2 = realloc(ptr2, i);
	for(size_t j = 0; j < i; ++j) {
            assert(((char *)ptr2)[j] == 'B');
        }
    }

    ptr2 = realloc(ptr2, 0);
    ptr2 = realloc(NULL, 0);

    /* confirm no tampering */
    for(size_t i = 0; i < 25000; ++i) {
        assert(((char *)ptr)[i] == 'A');
    }

    free(ptr);

    TEST_PASS();
}
