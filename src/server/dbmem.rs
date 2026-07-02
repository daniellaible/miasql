use std::sync::{LazyLock, Mutex};
use crate::database::table::Table;

/// DbMem is the struct that holds the tables in memory.
/// It consists of a vector with all the tables that are in use.
/// The vector that stores those tables uses tupels which sre structures
/// Vec[(Database_Name, Table_Name, Table)]
#[derive(Debug)]
pub struct DbMem {
    pub tables: Vec<(String,String, Box<Table>)>,
}

static DBS: LazyLock<Mutex<DbMem>> = LazyLock::new(|| Mutex::new(DbMem { tables: vec![] }));

impl DbMem{

    pub fn init() {
        let mut dbs = DBS.lock().unwrap();
        dbs.tables = Vec::new();
    }

    pub fn add_table(table: Table){
        let mut dbs = DBS.lock().unwrap();
        dbs.tables.push((table.db_name.clone(), table.table_name.clone(), Box::new(table)));
    }

    pub fn is_table_loaded(db_name: String ,table_name: String) -> bool {
        let mut dbs = DBS.lock().unwrap();
        for i in 0 .. dbs.tables.len() {
            let (db_n, table_n, _) = &dbs.tables[i];
            if db_n.to_uppercase() == db_name.to_uppercase() && table_n.to_uppercase() == table_name.to_uppercase(){
                return true;
            }
        }
        false
    }

    //TODO implement
    pub fn remove_table(db_name:String, table_name:String){
        let mut dbs = DBS.lock().unwrap();

        for table in dbs.tables.iter_mut(){
            println!("needs to be implemented");
        }
    }

    pub fn print_tables() {
        let mut dbs = DBS.lock().unwrap();

        for table in dbs.tables.iter_mut(){
            println!("{:?}", table);
        }
    }

    pub fn calc_mem(){
        println!("needs to be implemented!");
    }

}

#[cfg(test)]
mod tests {
    use crate::database::table::Table;
    use crate::server::dbmem::DbMem;

    #[test]
    fn test_is_table_loaded_standard(){
        DbMem::init();
        let mut default_table = Table::default();
        default_table.db_name = String::from("business");
        default_table.table_name = String::from("customer");
        DbMem::add_table(default_table);
        let result = DbMem::is_table_loaded(String::from("business"), String::from("customer"));
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_table_loaded_case(){
        DbMem::init();
        let mut default_table = Table::default();
        default_table.db_name = String::from("Business");
        default_table.table_name = String::from("Customer");
        DbMem::add_table(default_table);
        let result = DbMem::is_table_loaded(String::from("business"), String::from("customer"));
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_table_loaded_bad_case(){
        DbMem::init();
        let mut default_table = Table::default();
        default_table.db_name = String::from("business");
        default_table.table_name = String::from("Employee");
        DbMem::add_table(default_table);
        let result = DbMem::is_table_loaded(String::from("business"), String::from("customer"));
        assert_eq!(result, false);
    }
}

