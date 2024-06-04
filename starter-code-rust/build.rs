
fn main() {
    println!("cargo:rerun-if-changed=src/kernel/boot.rs");
    println!("cargo:rerun-if-changed=link/boot.ld");
    println!("cargo:rerun-if-changed=link/kernel.ld");
    println!("cargo:rerun-if-changed=link/p-allocator.ld");
    println!("cargo:rerun-if-changed=link/p-alloctests.ld");
    println!("cargo:rerun-if-changed=link/process.ld");
    println!("cargo:rerun-if-changed=link/shared.ld");
}
