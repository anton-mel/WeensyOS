# Getting Started with Rust WeensyOS

Before beginning work on the assignment (spec is placed inside the `starter-code` folder), follow these steps to configure your Rust compiler.

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
> If you are using a Docker file provided or working locally, you should disable the locking of `qemu` by manually modifying the `USE_HOST_LOCK=0` flag in GNUMakefile to 0 (line 50). This prevents two `qemu` processes from running on different Zoo nodes—a feature that is no longer needed. Otherwise, your environement will not be able to find required variables.

<p align="center">
  Congratulations! You're all set up to start working with WeensyOS in Rust!
</p>

## How to build

To start WeensyOS, run as usual `make run` or `make run-console`. You should see `PANIC: kernel/c/vm.c:45: assertion 'vmap.pa == addr' failed` error displayed.

## IMPORTANT: C vs Rust WeensyOS Differences

If you have already reviewed the project structure, you'll notice that the main difference with C-WeensyOS is that the `kern` folder contains both `c` and `rust` implementations. We have separated the bootloader and helper functions (written in C) from the core kernel functions required for this problem set—which are implemented in Rust. There are only 5 of them: `kernel`, `exception`, `process_setup`, `virtual_memory_map`, and `program_load_segment`. This is necessary to preserve the original assignment specification, ensuring a fair game for both of the groups. As a bonus point for the Rust folks, you don’t need to worry about what parts should be modified from this vast amount of code, since the five functions you need to implement have already been extracted for you.

As mentioned, this hybrid operating system is built using two languages, which can be particularly challenging because you must carefully manage their differences to link correctly. The easiest way to achieve this is by using FFI (Foreign Function Interface). We export functions from C to object files (in all `mod.rs` files), link them together in the `Makefile`, and then import them in Rust using an `extern "C"` or `extern "C-unwind"` block:

```rust
extern "C-unwind" {
  // This function is written in C.
  pub fn hardware_init();
}
```

You do not need to understand how this works under the hood! You should work only with the `kernel.rs`, `vm.rs`, and `kloader.rs` files. For example, in `kloader.rs`, there is only one function:

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

and it depends on this C-functions/variables imported:

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

You can safely assume that the C implementation has been verified and is memory safe. You can also assume that all functions needed to complete this problem set are already imported in the `mod.rs` files. Review what the C functions do to understand how to use them in order to complete the problem set. Otherwise, follow the general spec provided with the assignment.

## WeensyOS-Rust Support

We acknowledge that building an operating system in Rust can be challenging. Although only a few ULAs are available to assist, I (Anton) and George will provide support whenever possible via the channel provided below. Please note that we will not help with assignment-related questions outside of office hours, except for issues specifically related to WeensyOS-Rust (such as undefined behavior, errors, or other unexpected problems).
