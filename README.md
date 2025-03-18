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
