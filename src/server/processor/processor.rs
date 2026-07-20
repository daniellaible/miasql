use std::ops::Deref;
use crate::command::sqlcommands::SqlCommand;
use crate::database::table;
use crate::file::{moihandler, mtdhandler};
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionContext;
use crate::{command, file};
use anyhow::anyhow;
use log::info;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use tokio::net::TcpStream;
use uuid::Uuid;

pub static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn process_transaction(mut stream: &TcpStream, mut transaction: TransactionContext) -> anyhow::Result<TransactionContext> {
    info!("In the processor: {:?}", transaction.command);

    let transaction_id = get_transaction_counter();
    transaction.is_processing = true;
    transaction.transaction_id = transaction_id;

    match &transaction.command {
        SqlCommand::CreateDatabase { .. } => {
            let last_id = moihandler::get_max_id("C:\\MiaSql\\system\\database.moi");
            transaction.row_id = last_id + 1;
        }
        SqlCommand::CreateTable { .. } => {
            let last_id = moihandler::get_max_id("C:\\MiaSql\\system\\tables.moi");
            transaction.row_id = last_id + 1;
            let uuid = Uuid::new_v4();
            transaction.table_uuid = uuid;
        }
        _ => {}
    }
    load_table_to_ram(transaction.clone());

    //update ledger file
    let ledger_clone_file = transaction.clone();
    let result = file::ledgerhandler::append_to_file(
        &ledger_clone_file.user,
        &ledger_clone_file.command,
        &ledger_clone_file.db_name,
    );
    match result {
        Ok(_) => {}
        Err(why) => {
            return Err(anyhow!("unable to update ledger file because: {}", why));
        }
    }

    //update the b-tree
    let trans_clone_btree = transaction.clone();
    let table_update_result = table::update_table(trans_clone_btree);
    match table_update_result {
        Ok(_) => {
            transaction.is_btree_updated = true;
            info!("Btrees updated");
        }
        Err(why) => {
            transaction.is_btree_updated = false;
            transaction.error = true;
            return Err(anyhow!("unable to update tree because:{}", why));
        }
    }

    //update system table if necessary
    let trans_clone_sys_tab = transaction.clone();

    let system_table_update_result = update_system_table(trans_clone_sys_tab);
    match system_table_update_result {
        Ok(_) => {
            transaction.is_system_table_updated = true;
            info!("System Table updated");
        }
        Err(why) => {
            transaction.is_system_table_updated = false;
            transaction.error = true;
            return Err(anyhow!("unable to update system table because:{}", why));
        }
    }

    //update/create mtd file
    let trans_clone_mtd_file = transaction.clone();
    let mtd_file_result = update_mtd_file(trans_clone_mtd_file);
    match mtd_file_result {
        Ok(_) => {
            transaction.is_mtd_file_updated = true;
            info!("MTD file  updated");
        }
        Err(why) => {
            transaction.is_mtd_file_updated = false;
            transaction.error = true;
            return Err(anyhow!("unable to update mtd file because:{}", why));
        }
    }

    let trans_select_show = transaction.clone();
    let select_result = select_and_show(&stream, trans_select_show);
    match mtd_file_result {
        Ok(_) => {

        }
        Err(why) => {

        }
    }

    //update moi file
    let trans_clone_moi_file = transaction.clone();

        let moi_file_result = moihandler::update(trans_clone_moi_file);
        match moi_file_result {
            Ok(_) => {
                transaction.is_moi_file_updated = true;
                info!("Moi file  updated");
            }
            Err(why) => {
                transaction.is_moi_file_updated = false;
                transaction.error = true;
                return Err(anyhow!("unable to update moi file because:{}", why));
            }
        }

    Ok(transaction)
}

fn select_and_show(mut stream: &TcpStream, tc: TransactionContext)  {
    match tc.command {
        SqlCommand::Select { .. } => {}
        SqlCommand::ShowDatabases { .. } => {
            let table = DbMem::find_table("system", "database");
            match table{
                Some(t) => {
                    let table_clone = Arc::clone(&t);
                    let guard = table_clone.lock().unwrap();

                    let tree = &guard.tree;
                    let root = &tree.root;
                    let left_leaf = tree.leftmost_leaf(root.clone());
                    
                    loop{
                        let leaf_guard = left_leaf.lock().unwrap();
                        
                        //shit
                    }
                },
                None => {}
            }
        }
        SqlCommand::ShowTables { .. } => {}
        _ => {}
    }
}

fn update_mtd_file(mut tp: TransactionContext) -> anyhow::Result<()> {
    match &tp.command {
        SqlCommand::CreateTable {
            table,
            columns,
            foreign_keys,
            ..
        } => {
            match mtdhandler::new_mtd_file(
                &tp.db_name,
                table,
                columns,
                foreign_keys,
                tp.table_uuid.clone(),
            ) {
                Ok(_) => tp.is_mtd_file_updated = true,
                Err(_) => tp.error = true,
            }
        }
        _ => {}
    }
    Ok(())
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

fn update_system_table(mut tp: TransactionContext) -> anyhow::Result<TransactionContext> {
    match &tp.command {
        SqlCommand::CreateDatabase { database: db, .. } => {
            let result = command::createdatabase::update_system_table(tp.row_id, db);
            match result {
                Ok(_) => {
                    tp.is_system_table_updated = true;
                }
                Err(_) => tp.error = true,
            }
        }
        SqlCommand::CreateTable { table, .. } => {
            let result = command::createtable::update_system_table(
                tp.row_id,
                Arc::from(tp.db_name.as_str()),
                Arc::from(table.as_str()),
            );
            match result {
                Ok(_) => tp.is_system_table_updated = true,
                Err(_) => {
                    tp.error = true;
                }
            }
        }
        _ => { }
    }

    Ok(tp)
}

fn get_transaction_counter() -> u64 {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
