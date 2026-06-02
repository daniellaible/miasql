use std::sync::{Mutex, OnceLock};

#[derive(Debug)]
pub struct Config {
    pub config_version: f32,
    pub mia_version: String,
    pub licence_type: String,
    pub masterqueue_capacity: u32,
    pub ledger_location: String,
}

pub struct ConfigSingelton;

static INSTANCE: OnceLock<Mutex<Config>> = OnceLock::new();

impl ConfigSingelton {
    pub fn instance() -> &'static Mutex<Config> {
        INSTANCE.get_or_init(
            || Mutex::new(Config 
            {
                config_version: 0.1 ,
                mia_version: String::new(),
                licence_type: String::from("community"),
                masterqueue_capacity: 10,
                ledger_location: String::new(),
            }))
    }
}

