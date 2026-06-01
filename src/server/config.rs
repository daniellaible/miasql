use std::sync::{Mutex, OnceLock};

#[derive(Debug)]
pub struct Config {
    pub version: f32,
}

pub struct ConfigSingelton;

static INSTANCE: OnceLock<Mutex<Config>> = OnceLock::new();

impl ConfigSingelton {
    pub fn instance() -> &'static Mutex<Config> {
        INSTANCE.get_or_init(
            || Mutex::new(Config { version: 0.1 }))
    }
}

