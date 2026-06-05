use std::sync::atomic::AtomicU64;
use std::sync::{Mutex, OnceLock};
use std::thread;

pub struct Core{

}

pub static COUNTER: AtomicU64 = AtomicU64::new(0);
static INSTANCE: OnceLock<Core> = OnceLock::new();

impl Core{

    pub fn instance() -> &'static Core {
        INSTANCE.get_or_init(
            || Core { })
    }

    pub fn start_core() {
        let mut counter = 0;;
        loop {
            thread::sleep(std::time::Duration::from_millis(1));

        }
    }
}

/*pub fn run_core(){
    loop {
        //We wait 1 millisecond so the core doesn't consume too much CPU
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

pub fn use_database(database_name: &str) {
    todo!();
}

pub fn load_table(database_name: &str, table_name: &str) {
    todo!();
}*/

