use crate::command::sqlcommands::SqlCommand;
use crate::server::config::config::ConfigSingelton;
use crate::server::processor::processor;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use log::info;

#[derive(Debug, Clone)]
pub struct TransactionProtocol {
    pub is_processing: bool,
    pub is_finished: bool,
    pub transaction_id: u64,
    pub command: SqlCommand,
    pub is_moi_file_updated: bool,
    pub is_ledger_updated: bool,
    pub is_btree_updated: bool,
    pub is_cluster_updated: bool,
    pub is_shard_updated: bool,
    pub is_error_detected: bool,
    pub error_msg: Option<String>,
}

#[derive(Debug)]
pub struct MasterQueue {
    pub is_working: AtomicBool,
    pub queue: Mutex<VecDeque<TransactionProtocol>>,
}

pub static COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct MasterQueueSingelton;

static INSTANCE: OnceLock<MasterQueue> = OnceLock::new();

impl MasterQueueSingelton {

    pub fn instance() -> &'static MasterQueue {
        let config = ConfigSingelton::instance().lock().unwrap();
        let ringbuffer: VecDeque<TransactionProtocol> =
            VecDeque::with_capacity(config.masterqueue_capacity as usize);
        INSTANCE.get_or_init(|| MasterQueue {
            is_working: AtomicBool::new(false),
            queue: Mutex::new(ringbuffer),
        })
    }

    // TODO: here we could end up in a race condition or is it actually impossible since there is just one queue
    // and do_all_transactions is not public
    // High frequency parallel testing
    pub fn add(&self, transaction: TransactionProtocol) {
        MasterQueueSingelton::instance()
            .queue
            .lock()
            .unwrap()
            .push_back(transaction);
       if !MasterQueueSingelton::instance().is_working.load(Ordering::SeqCst) {
            do_all_transactions();
       }

    }
}
fn do_all_transactions() {
    MasterQueueSingelton::instance().is_working.store(true, Ordering::SeqCst);
    let mut queue = MasterQueueSingelton::instance().queue.lock().unwrap();
    while queue.len() > 0 {
        processor::process_transaction(&queue.pop_front().unwrap().command);
    }
    MasterQueueSingelton::instance().is_working.store(false, Ordering::SeqCst);
}
