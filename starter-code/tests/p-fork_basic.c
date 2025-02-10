#include "process.h"
#include "lib.h"
#define ALLOC_SLOWDOWN 100

extern uint8_t end[];

uint8_t* heap_top;
uint8_t* stack_bottom;

uint8_t padding_page[0x1000];
char const * preamble = 
"ANOTHER galaxy, another time."
"The Old Republic was the Republic of legend, greater than distance"
"or time.  No need to note where it was or whence it came, only to know"
"that ... it was THE Republic"
"Once, under the wise rule of the Senate and the protection of the"
"Jedi Knights, the Republic throve and grew.  But as often happens when"
"wealth and power pass beyond the admirable and attain the awesome, then"
"appear those evil ones who have greed to match."
"So it was with the Republic at its height.  Like the greatest of"
"trees, able to withstand any external attack, the Republic rotted from"
"within though the danger was not visible from outside."
"Aided and abetted by restless, power-hungry individuals within the"
"government, and the massive organs of commerce, the ambitious Senator"
"Palpatine caused himself to be elected President of the Republic.  He"
"promised to reunite the disaffected among the people and to restore the"
"remembered glory of the Republic."
"Once secure in office he declared himself Emperor, shutting"
"himself away from the populace.  Soon he was controlled by the very"
"assistants and bootlickers he had appointed to high office, and the"
"cries of the people for justice did not reach his ears."
"Having exterminated through treachery and deception the Jedi"
"Knights, guardians of justice in the galaxy, the Imperial governors and"
"bureaucrats prepared to institute a reign of terror among the"
"disheartened worlds of the galaxy.  Many used the imperial forces and"
"the name of the increasingly isolated Emperor to further their own"
"personal ambitions"
"But a small number of systems rebelled at these new outrages."
"Declaring themselves opposed to the New Order they began the great"
"battle to restore the Old Republic."
"From the beginning they were vastly outnumbered by the systems"
"held in thrall by the Emperor.  In those first dark days it seemed"
"certain the bright flame of resistance would be extinguished before it"
"could cast the light of new truth across a galaxy of oppressed and"
"beaten peoples ..."
"From the First Saga"
"Journal of the Whills"
"They were in the wrong place at the wrong time.  Naturally they"
"became heroes."
"Leia Organa of Aldernaan, Senator"
"--  George Lucas, Prologue to Star Wars";

// similar to p-fork but with large global memory 

void process_main(void) {
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

    // The rest of this code is like p-allocator.c.

    pid_t p = sys_getpid();
    srand(p);

    heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);
    stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    while (1) {
        if ((rand() % ALLOC_SLOWDOWN) < p) {
            if (heap_top == stack_bottom || sys_page_alloc(heap_top) < 0) {
                break;
            }
            *heap_top = p;      /* check we have write access to new page */
            heap_top += PAGESIZE;
            padding_page[0] = p1 + preamble[(uint8_t)p1];
        }
        sys_yield();
    }

    sys_yield();
    sys_yield();
    if(p1 != 0 && p2 != 0) // parent
        TEST_PASS();
    else { // child
        while(1) {
            sys_yield();
        }
    }
}
