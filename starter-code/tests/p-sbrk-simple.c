#include "lib.h"
#include "malloc.h"


#define ALLOC_SLOWDOWN 100
#define MAX_ALLOC 100
extern uint8_t end[];

uint8_t* heap_top;
uint8_t* stack_bottom;



void process_main(void) {
    pid_t p = getpid();
    srand(p);
    heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);
    stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    uint8_t * heap = heap_top;

    intptr_t break1, break2;

    //Sanity
    break1 = (intptr_t)sbrk(0);
    assert(break1 == (intptr_t)heap_top);
    break2 = break1;

    //Increase, returns previous break
    break1 = (intptr_t)sbrk(10);
    assert(break1 == break2);
    break2 = break1;

    //Inrease again, returns previously increased break
    break1 = (intptr_t)sbrk(11);
    assert(break1 == break2 + 10);
    break2 = break1;

    //Decrease, returns twice increased break
    break1 = (intptr_t)sbrk(-5);
    assert(break1 == break2 + 11);
    break2 = break1;

    //Check safe decrease
    break1 = (intptr_t)sbrk(0);
    assert(break1 == break2 - 5);
    break2 = break1;

    TEST_PASS();
}
