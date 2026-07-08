use std::sync::Arc;
use crate::command::sqlcommands::SqlCommand;
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionContext;
use crate::{command, ledger};
use log::{error, info};
use std::sync::atomic::AtomicU64;
use std::thread;
use uuid::Uuid;
use crate::database::datatype::DataType;
use crate::database::table::Row;
use crate::file::{moihandler, mtdhandler};

pub static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn process_transaction(mut transaction: TransactionContext) -> Option<TransactionContext> {
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

    {
        //update the b-tree
        let trans_clone_btree = transaction.clone();
        let btree_thread_handle = thread::spawn(move || {
            let table_update_result = update_table(trans_clone_btree);
            match table_update_result {
                Ok(_) => {
                    transaction.is_btree_updated = true;
                    info!("Btrees updated");
                }
                Err(_) => {
                    transaction.error = true;
                    info!("Btrees NOT updated");
                }
            }
        });

        //update system table if necessary
        let trans_clone_sys_tab = transaction.clone();
        let system_table_thread_handle = thread::spawn(move || {
            let system_table_update_result = update_system_table(trans_clone_sys_tab);
            match system_table_update_result {
                Ok(_) => {
                    transaction.is_system_table_updated = true;
                    info!("System Table updated");
                }
                Err(_) => {
                    transaction.error = true;
                    info!("System Table NOT updated");
                }
            }
        });

        //update/creat mtd file
        let trans_clone_mtd_file = transaction.clone();
        let mtd_file_thread_handle = thread::spawn(move || {
            let mtd_file_result = update_mtd_file(trans_clone_mtd_file);
            match mtd_file_result{
                Ok(_) => {
                    transaction.is_mtd_file_updated = true;
                    info!("MTD file  updated");
                }
                Err(_) => {
                    transaction.error = true;
                    info!("MTD file NOT updated");
                }
            }
        });

        //update moi file
        let trans_clone_moi_file = transaction.clone();
        let moi_file_thread_handle = thread::spawn(move || {
            let moi_file_result = update_moi_file(trans_clone_moi_file);
            match moi_file_result {
                Ok(_) => {
                    transaction.is_moi_file_updated = true;
                    info!("Moi file  updated");
                }
                Err(_) => {
                    transaction.error = true;
                    info!("Moi file NOT updated");
                }
            }
        });

        btree_thread_handle.join().unwrap();
        system_table_thread_handle.join().unwrap();
        moi_file_thread_handle.join().unwrap();
    }
    Some(transaction)
}

fn update_mtd_file(tp: TransactionContext) -> anyhow::Result<()> {
    match &tp.command {
        SqlCommand::CreateTable {table, columns, foreign_keys, ..} => {
            match mtdhandler::new_mtd_file(table, columns, foreign_keys, tp.table_uuid.clone()) {
                Ok(_) => {},
                Err(_) => {}
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
        SqlCommand::CreateDatabase {database: db, .. } => {
            let result = command::createdatabase::update_system_table(tp.row_id, db);
            match result {
                Ok(_) => {
                    tp.is_system_table_updated = true;
                },
                Err(_) => {
                    tp.error = true
                }
            }
        }
        SqlCommand::CreateTable {table, ..} => {
            let result = command::createtable::update_system_table(tp.row_id, Arc::from(tp.db_name.as_str()), Arc::from(table.as_str()));
            match result{
                Ok(_) => {
                    tp.is_system_table_updated = true
                },
                Err(_) => {
                    tp.error = true;
                }
            }
        }
        _ => {
            //no need for lots of commands
        }
    }

    Ok(tp)
}

fn update_moi_file(mut tp: TransactionContext) -> anyhow::Result<TransactionContext> {
    match &tp.command {
        SqlCommand::CreateDatabase {database, ..} => {
            let mut row: Row = Row{
                data: Vec::new(),
            };
            row.data.push(DataType::BigInt(tp.row_id));
            row.data.push(DataType::VarChar(database.len() as u8, String::from(database)));
            moihandler::add_row("C:\\MiaSql\\system\\database.moi", row).expect("Unable to update database moi file");
        }
        SqlCommand::CreateTable {table, ..} => {
            let mut row: Row = Row{
                data: Vec::new(),
            };
            let database = tp.db_name.clone();
            tp.table_names.push(table.clone());

            let path = "C:\\MiaSql\\tables\\".to_owned() + tp.table_uuid.to_string().as_str() + ".mtd";
            row.data.push(DataType::BigInt(tp.row_id));
            row.data.push(DataType::VarChar(database.len() as u8, String::from(database)));
            row.data.push(DataType::VarChar(table.len() as u8, String::from(table)));
            row.data.push(DataType::VarChar(path.len() as u8, String::from(path)));
            moihandler::add_row("C:\\MiaSql\\system\\tables.moi", row).expect("Unable to update database moi file");
        }
        _ => {

        }
    }
    Ok(tp)
}



fn update_shard_file(transaction_id: u64) {
    println!("update shard");
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.is_shard_updated = false;
    }
}

fn update_cluster_file(transaction_id: u64) {
    println!("update cluster");
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.is_cluster_updated = false;
    }
}

fn update_table(mut tp: TransactionContext) -> anyhow::Result<TransactionContext> {
    println!("update b-tree");

    match tp.command {
        _ => {
            //Create Database Command needs no btree update
        }
    }
    tp.is_btree_updated = true;
    Ok(tp)
}

fn update_ledger_file(transaction_id: u64) {
    let _ = ledger::writer::write_ledger(transaction_id);

    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.is_ledger_updated = true;
    }
}



fn get_transaction_counter() -> u64 {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

fn remove_transaction(transaction_id: u64) -> Option<TransactionContext> {
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(index) = queue
        .iter()
        .position(|tp| tp.transaction_id == transaction_id)
    {
        return queue.remove(index);
    }
    None
}
