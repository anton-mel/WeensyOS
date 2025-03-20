#include "lib.h"
#include "malloc.h"


void process_main(void) {
    pid_t p = getpid();
    srand(p);

    for(int i = 0; i < 256; ++i) {
        void * ret = malloc(PAGESIZE);
        assert(ret != NULL);

        *((int*)ret) = p;       // check we have write access
    }

    TEST_PASS();
}
