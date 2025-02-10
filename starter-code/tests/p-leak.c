#include "process.h"
#include "lib.h"
#define ALLOC_SLOWDOWN 100

extern uint8_t end[];

// These global variables go on the data page.
uint8_t* heap_top;
uint8_t* stack_bottom;

// Parent: continuously forks/yields without exiting
// Child: continuously allocates in a row, then exits

void process_main(void) {

    pid_t parent = sys_getpid();
    app_printf(parent, "Parent pid is %d\n", parent);
    pid_t fork = sys_fork();
    assert(fork >= 0);

    srand(parent);
    if(fork != 0){
        // parent in a while loop which keeps randomly forking
        app_printf(parent, "%dp\n", parent);
        while(1){
            if(rand() % ALLOC_SLOWDOWN == parent){
                int fork_new = sys_fork();
                if(fork_new == 0){
                    goto child;
                }
                sys_yield();
            }
            else{
                sys_yield();
            }
        }
    }
    else
    {
child:;
        pid_t p = sys_getpid();
        srand(p);

        // The heap starts on the page right after the 'end' symbol,
        // whose address is the first address not allocated to process code
        // or data.
        heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);

        // The bottom of the stack is the first address on the current
        // stack page (this process never needs more than one stack page).
        stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

        // Allocate heap pages until (1) hit the stack (out of address space)
        // or (2) allocation fails (out of physical memory).
        while (1) {
            if ((rand() % ALLOC_SLOWDOWN) < p) {
                assert(sys_getpid() != parent);
                if (heap_top == stack_bottom || sys_page_alloc(heap_top) < 0) {
                    break;
                }
                *heap_top = p;      /* check we have write access to new page */
                heap_top += PAGESIZE;
            }
            sys_yield();
        }

        // After running out of memory, make an exit after 10 yields
        int i = 10;
        while(i--){
            sys_yield();
        }
        app_printf(p, "%d\n", p);
        sys_exit();

    }
}
