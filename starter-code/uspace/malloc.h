#ifndef MALLOC_H
#define MALLOC_H
#include "lib.h" /*for uint64_t*/
#if WEENSYOS_KERNEL
#error "malloc.h should not be used by kernel code."
#endif
#if defined(USE_LINUX)
#include "process-linux.h"
#else
#include "process.h"
#endif
// malloc(sz):
// allocates sz bytes of uninitialized memory and returns a pointer to the allocated memory
// if sz == 0, then malloc() either returns NULL or a unique pointer value that can be
// successfully passed to a later free
// the pointer should be aligned to 8 bytes
void *malloc(uint64_t sz);

// calloc(num, sz):
// allocates memory of an array of num elements of size sz bytes each and returns a pointer
// to the allocated array. The memory is set to 0. if num or sz is equal to 0, then calloc
// returns NULL or a unique pointer value that can be successfully passed to a later free
// calloc also checks for size overflow caused by num*sz
// returns NULL on failure
void *calloc(uint64_t num, uint64_t sz);

// free(ptr)
// the free funtion frees the memory space pointed to by ptr, which must have been returned
// by a previous call to malloc or realloc, or if free has already been called before, then
// undefined behavior occurs
// if ptr == NULL, then no operation happens
void free(void *ptr);

// realloc(ptr, sz)
// realloc changes the size of the memory block pointed to by ptr to size bytes.
// the contents will be unchanged in the range from the start of the region up to the
// minimum of the old and new sizes
// if the new size is larger than the old size, the added memory will not be initialized
// if ptr is NULL, then the call is equivalent to malloc(size) for all values of size
// if size is equal to zero, and ptr is not NULL, then the call is equivalent to free(ptr)
// unless ptr is NULL, it must have been returned by an earlier call to malloc(), or realloc().
// if the area pointed to was moved, a free(ptr) is done.
void *realloc(void *ptr, uint64_t sz);

// defrag()
//
void defrag();

// heap_info_struct
// will be used by the heap_info function to store relevant data
// num_allocs: store current number of allocations
// ptr_array: pointer to an array of pointers of each allocation.
//			each pointer should be a currently alloc'd pointer
//			size of array should be equal to num_allocs
//			list should be sorted by size of allocation
// size_array: pointer to an array of size of each allocation
//		size_array[i] should hold the size of allocation for ptr_array[i]
//		should be sorted in descending order
// free_space: size of free space
// largest_free_chunk: size of the largest free chunk
typedef struct heap_info_struct
{
    int num_allocs;
    long *size_array;
    void **ptr_array;
    int free_space;
    int largest_free_chunk;
} heap_info_struct;

// heap_info(info)
// set the appropriate values in the heap_info_struct passed
// the malloc library will be responsible for alloc'ing size_array and
// ptr_array
// the user, i.e. the process will be responsible for freeing these allocations
// note that the allocations used by the heap_info_struct will count as metadata
// and should NOT be included in the heap info
// return 0 for a successfull call
// if for any reason the information cannot be saved, return -1
int heap_info(heap_info_struct *info);

#endif
