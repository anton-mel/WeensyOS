
mod log;
pub mod serial;
pub mod vga_buffer;

pub fn init_log() {
    log::init().expect("logger can only be initialized once");
}
