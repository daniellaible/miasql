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

    pub fn new(){
        let mut dbs = DBS.lock().unwrap();
        dbs.tables = Vec::new();

    }

    pub fn add_table(table: Table){
        let mut dbs = DBS.lock().unwrap();
        dbs.tables.push((table.db_name.clone(), table.table_name.clone(), Box::new(table)));
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

