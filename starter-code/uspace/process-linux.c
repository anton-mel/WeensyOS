#include <unistd.h>
#include <stdlib.h>
#include <sched.h>
#include <stdio.h>
#include <stdarg.h>
#include <string.h>

extern int process_main(void);

pid_t shim_getpid(void)
{
    return getpid();
}

void shim_yield(void)
{
    sched_yield();
}

pid_t shim_fork(void)
{
    return fork();
}

void __attribute__((noreturn)) shim_exit(void)
{
    exit(0);
}

void __attribute__((noreturn)) shim_panic(const char *msg)
{
    perror(msg);
    exit(1);
}

int shim_brk(void *addr)
{
    return brk(addr);
}

void *shim_sbrk(const intptr_t inc)
{
    return sbrk(inc);
}

void app_printf(int colorid, const char *format, ...)
{
    va_list args;
    va_start(args, format);
    vprintf(format, args);
    fflush(stdout);
    va_end(args);
}

void assert_fail(const char *file, int line, const char *msg)
{
    fprintf(stderr, "%s:%i: %s", file, line, msg);
    exit(1);
}

void console_clear(void) {}

int main(void)
{
    return process_main();
}
