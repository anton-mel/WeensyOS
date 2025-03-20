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


    heap_info_struct h1, h2, h3;

    void* ptr = malloc(16384);
    void * ptr2 = malloc(10);
    void * ptr3 = malloc(10);

    heap_info(&h1);

    free(ptr);
    free(ptr2);
    free(ptr3);

    free(h1.size_array);
    free(h1.ptr_array);
    
    heap_info(&h2);

    free(h2.size_array);
    free(h2.ptr_array);

    defrag();
    heap_info(&h3);

    assert(h3.largest_free_chunk > h2.largest_free_chunk);

    app_printf(0, "DEFRAG PASS\n");
    TEST_PASS();

    // After running out of memory, do nothing forever
    while (1) {
        yield();
    }
}
