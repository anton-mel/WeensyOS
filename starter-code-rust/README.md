### WeesyOs in Rust

This OS is written in Rust @ Cobra (Yale)
ssh yournetid@cobra.zoo.cs.yale.edu [host]

CPU-arch: x86_64
Vendor: Unknown
Operating System: Linux (Ubuntu)
Application binary interface: GNU
 
By compiling for our host triple, the Rust compiler and the linker assume that there is an underlying [host] operating system such as Linux or Windows that uses the C runtime by default, which causes the linker errors. So, to avoid the linker errors, we compile for our own environment ```x86_64-weensyos.json``` with no underlying operating system.

Note: Use Cisco VPN to connect off-campus
https://docs.ycrc.yale.edu/clusters-at-yale/access/vpn/ 
                                                   
Project is currently under development
For any questions, please reach out to 
anton.melnychuk@yale.edu


### How to build?

Note: .cargo/config.toml automatically sets 
the weensyOS target we created, but we need to
rebuild everything. Here are all the commands:

Change rust-toolchain: ```rustup override set nightly```

Create Cargo Image: ```cargo bootimage``` (only once)

Build OS: ```cargo build```

(look for dependencies below)

Finally, run QEMU ```qemu-system-x86_64 -drive format=raw,file=target/x86_64-weensyos/debug/bootimage-weensyos.bin```


# Environment Configs

Rust has three release channels: stable, beta, and nightly. The Rust Book explains the difference between these channels really well, so take a minute and check it out. For building an operating system, we will need some experimental features that are only available on the nightly channel, so we need to install a nightly version of Rust.
```rustc 1.80.0-nightly (87293c958 2024-05-08)```

Instead of writing our own bootloader, which is a project on its own, we use the bootloader crate. This crate implements a basic BIOS bootloader without any C dependencies, just Rust and inline assembly.


# Install Dependancies

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo install bootimage

rustup component add llvm-tools-preview


# Source Inspirations

Note: Please, consider reading throuhg these documentations is you're plan to contribute.

https://os.phil-opp.com/

https://www.theseus-os.com/

https://zoo.cs.yale.edu/classes/cs323/323/proj5/starter-code/
