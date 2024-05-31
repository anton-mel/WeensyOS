#[inline]
pub fn rdtsc() -> u64 {
    let lo: u32;
    let hi: u32;

    unsafe {
        asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
        );
    }

    ((hi as u64) << 32) | (lo as u64)
}
