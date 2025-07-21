![68747470733a2f2f692e696d6775722e636f6d2f38356a677446622e676966](https://github.com/user-attachments/assets/50aff54e-1746-4b40-ad5e-f8738af1393e)

# Rust WeensyOS 🦀

WeensyOS is a tiny kernel that can run on bare-metal x86-64 machines (QEMU's emulated CPUs). The initial state of the kernel contains code for bootstrapping kernel, handling exceptions/syscalls, executing user-level program, and helper functions for your CPSC 323 exercises.

> Host CPU-arch: x86_64, Operating System: Linux (Fedora), Application binary interface: GNU
 
By compiling for our host triple, the Rust compiler and the linker assume that there is an underlying operating system such as Linux that uses the C runtime by default, which causes the linker errors. So, to avoid the linker errors, we compile for our own environment ```x86_64-weensyos.json``` with no underlying operating system.

Before beginning work (spec is placed inside the `starter-code` folder), follow these 2 steps to configure your Rust compiler.

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
> Only for those who are using a Docker file provided or working locally on your Linux machine, you should disable the locking of `qemu` by manually modifying the `USE_HOST_LOCK=0` flag in GNUMakefile to 0 (line 50).

<p align="center">
  Congratulations! You're all set up to start working with WeensyOS in Rust!
</p>

## How to build

To start C-Rust Hybrid WeensyOS, run as usual `make run` or `make run-console`. You should see `PANIC: kernel/c/vm.c:45: assertion 'vmap.pa == addr' failed` error displayed.

If you choose to work on a fully memory-safe version of WeensyOS, follow these steps:

> [!NOTE]
> Find the original full Rust OS code here:
> https://github.com/anton-mel/RWeensyOS

<details>
<summary>Execute only once</summary>

1. Get to the root: ```cd ./starter-code-rust```
2. Change rust-toolchain: ```rustup override set nightly```, you might need ```rustup component add rust-src --toolchain nightly-aarch64-apple-darwin``` for MacOS-Darwin
3. Build Target: ```cargo +nightly build --target x86_64-weensyos.json```
4. Create Cargo Image: ```cargo bootimage``` (look for dependencies below). Image Created at `./target/x86_64-weensyos/debug/bootimage-weensyos.bin`
</details>

### Run QEMU-Display

Finally, run QEMU via bootimage runner ```cargo run --target your_custom_target.json [other_args] -- [qemu args]```. EDIT: Should work with just ```cargo run```. Note: if you are stuck in the loop and cannot exit QEMU (press `q` for `quit`), try to `pkill qemu` from another terminal. You can also run QEMU directly via ```qemu-system-x86_64 -nographic -drive format=raw,file=target/x86_64-weensyos/debug/bootimage-weensyos.bin``` (you can also enable QEMU graphic display if you work locally).

# How to test

Current implementation provides our own simple `cargo test` setup using standart `#[test_case]` implementation that outputs directly to the terminal via serial port and quits qemu. Public test cases for the project 4/5 will be implemented throuhgout WeensyOS growth.

# Environment Configs

Instead of writing our own bootloader, which is a project on its own, we use the bootloader crate. This crate implements a basic BIOS bootloader without any C dependencies, just Rust and inline assembly. Read more how bootimage linking works here: https://github.com/rust-osdev/bootimage.

<details>
  <summary>Dependency List</summary>

  1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  2. `cargo install bootimage`
  3. `rustup component add llvm-tools-preview`
  4. `apt install qemu-system-x86_64` for Linux or `brew isntall qemu` for MacOS
  
</details>


# Source Inspirations & How to contribute

Please, consider reading through these documentations is you plan to contribute:

```
    1. https://os.phil-opp.com/
    2. https://www.theseus-os.com/
    3. https://zoo.cs.yale.edu/classes/cs323/323/proj5/starter-code/
```

Use Cisco VPN to connect off-campus https://docs.ycrc.yale.edu/clusters-at-yale/access/vpn/ or develop locally.

<p align="center">
  🦀 Stay safe, stay Rusty. Good luck!
</p>




