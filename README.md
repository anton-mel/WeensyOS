# WeesyOS in Rust

WeensyOS is a tiny kernel that can run on bare-metal x86-64 machines (QEMU's emulated CPUs). The initial state of the kernel contains code for bootstrapping kernel, handling exceptions/syscalls, executing user-level program, and helper functions for your CPSC 323 exercises.

> Host CPU-arch: x86_64, Operating System: Linux (Fedora), Application binary interface: GNU
 
By compiling for our host triple, the Rust compiler and the linker assume that there is an underlying operating system such as Linux (Frog Node) that uses the C runtime by default, which causes the linker errors. So, to avoid the linker errors, we compile for our own environment ```x86_64-weensyos.json``` with no underlying operating system *[look how to build]*.

> [!NOTE]
> This branch is currently closed and undergoing restructuring to more closely follow the assignment's purpose. You can still boot it. For any issues, please reach out via <a href="mailto:anton.melnychuk@yale.edu">anton.melnychuk@yale.edu</a>.

# Source Code

This project comprises two branches: one for `rust-safe` (semisafe) code, serving as an example of a potentially correct OS implementation in Rust that aims to minimize the use of unsafe code, and another for `unsafe` code closely following C conventions, essential for its similarity to the WeensyOS pset.

# Source Inspirations & How to contribute

Please, consider reading through these documentations is you plan to contribute:

```
    1. https://os.phil-opp.com/
    2. https://www.theseus-os.com/
    3. https://zoo.cs.yale.edu/classes/cs323/323/proj5/starter-code/
```


