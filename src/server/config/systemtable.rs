use log::info;
use server::dbmem::AllDbSingelton;
use crate::database::database::Database;
use crate::server;
use crate::server::dbmem::DbMem;

pub fn import_system_tables() {
   let mut database: Database = server::config::systemtablereader::read_system_table();
   //let tables = database.get_tables();
    let interal_list_all_dbs = AllDbSingelton::instance();
    let db_mem: DbMem = DbMem {
       db_name: database.get_db_name().to_string(),
        //todo fix
       tables: Vec::new()
    };
   interal_list_all_dbs.databases.lock().unwrap().push(db_mem);

   info!("System tables imported {:?}", interal_list_all_dbs.databases.lock().unwrap());

}