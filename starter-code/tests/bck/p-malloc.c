#include "lib.h"
#include "malloc.h"

#define ALLOC_SLOWDOWN 100
#define MAX_ALLOC 100

extern uint8_t end[];

uint8_t* heap_top;
uint8_t* stack_bottom;

void process_main(void) {
    pid_t p = getpid();

    heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);

    // The bottom of the stack is the first address on the current
    // stack page (this process never needs more than one stack page).
    stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    // Allocate heap pages until (1) hit the stack (out of address space)
    // or (2) allocation fails (out of physical memory).
    while (1) {
	if ((rand() % ALLOC_SLOWDOWN) < p) {
	    void * ret = malloc(PAGESIZE);
	    if(ret == NULL)
		break;
	    *((int*)ret) = p;       // check we have write access
	}
	yield();
    }
    // After running out of memory, do nothing forever
    while (1) {
	yield();
    }
}
