# WeesyOS in Rust

WeensyOS is a tiny kernel that can run on bare-metal x86-64 machines (QEMU's emulated CPUs). The initial state of the kernel contains code for bootstrapping kernel, handling exceptions/syscalls, executing user-level program, and helper functions for your CPSC 323 exercises.

This project comprises two branches: one for `rust-safe` (semisafe) code, serving as an example of a potentially correct OS implementation in Rust that aims to minimize the use of unsafe code, and another for `unsafe` code closely following C conventions, essential for its similarity to the WeensyOS pset.

**Please switch to the respective branch to access the source code.**
