
#[derive(Clone, Debug, PartialEq)]
pub struct Database {
    db_name: String,
    path: String,
    tables: Vec<String>,
    users: Vec<String>,
}

impl Database {
    
    pub fn default() -> Self {
        Database {
            db_name: "".to_string(),
            path: "".to_string(),
            tables: Vec::new(),
            users: Vec::new(),
        }
    }
    
    
}
