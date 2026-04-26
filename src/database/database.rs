pub struct Database {
    db_name: String,
    path: String,
    //users: Vec<>,
}

impl Database {
    
    pub fn default() -> Self {
        Database {
            db_name: "".to_string(),
            path: "".to_string(),
        }
    }
}
