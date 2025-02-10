#include "process.h"
#include "lib.h"

extern uint8_t end[];

uint8_t* heap_top;
uint8_t* stack_bottom;

// Program that checks if pages are shared between parent and child
// It also checks if code pages are allocated one-to-one va->pa or not

void process_main(void) {

    uint8_t * code_page = ROUNDDOWN(end - PAGESIZE - 1, PAGESIZE);
    vamapping map;
    sys_mapping((uintptr_t)code_page, &map);

    // Fork a total of three new copies.

    pid_t p1 = sys_fork();
    assert(p1 >= 0);
    pid_t p2 = sys_fork();
    assert(p2 >= 0);

    // Check fork return values: fork should return 0 to child.
    if (sys_getpid() == 1) {
        assert(p1 != 0 && p2 != 0 && p1 != p2);
    } else {
        assert(p1 == 0 || p2 == 0);
    }

    // Now, lets check code page mapping
    vamapping child_cmap;
    sys_mapping((uintptr_t) code_page, &child_cmap);

    if(child_cmap.pa != map.pa){
        panic("Error, code pages not shared!");
    }

    sys_yield();
    sys_yield();

    if(child_cmap.pa == (uintptr_t)code_page || map.pa == (uintptr_t)code_page)
        panic("Error, code pages are not virtually mapped!");

    sys_yield();
    TEST_PASS();
}
