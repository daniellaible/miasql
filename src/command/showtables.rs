use anyhow::Error;
use crate::command::resultset::ResultSet;
use crate::database::bptree::Node;
use crate::database::table::Row;
use crate::server::dbmem::DbMem;

pub fn show_table(dbname: &str, tablename: &str) -> anyhow::Result<ResultSet, Error>{

    let mut result:ResultSet = ResultSet::create();
    if let Some(table_arc) = DbMem::find_table_in_mem(dbname, tablename) {
        let table_guard = table_arc.lock().unwrap();
        let tablemap = &table_guard.data;
        let header = &table_guard.column_names;
        result.header = header.clone();

        for (_, cur_row) in tablemap.hashmap.iter() {
            result.rows.push(cur_row.clone());
        }
    }

    anyhow::Ok(result)
}