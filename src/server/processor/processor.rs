use crate::command::createdatabase::create_database;
use crate::command::createtable::create_table;
use crate::command::showdatabases::show_databases;
use crate::command::showtables::show_table;
use crate::command::sqlcommands::SqlCommand;
use crate::database::bptree::Node;
use crate::file::{moihandler, mtdhandler};
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionContext;
use crate::{command};
use log::info;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use anyhow::Error;
use tokio::net::TcpStream;
use crate::command::resultset::ResultSet;

pub static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn process_transaction(
    stream: &TcpStream,
    mut transaction: TransactionContext,
) -> anyhow::Result<()> {
    info!("In the processor: {:?}", transaction.command);

    let transaction_id = get_transaction_counter();
    transaction.is_processing = true;
    transaction.transaction_id = transaction_id;
    load_table_to_ram(transaction.clone());

    match &transaction.command {
        SqlCommand::Select { .. } => {
            todo!()
        }
        SqlCommand::DropTable { .. } => {
            todo!()
        }
        SqlCommand::DropDatabase { .. } => {
            todo!()
        }
        SqlCommand::Delete { .. } => {
            todo!()
        }
        SqlCommand::Truncate { .. } => {Ok(())}
        SqlCommand::Update { .. } => {Ok(())}
        SqlCommand::Insert { .. } => {Ok(())}
        SqlCommand::AlterAddColumn { .. } => {Ok(())}
        SqlCommand::AlterDropColumn { .. } => {Ok(())}
        SqlCommand::AlterRenameColumn { .. } => {Ok(())}
        SqlCommand::AlterModifyColumn { .. } => {Ok(())}
        SqlCommand::AlterTableRename { .. } => {Ok(())}
        SqlCommand::ShowDatabases { .. } => {
            let result = show_databases();
            print_resultset_to_stream(result, &stream);
            Ok(())
        }
        SqlCommand::ShowTables { .. } => {
            let resultset = show_table("system", "tables");
            print_resultset_to_stream(resultset, &stream);
            Ok(())
        }

        SqlCommand::CreateDatabase { database, .. } => {
            let last_id = moihandler::get_max_id("C:\\MiaSql\\system\\database.moi");
            transaction.row_id = last_id + 1;
            let result = create_database(transaction.clone(), database);
            match result {
                Ok(context) => {
                    if !context.error {
                        let line = format!("{} was created\n", database);
                        if let Err(e) = stream.try_write(line.as_bytes()) {
                            eprintln!("write failed: {e}");
                        }
                        Ok(())
                    } else {
                        let line = format!("There was an error while {} was created\n", database);
                        if let Err(e) = stream.try_write(line.as_bytes()) {
                            eprintln!("write failed: {e} {context}");
                        }
                        Ok(())
                    }
                }
                Err(why) => {
                    let line = format!("There was an error while {} was created - database already exists\n", database);
                    if let Err(e) = stream.try_write(line.as_bytes()) {
                        eprintln!("write failed: {why}");
                    }
                    Ok(())
                }
            }
        }
        SqlCommand::CreateTable { table, columns, .. } => {
            let result = create_table(transaction.clone(), table.to_string(), columns.clone());

            match result {
                Ok(t) => {
                    if !t.error {
                        let line = format!("{} was created\n", table);
                        if let Err(e) = stream.try_write(line.as_bytes()) {
                            eprintln!("write failed: {e}");
                        }
                        Ok(())
                    } else {
                        let line = format!("There was an error while {} was created\n", table);
                        if let Err(e) = stream.try_write(line.as_bytes()) {
                            eprintln!("write failed: {e} {t}");
                        }
                        Ok(())
                    }
                }
                _ => {
                    panic!("Something strange happend here while creating a table");
                }
            }
        }
        _ => {
            Ok(())
        }
    }
}

fn print_resultset_to_stream(result: anyhow::Result<ResultSet, Error>, stream:&TcpStream){
    match result{
        Ok(res) => {
            for i in 0..res.header.len(){
                let columnname = res.header[i].as_str().to_owned() + " ";
                stream.try_write(columnname.as_bytes());
            }
            stream.try_write("\n\r".as_bytes());
            for j in 0 .. res.rows.len(){
                let row = &res.rows[j];
                for k in 0 .. row.data.len(){
                    let dt = &row.data[k];
                    let datatype = dt.to_string() + " ";
                    stream.try_write(datatype.as_bytes());
                }
                stream.try_write("\n\r".as_bytes());
            }
        }
        Err(why) => {
            let line = "Something went wrong";
            stream.try_write(line.as_bytes());
            stream.try_write("\n".as_bytes());
        }
    }
}


fn load_table_to_ram(tp: TransactionContext) {
    for i in 0..tp.table_names.len() {
        let is_table_loaded = DbMem::is_table_loaded(tp.db_name.clone(), tp.table_names[i].clone());

        if is_table_loaded == false {
            println!("You need to load the table that isn't loaded");
            //todo: DbMem::load_table()
        }
    }
}


fn get_transaction_counter() -> u64 {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
