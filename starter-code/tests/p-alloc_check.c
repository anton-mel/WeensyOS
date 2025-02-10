#include "process.h"
#include "lib.h"
#define ALLOC_SLOWDOWN 100

extern uint8_t end[];

// These global variables go on the data page.
uint8_t* heap_top;
uint8_t* stack_bottom;

#define CANARY 0xDEADBEEF
#define MULT   0x000B0000

// write to entire page with `value`
void write_page(uint8_t *addr, uint32_t value){
    uint32_t * int_addr = (uint32_t *) addr;
    for(unsigned long i = 0 ; i < PAGESIZE/sizeof(uint32_t) ; i++){
        int_addr[i] = value;
    }
}

// check if enter page contains `value`
void assert_page(uint8_t * addr, uint32_t value){
    uint32_t * int_addr = (uint32_t *) addr;
    for(unsigned long i = 0 ; i < PAGESIZE/sizeof(uint32_t) ; i++){
        assert(int_addr[i] == value && "Error: page was corrupted!");
    }
}

// Behaves similar to p-allocator.c, except it writes to the entire page
// and checks if the memory was untouched

void process_main(void) {
    pid_t p = sys_getpid();
    srand(p);

    // The heap starts on the page right after the 'end' symbol,
    // whose address is the first address not allocated to process code
    // or data.
    heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);
    uint8_t * heap_end = heap_top;

    // The bottom of the stack is the first address on the current
    // stack page (this process never needs more than one stack page).
    stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    // Allocate heap pages until (1) hit the stack (out of address space)
    // or (2) allocation fails (out of physical memory).
    while (1) {
        if ((rand() % ALLOC_SLOWDOWN) < p) {
            if (heap_top == stack_bottom || sys_page_alloc(heap_top) < 0) {
                break;
            }
            // write with random canary and process ID based unique value to check later
            write_page(heap_top, CANARY + p * MULT); 
            heap_top += PAGESIZE;
        }
        sys_yield();
    }
    //check all addresses so far
    while(heap_end < heap_top){
        // for all alloc'd pages, check if page still contains same value
        assert_page(heap_end, CANARY + p * MULT);
        heap_end += PAGESIZE;
    }

    // After running out of memory, do nothing forever, done here
    while (1) {
        sys_yield();
    }
}
