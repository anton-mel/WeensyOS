#include "lib.h"
#include "malloc.h"
#include "time.h"

#define ALLOC(ind) sizes[ind%4] * ((ind >> 2) % 4 + 1) 

extern uint8_t end[];

uint8_t *heap_top;
uint8_t *heap_bottom;
uint8_t *stack_bottom;

typedef struct ptr_with_size{
    void * ptr;
    long size;
} ptr_with_size;


/*Stolen from qsort.c*/
#define SWAP(a, b, size)                                                      \
  do                                                                              \
    {                                                                              \
      size_t __size = (size);                                                      \
      char *__a = (a), *__b = (b);                                              \
      do                                                                      \
        {                                                                      \
          char __tmp = *__a;                                                      \
          *__a++ = *__b;                                                      \
          *__b++ = __tmp;                                                      \
        } while (--__size > 0);                                                      \
    } while (0)
/* Discontinue quicksort algorithm when partition gets below this size.
   This particular magic number was chosen to work best on a Sun 4/260. */
#define MAX_THRESH 4
/* Stack node declarations used to store unfulfilled partition obligations. */
typedef struct
  {
    char *lo;
    char *hi;
  } stack_node;
#  define CHAR_BIT        8
#define STACK_SIZE        (CHAR_BIT * sizeof (size_t))
#define PUSH(low, high)        ((void) ((top->lo = (low)), (top->hi = (high)), ++top))
#define        POP(low, high)        ((void) (--top, (low = top->lo), (high = top->hi)))
#define        STACK_NOT_EMPTY        (stack < top)
typedef int (*__compar_fn_t) (const void *, const void *);

void
_quicksort (void *const pbase, size_t total_elems, size_t size,
            __compar_fn_t cmp)
{
    char *base_ptr = (char *) pbase;
    const size_t max_thresh = MAX_THRESH * size;
    if (total_elems == 0)
	/* Avoid lossage with unsigned arithmetic below.  */
	return;
    if (total_elems > MAX_THRESH)
    {
	char *lo = base_ptr;
	char *hi = &lo[size * (total_elems - 1)];
	stack_node stack[STACK_SIZE];
	stack_node *top = stack;
	PUSH (NULL, NULL);
	while (STACK_NOT_EMPTY)
	{
	    char *left_ptr;
	    char *right_ptr;
	    /* Select median value from among LO, MID, and HI. Rearrange
	       LO and HI so the three values are sorted. This lowers the
	       probability of picking a pathological pivot value and
	       skips a comparison for both the LEFT_PTR and RIGHT_PTR in
	       the while loops. */
	    char *mid = lo + size * ((hi - lo) / size >> 1);
	    if ((*cmp) ((void *) mid, (void *) lo) < 0)
		SWAP (mid, lo, size);
	    if ((*cmp) ((void *) hi, (void *) mid) < 0)
		SWAP (mid, hi, size);
	    else
		goto jump_over;
	    if ((*cmp) ((void *) mid, (void *) lo) < 0)
		SWAP (mid, lo, size);
jump_over:;
	  left_ptr  = lo + size;
	  right_ptr = hi - size;
	  /* Here's the famous ``collapse the walls'' section of quicksort.
	     Gotta like those tight inner loops!  They are the main reason
	     that this algorithm runs much faster than others. */
	  do
	  {
	      while ((*cmp) ((void *) left_ptr, (void *) mid) < 0)
		  left_ptr += size;
	      while ((*cmp) ((void *) mid, (void *) right_ptr) < 0)
		  right_ptr -= size;
	      if (left_ptr < right_ptr)
	      {
		  SWAP (left_ptr, right_ptr, size);
		  if (mid == left_ptr)
		      mid = right_ptr;
		  else if (mid == right_ptr)
		      mid = left_ptr;
		  left_ptr += size;
		  right_ptr -= size;
	      }
	      else if (left_ptr == right_ptr)
	      {
		  left_ptr += size;
		  right_ptr -= size;
		  break;
	      }
	  }
	  while (left_ptr <= right_ptr);
	  /* Set up pointers for next iteration.  First determine whether
	     left and right partitions are below the threshold size.  If so,
	     ignore one or both.  Otherwise, push the larger partition's
	     bounds on the stack and continue sorting the smaller one. */
	  if ((size_t) (right_ptr - lo) <= max_thresh)
	  {
	      if ((size_t) (hi - left_ptr) <= max_thresh)
		  /* Ignore both small partitions. */
		  POP (lo, hi);
	      else
		  /* Ignore small left partition. */
		  lo = left_ptr;
	  }
	  else if ((size_t) (hi - left_ptr) <= max_thresh)
	      /* Ignore small right partition. */
	      hi = right_ptr;
	  else if ((right_ptr - lo) > (hi - left_ptr))
	  {
	      /* Push larger left partition indices. */
	      PUSH (lo, right_ptr);
	      lo = left_ptr;
	  }
	  else
	  {
	      /* Push larger right partition indices. */
	      PUSH (left_ptr, hi);
	      hi = right_ptr;
	  }
	}
    }
    /* Once the BASE_PTR array is partially sorted by quicksort the rest
       is completely sorted using insertion sort, since this is efficient
       for partitions below MAX_THRESH size. BASE_PTR points to the beginning
       of the array to sort, and END_PTR points at the very last element in
       the array (*not* one beyond it!). */
#define min(x, y) ((x) < (y) ? (x) : (y))
    {
	char *const end_ptr = &base_ptr[size * (total_elems - 1)];
	char *tmp_ptr = base_ptr;
	char *thresh = min(end_ptr, base_ptr + max_thresh);
	char *run_ptr;
	/* Find smallest element in first threshold and place it at the
	   array's beginning.  This is the smallest array element,
	   and the operation speeds up insertion sort's inner loop. */
	for (run_ptr = tmp_ptr + size; run_ptr <= thresh; run_ptr += size)
	    if ((*cmp) ((void *) run_ptr, (void *) tmp_ptr) < 0)
		tmp_ptr = run_ptr;
	if (tmp_ptr != base_ptr)
	    SWAP (tmp_ptr, base_ptr, size);
	/* Insertion sort, running from left-hand-side up to right-hand-side.  */
	run_ptr = base_ptr + size;
	while ((run_ptr += size) <= end_ptr)
	{
	    tmp_ptr = run_ptr - size;
	    while ((*cmp) ((void *) run_ptr, (void *) tmp_ptr) < 0)
		tmp_ptr -= size;
	    tmp_ptr += size;
	    if (tmp_ptr != run_ptr)
	    {
		char *trav;
		trav = run_ptr + size;
		while (--trav >= run_ptr)
		{
		    char c = *trav;
		    char *hi, *lo;
		    for (hi = lo = trav; (lo -= size) >= tmp_ptr; hi = lo)
			*hi = *lo;
		    *hi = c;
		}
	    }
	}
    }
}

