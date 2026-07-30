use crate::command::createdatabase::create_database;
use crate::command::createtable::create_table;
use crate::command::resultset::ResultSet;
use crate::command::showdatabases::show_databases;
use crate::command::showtables::show_table;
use crate::command::sqlcommands::SqlCommand;
use crate::file::{moihandler};
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionContext;
use log::info;
use std::sync::atomic::AtomicU64;
use anyhow::Error;
use crate::command::insert;

pub static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn process_transaction( mut transaction: TransactionContext) -> anyhow::Result<ResultSet> {
    info!("In the processor: {:?}", transaction.command);

    let transaction_id = get_transaction_counter();
    transaction.is_processing = true;
    transaction.transaction_id = transaction_id;
    load_table_to_ram(transaction.clone());

    match &transaction.command {
        SqlCommand::Select { .. } => {
            Ok(ResultSet::create())
        }
        SqlCommand::DropTable { .. } => {
            Ok(ResultSet::create())
        }
        SqlCommand::DropDatabase { .. } => {
            Ok(ResultSet::create())
        }
        SqlCommand::Delete { .. } => {
            Ok(ResultSet::create())
        }
        SqlCommand::Truncate { .. } => {Ok(ResultSet::create())}
        SqlCommand::Update { .. } => {Ok(ResultSet::create())}
        SqlCommand::Insert { table, columns, values, .. } => {
            let result : anyhow::Result<TransactionContext, Error> = insert::insert_into(&transaction,table, columns, values);
            Ok(ResultSet::create())
        }
        SqlCommand::AlterAddColumn { .. } => {Ok(ResultSet::create())}
        SqlCommand::AlterDropColumn { .. } => {Ok(ResultSet::create())}
        SqlCommand::AlterRenameColumn { .. } => {Ok(ResultSet::create())}
        SqlCommand::AlterModifyColumn { .. } => {Ok(ResultSet::create())}
        SqlCommand::AlterTableRename { .. } => {Ok(ResultSet::create())}
        SqlCommand::ShowDatabases { .. } => {
            show_databases()
        }
        SqlCommand::ShowTables { .. } => {
            show_table("system", "tables")
        }

        SqlCommand::CreateDatabase { database, .. } => {
            let last_id = moihandler::get_max_id("C:\\MiaSql\\system\\database.moi");
            transaction.row_id = last_id + 1;
            let result = create_database(transaction.clone(), database);
            match result {
                Ok(context) => {
                    if !context.error {
                        Ok(ResultSet::create())
                    } else {
                        Err(anyhow::anyhow!("create database failed"))
                    }
                }
                Err(why) => {
                    Err(anyhow::anyhow!("create database failed: {:?}", why))

                }
            }
        }
        SqlCommand::CreateTable { table, columns, .. } => {
            let result = create_table(transaction.clone(), table.to_string(), columns.clone());

            match result {
                Ok(t) => {
                    if !t.error {
                        Ok(ResultSet::create())
                    } else {
                        Err(anyhow::anyhow!("create table failed"))
                    }
                }
                _ => {
                    panic!("Something strange happend here while creating a table");
                }
            }
        }
        _ => {
            Err(anyhow::anyhow!("Something really strange happened here"))
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
