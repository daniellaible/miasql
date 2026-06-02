use crate::command::sqlcommands::SqlCommand;
use crate::server::queue::{TransactionProtocol, COUNTER};

pub fn process_transaction(command: SqlCommand) {

    let transaction_id = get_transaction_counter();
    let transaction_protocol: TransactionProtocol = TransactionProtocol {
        transaction_id,
        command,
        is_moi_file_updated: false,
        is_ledger_updated: false,
        is_btree_updated: false,
        is_cluster_updated: false,
        is_shard_updated: false,
        is_error_detected: false,
        error_msg: None,
    };

    {
        let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
        masterqueue
            .queue
            .lock()
            .unwrap()
            .push_back(transaction_protocol);

        update_moi_file(transaction_id);
        update_ledger_file(transaction_id);
        update_btree_file(transaction_id);
        update_cluster_file(transaction_id);
        update_shard_file(transaction_id);
    }

    let finished_transaction = remove_transaction(transaction_id);
    println!("finished transaction: {:?}", finished_transaction);
}

fn update_shard_file(transaction_id: u64) {
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
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.is_ledger_updated = false;
    }
}

fn update_moi_file(transaction_id: u64) {
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

fn remove_transaction(transaction_id: u64) -> Option<TransactionProtocol>{
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