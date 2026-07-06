use crate::command::sqlcommands::SqlCommand;
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionProtocol;
use crate::{command, ledger};
use log::{error, info};
use std::sync::atomic::AtomicU64;
use std::thread;
use crate::database::table::Row;

pub static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn process_transaction(mut transaction: TransactionProtocol) -> Option<TransactionProtocol> {
    info!("In the processor: {:?}", transaction.command);

    let transaction_id = get_transaction_counter();
    transaction.is_processing = true;
    transaction.transaction_id = transaction_id;

    load_table_to_ram(transaction.clone());

    {
        //update the b-tree
        let trans_clone_btree = transaction.clone();
        let btree_thread_handle = thread::spawn(move || {
            let table_update_result = update_table(trans_clone_btree);
            match table_update_result {
                Some(_) => {
                    transaction.is_btree_updated = true;
                    info!("Btrees updated");
                }
                None => {
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
                Some(_) => {
                    transaction.is_system_table_updated = true;
                    info!("System Table updated");
                }
                None => {
                    transaction.error = true;
                    info!("System Table NOT updated");
                }
            }
        });

        //update moi file
        let trans_clone_moi_file = transaction.clone();
        let moi_file_thread_handle = thread::spawn(move || {
            let moi_file_result = update_moi_file(trans_clone_moi_file);
            match moi_file_result {
                Some(_) => {
                    transaction.is_moi_file_updated = true;
                    info!("Moi file  updated");
                }
                None => {
                    transaction.error = true;
                    info!("Moi file NOT updated");
                }
            }
        });

        btree_thread_handle.join().unwrap();
        system_table_thread_handle.join().unwrap();
        moi_file_thread_handle.join().unwrap();
    }

    /*    let transaction_id = get_transaction_counter();
    info!("transaction_id: {}", transaction_id);
    let mut transaction_protocol: TransactionProtocol = TransactionProtocol {
        is_processing: true,
        is_finished: false,
        transaction_id,
        command: command.clone(),
        is_moi_file_updated: false,
        is_ledger_updated: false,
        is_btree_updated: false,
        is_cluster_updated: false,
        is_shard_updated: false,
        is_error_detected: false,
        is_system_table_updated: false,
        error_msg: None,
    };*/

    /*    {
        let btree_join_handle = thread::spawn(move || {
            update_table(transaction_protocol.transaction_id);
        });

        match command {

        }
    }*/

    Some(transaction)
}

fn load_table_to_ram(tp: TransactionProtocol) {
    for i in 0..tp.table_names.len() {
        let is_table_loaded = DbMem::is_table_loaded(tp.db_name.clone(), tp.table_names[i].clone());

        if is_table_loaded == false {
            println!("You need to load the table that isn't loaded");
            //todo: DbMem::load_table()
        }
    }
}

fn update_system_table(mut tp: TransactionProtocol) -> Option<TransactionProtocol> {
    match &tp.command {
        SqlCommand::CreateDatabase {database: db, .. } => {
            let result = command::createdatabase::execute(db);
            if result {
                tp.is_system_table_updated = true;
            }
        }
        _ => {
            //no need for lots of commands
        }
    }

    Some(tp)
}

fn update_moi_file(tp: TransactionProtocol) -> Option<TransactionProtocol> {
    match &tp.command {
        SqlCommand::CreateDatabase {..} => {
            
        }
        _ => {
            
        }
    }
    /*    println!("update file file");
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.is_moi_file_updated = false;
    }*/
    Some(tp)
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

fn update_table(mut tp: TransactionProtocol) -> Option<TransactionProtocol> {
    println!("update b-tree");

    match tp.command {
        _ => {
            //Create Database Command needs no btree update
        }
    }
    tp.is_btree_updated = true;
    Some(tp)
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

fn remove_transaction(transaction_id: u64) -> Option<TransactionProtocol> {
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
