use crate::command::sqlcommands::SqlCommand;
use crate::server::queue::{COUNTER, TransactionProtocol};
use std::thread;

use crate::ledger;


pub fn process_transaction(command: &SqlCommand) {
    println!(" ----  ");
    println!("In the processor: {:?}", command);
    println!(" ----  ");

/*    let transaction_id = get_transaction_counter();
    let transaction_protocol: TransactionProtocol = TransactionProtocol {
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
        error_msg: None,
    };*/

    {
/*        let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
        masterqueue
            .queue
            .lock()
            .unwrap()
            .push_back(transaction_protocol);

        let moi_join_handle = thread::spawn(move || {
            update_moi_file(transaction_id);
        });*/

        // Not needed right now - however mostly implemented. But we need a way to store the ledger counter,
        // The easiest and fastst way would probably be withing a systems table. But we don't have implemented them yet
        // therefore we ignore the ledger file now and finish implementing it when we have system tables.
        /*        let ledger_join_handle = thread::spawn(move || {
            update_ledger_file(transaction_id);
        });*/

/*        let btree_join_handle = thread::spawn(move || {
            update_btree_file(transaction_id);
        });

        let cluster_join_handle = thread::spawn(move || {
            update_cluster_file(transaction_id);
        });

        let shard_join_handle = thread::spawn(move || {
            update_shard_file(transaction_id);
        });

        moi_join_handle.join().unwrap();*/

        // Turned off . don't foregt to turn it back on when ready
        //ledger_join_handle.join().unwrap();
/*
        btree_join_handle.join().unwrap();
        cluster_join_handle.join().unwrap();
        shard_join_handle.join().unwrap();*/


    }

/*    let finished_transaction = remove_transaction(transaction_id);
    println!("finished transaction: {:?}", finished_transaction);*/
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

fn update_btree_file(transaction_id: u64) {
    println!("update b-tree");
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.is_btree_updated = false;
    }
}

fn update_ledger_file(transaction_id: u64) {
    ledger::writer::write_ledger(transaction_id);

    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.is_ledger_updated = true;
    }
}

fn update_moi_file(transaction_id: u64) {
    println!("update moi file");
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.is_moi_file_updated = false;
    }
}

fn get_transaction_counter() -> u64 {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
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
