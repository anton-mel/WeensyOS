mod log;

pub fn init() {
    log::init().expect("logger can only be initialized once");
}
