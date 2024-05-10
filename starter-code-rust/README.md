# WeesyOS in Rust

Project is currently under development. For any questions, please reach out via anton.melnychuk@yale.edu.

This OS is written in Rust @ Frog (Yale) `ssh yournetid@frog.zoo.cs.yale.edu` [host]. Note: All Ubuntu Nodes do NOT support qemu-system-x86_64. Use Cisco VPN to connect off-campus https://docs.ycrc.yale.edu/clusters-at-yale/access/vpn/

For [host]: CPU-arch: x86_64, Operating System: Linux (Fedora), Application binary interface: GNU
 
By compiling for our host triple, the Rust compiler and the linker assume that there is an underlying operating system such as Linux or Windows that uses the C runtime by default, which causes the linker errors. So, to avoid the linker errors, we compile for our own environment ```x86_64-weensyos.json``` with no underlying operating system. [look how to build]


# How to build

### Execute only once:

Get to the root: ```cd ./starter-code-rust```

Change rust-toolchain: ```rustup override set nightly```

Build Target: ```cargo +nightly build --target x86_64-weensyos.json```

Create Cargo Image: ```cargo bootimage```

Image Created at `./target/x86_64-weensyos/debug/bootimage-weensyos.bin`

(look for dependencies below)

### Run QEMU-Display

Finally, run QEMU ```qemu-system-x86_64 -nographic -drive format=raw,file=target/x86_64-weensyos/debug/bootimage-weensyos.bin``` (you can actually enable graphic if you work localy on your PC, I will keep it for now). Note: if you are stuck in the loop and cannot exit QEMU, try to `pkill qemu` from another terminal, we will implement quit commands soon.


# Environment Configs

Rust has three release channels: stable, beta, and nightly. The Rust Book explains the difference between these channels really well, so take a minute and check it out. For building an operating system, we will need some experimental features that are only available on the nightly channel, so we need to install a nightly version of Rust.
```rustc 1.80.0-nightly (87293c958 2024-05-08)```

Instead of writing our own bootloader, which is a project on its own, we use the bootloader crate. This crate implements a basic BIOS bootloader without any C dependencies, just Rust and inline assembly.


<details>
  <summary>Dependency List</summary>

  1. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  2. cargo install bootimage
  3. rustup component add llvm-tools-preview
  4. qemu-system-x86_64
  
</details>


# Source Inspirations

### Note: Please, consider reading through these documentations is you're plan to contribute

https://os.phil-opp.com/, https://www.theseus-os.com/, https://zoo.cs.yale.edu/classes/cs323/323/proj5/starter-code/