int ptr_comparator( const void * a, const void * b){
    ptr_with_size * a_ptr = (ptr_with_size *) a;
    ptr_with_size * b_ptr = (ptr_with_size *) b;

    return (int)b_ptr->size - (int)a_ptr->size;
}


int exists_in_between(ptr_with_size *ptrs, void * ptr, long size, int len){

    for(int i = 0; i < len ; i++){
        if(ptrs[i].ptr == ptr){
            if(ptrs[i].size <= size)
                return 1;
            if(ptrs[i].size != size){
                app_printf(0, "size (%ld, %ld) [%d, %d] ", size, ptrs[i].size, i, len - 1);
                return 0;
            }
        }
    }

    return 0;
}
void process_main(void) {
    pid_t p = getpid();
    srand(p);
    heap_bottom = heap_top = ROUNDUP((uint8_t*) end, PAGESIZE);
    stack_bottom = ROUNDDOWN((uint8_t*) read_rsp() - 1, PAGESIZE);

    int sizes[] = {10, 20, 30, 1999};

    // leave about 1 MB for malloc
    sbrk(123 * PAGESIZE);
    heap_top = sbrk(0);

    ptr_with_size *ptr = (ptr_with_size *)heap_bottom;
    // shift brk so we are 1 MB before stack
    brk((void *)(intptr_t)ROUNDDOWN(0x200000-1, PAGESIZE));


    volatile int ptr_size = 0;
    mem_tog(0);

    while((intptr_t)ROUNDUP(sbrk(0), PAGESIZE) <= 0x280000){
	int sz = ALLOC(ptr_size);
	void * temp_ptr = malloc(sz);
	if(temp_ptr == NULL)
	    break;
	ptr[ptr_size].ptr = temp_ptr;
	ptr[ptr_size].size = sz;
	ptr_size++;
    }

    _quicksort(ptr, ptr_size, sizeof(ptr[0]), &ptr_comparator);

    heap_info_struct h;
    register uint64_t time1 = rdtsc(); 
    int ret = heap_info(&h);
    time1 = rdtsc() - time1;
    app_printf(0, "time: %ld\n", time1);

    if(ret){
	exit();
    }

    assert(h.num_allocs == ptr_size);

    for(volatile int i = 0 ; i < h.num_allocs ; i++){
        assert(i == 0  || h.size_array[i] <= h.size_array[i-1]);
        int r = exists_in_between(ptr, h.ptr_array[i], h.size_array[i], ptr_size);
        assert(r);
    }

    app_printf(0, "HEAP INFO PASS\n");
    TEST_PASS();

    // After running out of memory, do nothing forever
    while (1) {
        yield();
    }
}
