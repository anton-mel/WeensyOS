#ifndef WEENSYOS_PROCESS_H
#define WEENSYOS_PROCESS_H
#include "x86-64.h"

#if WEENSYOS_KERNEL
#error "process.h should not be used by kernel code."
#endif

// process-linux.h
//
//    Support code for compiling WeensyOS processes to a Linux target.
//    This is done by wrapping Linux system calls in process-linux.c
//    and invoking them here, under the name of their WeensyOS equivalent.
//    See GNUmakefile for usage.

// SYSTEM CALLS

// getpid
//    Return current process ID.
extern pid_t shim_getpid(void);
static inline pid_t getpid(void)
{
    return shim_getpid();
}

// yield
//    Yield control of the CPU to the kernel. The kernel will pick another
//    process to run, if possible.
extern void shim_yield(void);
static inline void yield(void)
{
    shim_yield();
}

// sys_page_alloc(addr)
//    Allocate a page of memory at address `addr`. `Addr` must be page-aligned
//    (i.e., a multiple of PAGESIZE == 4096). Returns 0 on success and -1
//    on failure.
static inline int sys_page_alloc(void *addr)
{
    return 0;
}

// fork()
//    Fork the current process. On success, return the child's process ID to
//    the parent, and return 0 to the child. On failure, return -1.
extern pid_t shim_fork(void);
static inline pid_t fork(void)
{
    return shim_fork();
}

// exit()
//    Exit this process. Does not return.
extern void __attribute__((noreturn)) shim_exit(void);
static inline void __attribute__((noreturn)) sys_exit(void)
{
    shim_exit();
}

// panic(msg)
//    Panic.
extern void __attribute__((noreturn)) shim_panic(const char *msg);
static inline pid_t __attribute__((noreturn)) panic(const char *msg)
{
    shim_panic(msg);
}

// brk(addr)
//     change the location of the program break to addr
//     program break defines the end of the process's data segment
//     increasing the program break has the effect of allocating memory to the process
//     decreasing the break deallocates memory
//     on success, returns 0
//     on failure, return -1
//     brk cannot exceed MEMSIZE_VIRTUAL, and cannot be lower than data segment (loaded
//     by the loader)
extern int shim_brk(void *addr);
static inline int brk(void *addr)
{
    return shim_brk(addr);
}

// sbrk(increment)
//     increment the location of the program break by `increment` bytes
//     program break defines the end of the process's data segment
//     Calling sbrk() with an increment of 0 can be used to find the current location of the program break
//     On success, sbrk() returns the previous program break
//     (If the break was increased, then this value is a pointer to the start of the newly allocated memory)
//      On error, (void *) -1 is returned
extern void *shim_sbrk(const intptr_t increment);
static inline void *sbrk(const intptr_t increment)
{
    return shim_sbrk(increment);
}

// mem_tog
// toggles kernels printing of memory space for process if pid is its processID
// if pid == 0, toggles state globally (preference to global over local)
static inline void mem_tog(pid_t p) {}

// OTHER HELPER FUNCTIONS

// app_printf(format, ...)
//    Calls console_printf() (see lib.h). The cursor position is read from
//    `cursorpos`, a shared variable defined by the kernel, and written back
//    into that variable. The initial color is based on the current process ID.
extern void app_printf(int colorid, const char *format, ...);

extern void assert_fail(const char *file, int line, const char *msg);

extern void console_clear(void);

#endif
