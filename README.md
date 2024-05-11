# WeesyOS in Rust

Project is currently under development. For any questions, please reach out via anton.melnychuk@yale.edu.

WeensyOS is a tiny kernel that can run on bare-metal x86-64 machines (for project 4/5 @ Yale, QEMU's emulated CPUs), developed by Eddie Kohler. The initial state of the kernel contains code for bootstrapping kernel, handling exceptions/syscalls, executing user-level program, and helper functions for your CPSC 323 exercises.

This OS is written in Rust @ Frog (Yale) `ssh yournetid@frog.zoo.cs.yale.edu`:

<p align="center">
    CPU-arch: x86_64, Operating System: Linux (Fedora), Application binary interface: GNU
</p>
 
By compiling for our host triple, the Rust compiler and the linker assume that there is an underlying operating system such as Linux (Frog Node) that uses the C runtime by default, which causes the linker errors. So, to avoid the linker errors, we compile for our own environment ```x86_64-weensyos.json``` with no underlying operating system [look how to build].


# How to build

<details>
<summary>Execute only once</summary>

1. Get to the root: ```cd ./starter-code-rust```
2. Change rust-toolchain: ```rustup override set nightly```, you might need ```rustup component add rust-src --toolchain nightly-aarch64-apple-darwin``` for MacOS-Darwin
3. Build Target: ```cargo +nightly build --target x86_64-weensyos.json```
4. Create Cargo Image: ```cargo bootimage``` (look for dependencies below). Image Created at `./target/x86_64-weensyos/debug/bootimage-weensyos.bin`
</details>

### Run QEMU-Display

Finally, run QEMU via bootimage runner ```cargo run --target your_custom_target.json [other_args] -- [qemu args]```. I use ```cargo run -- -nographic``` for zoo I/O only. Note: if you are stuck in the loop and cannot exit QEMU, try to `pkill qemu` from another terminal, we will implement quit commands soon. You can also run QEMU directly via ```qemu-system-x86_64 -nographic -drive format=raw,file=target/x86_64-weensyos/debug/bootimage-weensyos.bin``` (you can actually enable graphic if you work localy on your PC, no remote).


# How to test

Current implementation provides our own simple `cargo test` setup using standart `#[test_case]` implementation that outputs directly to the terminal via serial port and quits qemu. Public test cases for the project 4/5 will be implemented throuhgout WeensyOS growth.


# Environment Configs

Rust has three release channels: stable, beta, and nightly. The Rust Book explains the difference between these channels really well, so take a minute and check it out. For building an operating system, we will need some experimental features that are only available on the nightly channel, so we need to install a nightly version of Rust.
```rustc 1.80.0-nightly (87293c958 2024-05-08)```

Instead of writing our own bootloader, which is a project on its own, we use the bootloader crate. This crate implements a basic BIOS bootloader without any C dependencies, just Rust and inline assembly. Read more how bootimage linking works here: https://github.com/rust-osdev/bootimage.

<details>
  <summary>Dependency List</summary>

  1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  2. `cargo install bootimage`
  3. `rustup component add llvm-tools-preview`
  4. `apt install qemu-system-x86_64` for Linux or `brew isntall qemu` for MacOS
  
</details>


# Source Inspirations & How to contribute

Please, consider reading through these documentations is you're plan to contribute:

<details>
    <summary>Documentation Links</summary>

    1. https://os.phil-opp.com/
    2. https://www.theseus-os.com/
    3. https://zoo.cs.yale.edu/classes/cs323/323/proj5/starter-code/
</details>

Use Cisco VPN to connect off-campus https://docs.ycrc.yale.edu/clusters-at-yale/access/vpn/ or develop locally.

# Preview

Current version of WeensyOS looks like this:

<p align="center">
    <img width="717" alt="image" src="https://github.com/anton-mel/WeensyOS/assets/78281795/cb14c67e-3b1a-4b07-9d49-6ed974bb4885">
</p>
