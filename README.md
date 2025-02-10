## Dependencies

When building baremetal, we need to setUp correct target architecture to explain `cargo` how to compile the project. In out case WeensyOS is a x86_64 operating system.

```bash
rustup target add x86_64-unknown-none
```

```bash
rustup default nightly
```

```bash
cargo build --target x86_64-unknown-linux-gnu --release
```

## How to build

We modify the `Makefile` build and clean commands to add extra steps for the Rust compilation and correct linking. To start WeensyOS, run as usual `make run` or `make run-console`.

Note, WeensyOS is usually complitted in the Yale zoo environement. Here we let the students to install it locally if they have Linux machine, or use the `Dockerfile` with the provided documentation in the `devenv` folder. You can disable the locking of `qemu` being not killed on some zoo node by running command with `USE_HOST_LOCK=0` flag, or manually modifying the GNUMakefile to 0 (line 50).
