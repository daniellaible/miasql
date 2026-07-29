use crate::database::table::Table;

#[derive(Debug,Clone)]
pub struct Database {
    pub db_name: String,
    pub tables: Vec<Table>,
}

