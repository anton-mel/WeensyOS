# WeesyOS in Rust

WeensyOS is a tiny kernel that can run on bare-metal x86-64 machines (QEMU's emulated CPUs), developed by Professor Eddie Kohler. The initial state of the kernel contains code for bootstrapping kernel, handling exceptions/syscalls, executing user-level program, and helper functions for your CPSC 323 exercises.

This OS is written in Rust @ Frog VNC - Virtual Machine (Yale) `ssh yournetid@frog.zoo.cs.yale.edu`
https://docs.google.com/document/d/1tmYkB2v8LV6mo7BImCbWP3qO5Y_ZlgZIx0r5luZnUP0/edit?usp=sharing

> CPU-arch: x86_64, Operating System: Linux (Fedora), Application binary interface: GNU
 
By compiling for our host triple, the Rust compiler and the linker assume that there is an underlying operating system such as Linux (Frog Node) that uses the C runtime by default, which causes the linker errors. So, to avoid the linker errors, we compile for our own environment ```x86_64-weensyos.json``` with no underlying operating system *[look how to build]*.

> [!NOTE]
> Project is currently under development. For any questions, please reach out via <a href="mailto:anton.melnychuk@yale.edu">anton.melnychuk@yale.edu</a>.

# How to build

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

Rust has three release channels: stable, beta, and nightly. We will need some experimental features that are only available on the nightly channel, so we need to install a nightly version of Rust.
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

Please, consider reading through these documentations is you plan to contribute:

```
    1. https://os.phil-opp.com/
    2. https://www.theseus-os.com/
    3. https://zoo.cs.yale.edu/classes/cs323/323/proj5/starter-code/
```

Use Cisco VPN to connect off-campus https://docs.ycrc.yale.edu/clusters-at-yale/access/vpn/ or develop locally.

# Preview

> [!NOTE]
> The project is currently focused on developing the kernel's paging.

<p align="center">
    <img width="1006" alt="image" src="https://github.com/anton-mel/WeensyOS/assets/78281795/34bb102e-112d-4cae-bc05-e5956017bc96">
</p>


