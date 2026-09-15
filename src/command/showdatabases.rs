use anyhow::{Error};
use crate::command::resultset::ResultSet;
use crate::command::sqlcommands::SqlCommand;
use crate::database::bptree::Node;
use crate::database::table::{Row};
use crate::server::dbmem::DbMem;

pub fn parse() -> SqlCommand {
    SqlCommand::ShowDatabases {
        command: String::from("SHOW DATABASES"),
    }
}

pub fn show_databases() -> anyhow::Result<ResultSet, Error> {

   let mut result:ResultSet = ResultSet::create();
    if let Some(table_arc) = DbMem::find_table_in_mem("system", "database") {
        let table_guard = table_arc.lock().unwrap();
        let db_map = &table_guard.data;
        let header = &table_guard.column_names;
        result.header = header.clone();

        for (_, cur_row) in db_map.hashmap.iter() {
            result.rows.push(cur_row.clone());
        }

    }
    anyhow::Ok(result)
}


