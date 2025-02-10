// NO NEED TO UNDERSTAND THIS FILE.
// 
// When building an operating system, we lack the ability to dynamically allocate 
// memory for features like `String` or the `format!` macro, as they depend on 
// the standard library. As a result, all string-based operations use statically 
// allocated fixed-size buffers. To simplify and streamline string handling in 
// this constrained environment, we provide auxiliary helper functions.

use core::fmt::Write;

pub struct BufferWriter<'a> {
    pub buffer: &'a mut [u8],
    pub pos: usize,
}

impl<'a> BufferWriter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, pos: 0 }
    }
}

impl<'a> Write for BufferWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &byte in s.as_bytes() {
            if self.pos < self.buffer.len() {
                self.buffer[self.pos] = byte;
                self.pos += 1;
            } else {
                // buffer overflow
                break; 
            }
        }
        Ok(())
    }
}
