
#[derive(Debug)]
pub struct Database {
    db_name: String,
    tables: Vec<(String,String)>,
}

impl Database {
    pub fn default() -> Self {
        Database {
            db_name: "".to_string(),
            tables: Vec::new(),
        }
    }

    pub fn set_db_name(&mut self, db_name: String) {
        self.db_name = db_name;
    }

    pub fn get_db_name(&mut self) -> &str {
        self.db_name.as_str()
    }

    pub fn add_table(&mut self, db_name: String, path: String) {
        self.tables.push((db_name, path));
    }

    pub fn get_tables(&mut self) -> Vec<(String, String)>{
        self.tables.clone()
    }

    //Todo remove table from database and rename table from database




    
    
}
