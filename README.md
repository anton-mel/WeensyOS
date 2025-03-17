# Getting Started with Rust WeensyOS

Before beginning work on the assignment (spec is placed inside the `starter-code` folder), follow these 2 steps to configure your Rust compiler.

## Installing Rust and Cargo

If you haven't installed Rust and Cargo yet, run the following command to install them using the official installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Dependencies

When building baremetal, we need to setUp correct target architecture to explain `cargo` how to compile the project. In out case WeensyOS is a x86_64 operating system.

```bash
rustup target add x86_64-unknown-none
```

Certain features used in the project are unstable (and potentially memory-unsafe), so they must be explicitly enabled.

```bash
rustup default nightly
```

> [!IMPORTANT]
> Only for those who are using a Docker file provided or working locally on your Linux machine, you should disable the locking of `qemu` by manually modifying the `USE_HOST_LOCK=0` flag in GNUMakefile to 0 (line 50). Previously, it prevented two `qemu` processes from running on different Zoo nodes. Otherwise, your environement will not be able to find required variables.

<p align="center">
  Congratulations! You're all set up to start working with WeensyOS in Rust!
</p>

## How to build

To start WeensyOS, run as usual `make run` or `make run-console`. You should see `PANIC: kernel/c/vm.c:45: assertion 'vmap.pa == addr' failed` error displayed.

## C vs Rust WeensyOS Differences

If you have already reviewed the project structure, you'll notice that the main difference with C-WeensyOS is that the `kern` folder contains both `c` and `rust` implementations. We have separated the bootloader and helper functions (written in C) from the core kernel functions required for this problem set—which are implemented in Rust as a playground. There are only 6 of them: `kernel`, `exception`, `process_setup`, `virtual_memory_map`, `program_load_segment`, and `lookup_l1pagetable`. This is necessary to preserve the original assignment specification, ensuring a fair game for both of the groups. Implementing an OS in a fully memory-safe way would require a somewhat different design approach, making this pset significantly different. However, you will be able to experiment more with memory safety later on for the final project. If you are curious about how to do it the right way, please read this tutorial: https://os.phil-opp.com/.


## Is It Possible to Build an OS with Two Languages? (Optional)

This hybrid operating system is built using both Rust and C, which can be particularly challenging because you must carefully manage their differences to ensure proper linking and function calling. Ultimately, only one language can serve as the entry point for the bootloader, managing the core state machine and all other processes.  

In our case, this language is Rust. This choice allows us to properly handle panics and display error messages, which would be impossible in foreign languages due to the need for stack unwinding. You can see this in the `k-exception` file—specifically, in the `entry_from_boot` routine, which sets the stack pointer to the top of the OS kernel stack before jumping to the `kernel` routine. This routine, written in Rust, is exported as an object file (`.obj`) for linking.  

To ensure everything links correctly, we use the Foreign Function Interface (FFI). C functions are compiled into object files (referenced in various `mod.rs` files), linked together via the `Makefile`, and then imported into Rust using `extern "C"` or `extern "C-unwind"`.


```rust
extern "C-unwind" {
  // FFI Import Example.
  // This function is written in C.
  pub fn hardware_init();
}
```

> [!NOTE]
> There is no reason to modify the `mod.rs` files unless a specific design choice requires it. Overall, you should only work with the `kernel.rs`, `vm.rs`, and `kloader.rs` files.

For example, in `kloader.rs`, there is only one function:

```rust
// k-loader.c
//
//    Load a Weensy application into memory from a RAM image.

use crate::*;
use crate::kloader::*;

#[no_mangle]
pub unsafe fn program_load_segment(
    p: *mut Proc,
    ph: *const ElfProgram,
    src: *const u64,
) -> i32 {
  // removed for simplicity
  ...
}
```

and it depends on this C-functions/variables imported (`mod.rs`):

```rust
extern "C-unwind" {
    pub static kernel_pagetable: *mut x86_64_pagetable;
}

extern "C-unwind" {
    pub fn set_pagetable(pagetable: *mut x86_64_pagetable);
    pub fn assign_physical_page(addr: usize, owner: i8) -> core::ffi::c_int;
    pub fn roundup(a: usize, n: usize) -> usize;
    // removed for simplicity
    ...
}
```

You can safely assume that the C implementation has been verified for the correctness. You can also assume that all functions needed to complete this problem set are already imported in the `mod.rs` files. Review what the C functions do to understand how to use them in order to complete the problem set. Otherwise, follow the general spec provided with the assignment.

