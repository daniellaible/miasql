use crate::database::table::Table;

#[derive(Debug,Clone)]
pub struct Database {
    pub db_name: String,
    pub tables: Vec<Table>,
}

impl Database {
    pub fn default() -> Self {
        Database {
            db_name: "".to_string(),
            tables: Vec::new(),
        }
    }
    
}
