#include "lib.h"
#include "malloc.h"

#define ALLOC_SLOWDOWN 100
#define MAX_ALLOC 100
extern uint8_t end[];

uint8_t *heap_top;
uint8_t *stack_bottom;

void process_main(void)
{
    pid_t p = getpid();
    srand(p);
    heap_top = ROUNDUP((uint8_t *)end, PAGESIZE);
    stack_bottom = ROUNDDOWN((uint8_t *)read_rsp() - 1, PAGESIZE);

    uint8_t *heap = heap_top;

    // Sanity
    assert(brk((void *)heap_top) == 0);
    assert(sbrk(0) == heap_top);

    // Small increase
    heap_top += 100;
    assert(brk((void *)heap_top) == 0);
    assert(sbrk(0) == heap_top);

    // Large increase
    heap_top += 4096;
    assert(brk((void *)heap_top) == 0);
    assert(sbrk(0) == heap_top);

    // Decrease
    heap_top -= 4096;
    assert(brk((void *)heap_top) == 0);
    assert(sbrk(0) == heap_top);

    TEST_PASS();
}
