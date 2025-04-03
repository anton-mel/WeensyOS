/// A trait to convert a value into text by writing into a mutable byte slice.
/// It returns the number of bytes written.
pub trait ToText {
    fn to_text(&self, buf: &mut [u8]) -> usize;
}

// Implementation for 32-bit integers.
impl ToText for i32 {
    fn to_text(&self, buf: &mut [u8]) -> usize {
        let mut pos = 0;
        let mut n = *self;
        if n < 0 {
            if pos < buf.len() {
                buf[pos] = b'-';
                pos += 1;
            }
            n = -n;
        }
        let mut temp = [0u8; 12];
        let mut tpos = 0;
        if n == 0 {
            temp[tpos] = b'0';
            tpos += 1;
        } else {
            while n > 0 {
                let digit = (n % 10) as u8;
                temp[tpos] = b'0' + digit;
                tpos += 1;
                n /= 10;
            }
        }
        while tpos > 0 {
            tpos -= 1;
            if pos < buf.len() {
                buf[pos] = temp[tpos];
                pos += 1;
            }
        }
        pos
    }
}

// Implementation for unsigned 64-bit integers.
impl ToText for u64 {
    fn to_text(&self, buf: &mut [u8]) -> usize {
        let mut pos = 0;
        let mut n = *self;
        let mut temp = [0u8; 21];
        let mut tpos = 0;
        if n == 0 {
            temp[tpos] = b'0';
            tpos += 1;
        } else {
            while n > 0 {
                let digit = (n % 10) as u8;
                temp[tpos] = b'0' + digit;
                tpos += 1;
                n /= 10;
            }
        }
        while tpos > 0 {
            tpos -= 1;
            if pos < buf.len() {
                buf[pos] = temp[tpos];
                pos += 1;
            }
        }
        pos
    }
}

// Implementation for raw pointers.
impl<T> ToText for *const T {
    fn to_text(&self, buf: &mut [u8]) -> usize {
        let mut pos = 0;
        let addr = *self as usize;
        if pos < buf.len() {
            buf[pos] = b'0';
            pos += 1;
        }
        if pos < buf.len() {
            buf[pos] = b'x';
            pos += 1;
        }
        let nibble_count = core::mem::size_of::<usize>() * 2;
        let mut started = false;
        for i in (0..nibble_count).rev() {
            let shift = i * 4;
            let nibble = ((addr >> shift) & 0xF) as u8;
            if nibble != 0 || started || i == 0 {
                started = true;
                let c = if nibble < 10 {
                    b'0' + nibble
                } else {
                    b'a' + (nibble - 10)
                };
                if pos < buf.len() {
                    buf[pos] = c;
                    pos += 1;
                }
            }
        }
        pos
    }
}

// Implement ToText for any fixed-size array of u8.
impl<const N: usize> ToText for [u8; N] {
    fn to_text(&self, buf: &mut [u8]) -> usize {
        let mut pos = 0;
        // Copy until we run out of space or encounter a null terminator.
        for &b in self.iter() {
            if pos < buf.len() {
                buf[pos] = b;
                pos += 1;
                if b == 0 {
                    break;
                }
            } else {
                break;
            }
        }
        pos
    }
}

// Implementation for string slices.
impl ToText for &str {
    fn to_text(&self, buf: &mut [u8]) -> usize {
        let bytes = self.as_bytes();
        let mut pos = 0;
        while pos < bytes.len() && pos < buf.len() {
            buf[pos] = bytes[pos];
            pos += 1;
        }
        pos
    }
}

/// Macro to generate a collated message from any number of arguments.
/// It expects a mutable byte buffer as the first argument, then a comma‑separated list
/// of expressions that implement `ToText`. It returns the total number of bytes written.
#[macro_export]
macro_rules! generate_msg {
    ($buf:expr, $($arg:expr),* $(,)?) => {{
         let mut pos = 0;
         $(
             pos += $arg.to_text(&mut $buf[pos..]);
         )*
         pos
    }};
}
