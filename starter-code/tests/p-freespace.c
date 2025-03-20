#include "lib.h"
#include "malloc.h"


extern uint8_t end[];

uint8_t *heap_top;
uint8_t *heap_bottom;
uint8_t *stack_bottom;

void process_main(void) {
    pid_t p = getpid();
    srand(p);
    heap_bottom = heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);
    stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    malloc(10);
    void* ptr2 = malloc(200);


    heap_info_struct h1, h2, h3;
    heap_info(&h1);


    void* ptr = malloc(16384);
    malloc(10);
    malloc(10);

    heap_info(&h2);

    free(ptr);
    free(ptr2);  // free ptr2 after ptr to allow explicit freelists with first fit strategies

    heap_info(&h3);

    assert(h1.size_array != NULL);
    assert(h2.size_array != NULL);
    assert(h1.ptr_array != NULL);
    assert(h2.ptr_array != NULL);


    assert(h3.free_space > h2.free_space);
    assert(h3.largest_free_chunk >= 16384);


    free(h1.size_array);
    free(h2.size_array);

    free(h1.ptr_array);
    free(h2.ptr_array);

    app_printf(0, "HEAP FREE SPACE PASS\n");
    TEST_PASS();

    // After running out of memory, do nothing forever
    while (1) {
        yield();
    }
}
