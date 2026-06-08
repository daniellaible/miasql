use std::sync::{Mutex, OnceLock};

#[derive(Debug)]
pub struct DbMem {
    pub db_name: String,
    pub tables: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct AllDatabases {
    pub databases: Mutex<Vec<DbMem>>,
}

pub struct AllDbSingelton;

static INSTANCE: OnceLock<AllDatabases> = OnceLock::new();

impl AllDbSingelton {
    pub fn instance() -> &'static AllDatabases {
        INSTANCE.get_or_init(|| AllDatabases {
            databases: Mutex::new(Vec::new()),
        })
    }
    
    pub fn add_db(&self, db_mem: DbMem){
        Self::instance().databases.lock().unwrap().push(db_mem);
    }
}
