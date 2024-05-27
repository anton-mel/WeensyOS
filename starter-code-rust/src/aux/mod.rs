
mod log;

pub fn init_log() {
    log::init().expect("logger can only be initialized once");
}
