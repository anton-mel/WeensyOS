#include "process.h"
#include "lib.h"

extern uint8_t end[];

uint8_t* heap_top;
uint8_t* stack_bottom;

// checks if multiple processes can allocate at the same Virtual Memory Address
// (run at least two instances)

void process_main(void) {
    pid_t p = sys_getpid();
    srand(p);
    // The heap starts on the page right after the 'end' symbol,
    // whose address is the first address not allocated to process code
    // or data.
    heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);

    app_printf(p, "Make sure you are running this test with at least 2 processes running by hitting 2!\n");

    // check if process can access each others memory

    int x = sys_page_alloc((void *) (heap_top));

    if(x != 0)
        panic("Error, couldn't allocate same memory location!\n");

    // yield to make sure other process also runs before continuing
    sys_yield();

    // write to allocd page
    *heap_top = p;

    // again, yield to allow other process to make progress before continuing
    // perhaps redundant

    sys_yield();

    // Now, test at least 100 times to see if values will ever change
    for(int i = 0 ; i < 100 ; i++){
        if(*heap_top != p)
            panic("Error, value changed! process memory not isolated!\n");
        sys_yield();
    }

    TEST_PASS();
}
