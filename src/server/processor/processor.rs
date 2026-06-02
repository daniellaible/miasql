use crate::command::sqlcommands::SqlCommand;
use crate::server::queue::{TransactionProtocol, COUNTER};

pub fn process_transaction(command: SqlCommand) {

    let transaction_id = getTransactionCounter();
    let transaction_protocol: TransactionProtocol = TransactionProtocol {
        transaction_id,
        command,
        isMoiFileUpdated: false,
        isLedgerUpdated: false,
        isBTreeUpdated: false,
        isClusterUpdated: false,
        isShardUpdated: false,
        isErrorDetected : false,
        errorMsg: None,
    };

    {
        let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
        masterqueue
            .queue
            .lock()
            .unwrap()
            .push_back(transaction_protocol);

        updateMoiFile(transaction_id);
        updateLedgerFile(transaction_id);
        updateBTreeFile(transaction_id);
        updateClusterFile(transaction_id);
        updateShardFile(transaction_id);
    }

    let finished_transaction = removeTransaction(transaction_id);
    println!("finished transaction: {:?}", finished_transaction);
}

fn updateShardFile(transaction_id: u64) {
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.isShardUpdated = false;
    }
}

fn updateClusterFile(transaction_id: u64) {
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.isClusterUpdated = false;
    }
}

fn updateBTreeFile(transaction_id: u64) {
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.isBTreeUpdated = false;
    }
}

fn updateLedgerFile(transaction_id: u64) {
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.isLedgerUpdated = false;
    }
}

fn updateMoiFile(transaction_id: u64) {
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        transaction_protocol.isMoiFileUpdated = false;
    }
}

fn getTransactionCounter() -> u64 {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

fn removeTransaction (transaction_id: u64) -> Option<TransactionProtocol>{
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