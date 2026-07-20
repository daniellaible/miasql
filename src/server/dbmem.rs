use crate::database::datatype::DataType;
use crate::database::table::{Row, Table};
use anyhow::{Result, anyhow};
use log::error;
use std::sync::{Arc, LazyLock, Mutex};

/// DbMem is the struct that holds the tables in memory.
/// It consists of a vector with all the tables that are in use.
/// The vector that stores those tables uses tupels with following structures
/// Vec[(Database_Name, Table_Name, Table)]
#[derive(Debug)]
pub struct DbMem {
    pub tables: Vec<(String, String, Arc<Mutex<Table>>)>,
}

static DBS: LazyLock<Mutex<DbMem>> = LazyLock::new(|| Mutex::new(DbMem { tables: vec![] }));

impl DbMem {
    /// This starts a new instance of the in-memory system of the database.
    /// There always should be max 1 of those instances
    /// This function is triggered when this program starts
    pub fn init() {
        let mut dbs = DBS.lock().unwrap();
        dbs.tables = Vec::new();
    }

    /// This adds a table to the in memory system of the database
    pub fn add_table(table: Table) {
        let mut dbs = DBS.lock().unwrap();
        dbs.tables.push((
            table.db_name.clone(),
            table.table_name.clone(),
            Arc::new(Mutex::new(table))
        ));
    }

    //This finds you a certain table you might want to work with
    pub fn find_table(db_name: &str, table_name: &str) -> Option<Arc<Mutex<Table>>> {
        let dbs = DBS.lock().unwrap();
        dbs.tables
            .iter()
            .find(|t| t.0 == db_name && t.1 == table_name)
            .map(|t| Arc::clone(&t.2))
    }

    /// This adds a row to a table in memory
    pub fn insert_row(db_name: &str, table_name: &str, row: Row) {
        let dbs = DBS.lock().unwrap();
        for (db_n, table_n, table_arc) in &dbs.tables {
            if db_n.eq_ignore_ascii_case(db_name) && table_n.eq_ignore_ascii_case(table_name) {
                let mut table = table_arc.lock().unwrap();
                match row.data[0] {
                    DataType::BigInt(number) => {
                        table.tree.insert(number, row.data.clone());
                    }
                    _ => error!("The first element is not the id ???"),
                }
                println!("{:?}", *table);
            }
        }
    }

    /// Checks if the table is in memory or not
    pub fn is_table_loaded(db_name: String, table_name: String) -> bool {
        let dbs = DBS.lock().unwrap();
        dbs.tables.iter().any(|(db_n, table_n, _)| {
            db_n.eq_ignore_ascii_case(&db_name) && table_n.eq_ignore_ascii_case(&table_name)
        })
    }

    //TODO implement
    pub fn remove_table(db_name: String, table_name: String) {
        todo!("implement")
    }

    pub fn print_tables() {
        let dbs = DBS.lock().unwrap();
        for (_, _, table_arc) in &dbs.tables {
            let table = table_arc.lock().unwrap();
            println!("{:?}", *table);
        }
    }

    pub fn calc_mem() {
        todo!("needs to be implemented");
    }
}

fn find_max_id(db_name: &str, table_name: &str) -> Result<i64> {
    let dbs = DBS.lock().unwrap();
    for (db_n, table_n, table_arc) in &dbs.tables {
        if db_name.eq_ignore_ascii_case(db_n) && table_name.eq_ignore_ascii_case(table_n) {
            let table = table_arc.lock().unwrap();
            return Ok(table.max_id);
        }
    }
    Err(anyhow!(
        "There is not db: {} with a table {}",
        db_name,
        table_name
    ))
}

fn check_constraints() -> bool {
    todo!()
}

fn check_datatypes() -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::database::table::Table;
    use crate::server::dbmem::DbMem;

    #[test]
    fn test_is_table_loaded_standard() {
        DbMem::init();
        let mut default_table = Table::default();
        default_table.db_name = String::from("business");
        default_table.table_name = String::from("customer");
        DbMem::add_table(default_table);
        let result = DbMem::is_table_loaded(String::from("business"), String::from("customer"));
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_table_loaded_case() {
        DbMem::init();
        let mut default_table = Table::default();
        default_table.db_name = String::from("Business");
        default_table.table_name = String::from("Customer");
        DbMem::add_table(default_table);
        let result = DbMem::is_table_loaded(String::from("business"), String::from("customer"));
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_table_loaded_bad_case() {
        DbMem::init();
        let mut default_table = Table::default();
        default_table.db_name = String::from("business");
        default_table.table_name = String::from("Employee");
        DbMem::add_table(default_table);
        let result = DbMem::is_table_loaded(String::from("business"), String::from("customer"));
        assert_eq!(result, false);
    }
}
