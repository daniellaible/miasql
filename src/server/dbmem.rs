use crate::database::bptree::Node;
use crate::database::datatype::DataType;
use crate::database::tabel::Tabel;
use crate::file::moihandler::load_moi_file;
use crate::file::mtdhandler::read_mtd_file;
use crate::server::tools::{clean_string, datatype_to_string_uppercase, remove_double_slash};
use anyhow::{Error, Result, anyhow};
use log::{error, info};
use std::sync::{Arc, LazyLock, Mutex};

/// DbMem is the struct that holds the tables in memory.
/// It consists of a vector with all the tables that are in use.
/// The vector that stores those tables uses tupels with following structures
/// Vec[(Database_Name, Table_Name, Table)]
#[derive(Debug)]
pub struct DbMem {
    pub tables: Vec<(String, String, Arc<Mutex<Tabel>>)>,
}

static DBS: LazyLock<Mutex<DbMem>> = LazyLock::new(|| Mutex::new(DbMem { tables: vec![] }));

impl DbMem {
    /// This starts a new instance of the in-memory system of the database.
    /// There always should be max 1 of those instances
    /// This function is triggered when the db engine starts
    pub fn init() {
        let mut dbs = DBS.lock().unwrap();
        dbs.tables = Vec::new();
    }

    /// This adds a table to the in memory system of the database
    pub fn add_table(table: Tabel) {
        let mut dbs = DBS.lock().unwrap();
        dbs.tables.push((
            table.db_name.clone(),
            table.table_name.clone(),
            Arc::new(Mutex::new(table)),
        ));
    }

    //This finds you a certain table you might want to work with
    pub fn find_table_in_mem(db_name: &str, table_name: &str) -> Option<Arc<Mutex<Tabel>>> {
        let dbs = DBS.lock().unwrap();

        for i in 0..dbs.tables.len() {
            let (db, local_table, table) = &dbs.tables[i].clone();
            if db.to_uppercase() == db_name.to_uppercase()
                && local_table.to_uppercase() == table_name.to_uppercase()
            {
                return Some(Arc::clone(table));
            }
        }
        None
    }

    pub fn load_table_from_system_tables(dbname: &str, tablename: &str) -> Result<String, Error> {
        match Self::find_table_in_mem("system", "tables") {
            None => {
                panic!("System table can not be found");
            }
            Some(system_table) => {
                let datatable = &system_table.lock().unwrap().data;
                for (_, row) in datatable.data.iter() {
                    let mut dbname_in_system_table = datatype_to_string_uppercase(&row.data[1]);
                    dbname_in_system_table = clean_string(dbname_in_system_table);

                    let mut tablename_in_system_table = datatype_to_string_uppercase(&row.data[2]);
                    tablename_in_system_table = clean_string(tablename_in_system_table);

                    let mut temp_dbname = dbname.to_string();
                    temp_dbname = temp_dbname.to_uppercase();
                    temp_dbname = clean_string(temp_dbname);

                    let mut temp_tablename = tablename.to_string();
                    temp_tablename = temp_tablename.to_uppercase();
                    temp_tablename = clean_string(temp_tablename);

                    if dbname_in_system_table == temp_dbname
                        && tablename_in_system_table == temp_tablename
                    {
                        let path = remove_double_slash(clean_string(row.data[3].to_string()));
                        let mdtfile = read_mtd_file(path.as_str());
                        let table_result = load_moi_file(&mdtfile);
                        match table_result {
                            Ok(mut table) => {
                                info!("Table {:?} added to DB", &table.table_name);
                                table.mtd_path = path.clone();
                                Self::add_table(table);
                                return Ok(path);
                            }
                            Err(why) => {
                                return Err(anyhow!("{:?}", why));
                            }
                        }
                    }
                }
            }
        }
        Err(anyhow!("Unable to load table from system"))
    }

    //load a given database into RAM
    pub fn load_db_to_mem(db_name: &str) -> Result<()> {
        match Self::find_table_in_mem("system", "tables") {
            None => {
                panic!("System table can not be found");
            }
            Some(table) => {
                let datamap = &table.lock().unwrap().data;
                for (_, row) in datamap.data.iter() {

                    let mut table_value_upper = datatype_to_string_uppercase(&row.data[1]);
                    table_value_upper = clean_string(table_value_upper);

                    let mut db_name_as_upper = db_name.to_string().to_uppercase();
                    db_name_as_upper = clean_string(db_name_as_upper);

                    if table_value_upper == db_name_as_upper {
                        let path = remove_double_slash(clean_string(row.data[3].to_string()));
                        let mdtfile = read_mtd_file(path.as_str());
                        let table_result = load_moi_file(&mdtfile);
                        match table_result {
                            Ok(mut table) => {
                                println!("Table {:?} added to Memory", &table.table_name);
                                table.mtd_path = path;
                                Self::add_table(table);
                            }
                            Err(why) => {
                                anyhow!("{:?}", why);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// This adds a row to a table in memory
    pub fn insert_row(db_name: &str, table_name: &str, data: Vec<DataType>) {
        panic!("needs new implementation");
        //Todo here we create our own id, if it is not given
        /*        let id = match data.first() {
            Some(DataType::BigInt(n)) => *n,
            Some(_) => {
                error!("insert_row: first element must be DataType::BigInt(id)");
                return;
            }
            None => {
                error!("insert_row: row is empty");
                return;
            }
        };
        let table_arc: Arc<Mutex<Table>> = {
            let dbs = DBS.lock().unwrap();

            match dbs.tables.iter().find(|(db_n, table_n, _)| {
                db_n.eq_ignore_ascii_case(db_name) && table_n.eq_ignore_ascii_case(table_name)
            }) {
                Some((_, _, arc)) => Arc::clone(arc),
                None => {
                    error!("insert_row: table not found: {}.{}", db_name, table_name);
                    return;
                }
            }
        };

        match table_arc.try_lock() {
            Ok(mut table) => {
                table.tree.insert(id, data);
            }
            Err(why) => {
                error!("insert_row: failed to lock table: {:?}", why);
            }
        }*/
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
    /*    #[test]
    fn test_is_table_loaded_standard() {
        DbMem::init();
        let mut default_table = Table::default();
        default_table.db_name = String::from("business");
        default_table.table_name = String::from("customer");
        DbMem::add_table(default_table);
        let result = DbMem::is_table_loaded(String::from("business"), String::from("customer"));
        assert_eq!(result, true);
    }*/

    /*    #[test]
    fn test_is_table_loaded_case() {
        DbMem::init();
        let mut default_table = Table::default();
        default_table.db_name = String::from("Business");
        default_table.table_name = String::from("Customer");
        DbMem::add_table(default_table);
        let result = DbMem::is_table_loaded(String::from("business"), String::from("customer"));
        assert_eq!(result, true);
    }*/

    /*    #[test]
    fn test_is_table_loaded_bad_case() {
        DbMem::init();
        let mut default_table = Table::default();
        default_table.db_name = String::from("business");
        default_table.table_name = String::from("Employee");
        DbMem::add_table(default_table);
        let result = DbMem::is_table_loaded(String::from("business"), String::from("customer"));
        assert_eq!(result, false);
    }*/
}
