#include "lib.h"
#include "malloc.h"
#include "time.h"

extern uint8_t end[];

uint8_t *heap_top;
uint8_t *heap_bottom;
uint8_t *stack_bottom;



void process_main(void) {
    pid_t p = getpid();
    srand(p);
    heap_bottom = heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);
    stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    // alloc int array of 10 elements
    int* array = (int *)malloc(sizeof(int) * 10);

    assert(array != NULL);
    assert((uintptr_t)array % 8 == 0);

    // set array elements
    for(int  i = 0 ; i < 10; i++){
	array[i] = i;
    }

    // realloc array to size 20
    array = (int*)realloc(array, sizeof(int) * 20);

    assert(array != NULL);
    assert((uintptr_t)array % 8 == 0);

    // check if contents are same
    for(int i = 0 ; i < 10 ; i++){
        assert(array[i] == i);
    }

    // alloc int array of size 30 using calloc
    int * array2 = (int *)calloc(30, sizeof(int));

    // assert array[i] == 0
    for(int i = 0 ; i < 30; i++){
        assert(array2[i] == 0);
    }

    heap_info_struct info;
    assert(heap_info(&info) == 0);

    // check if allocations are in sorted order
    for(int i = 1 ; i < info.num_allocs; i++){
        assert(info.size_array[i] < info.size_array[i-1]);
    }

    // free array, array2
    free(array);
    free(array2);

    uint64_t total_time = 0;
    int total_pages = 0;

    // allocate pages till no more memory
    for(int i = 0; i < 256; ++i) {
        uint64_t time = rdtsc();
        void * ptr = malloc(PAGESIZE);
        total_time += (rdtsc() - time);

        if(ptr == NULL) break;

        total_pages++;
        *((int *)ptr) = p; // check write access
    }


    TEST_PASS();
}
