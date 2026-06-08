use crate::command::sqlcommands::SqlCommand;
use crate::server::config::config::ConfigSingelton;
use crate::server::processor::processor;
use std::collections::VecDeque;
use std::sync::atomic::AtomicU64;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone)]
pub struct TransactionProtocol {
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
            queue: Mutex::new(ringbuffer),
        })
    }

    pub fn add(&self, transaction: TransactionProtocol) {
        println!("In the MasterQueue: {:?}", &transaction);
        MasterQueueSingelton::instance()
            .queue
            .lock()
            .unwrap()
            .push_back(transaction);
        do_all_transactions()
    }
}
fn do_all_transactions() {
    let mut queue = MasterQueueSingelton::instance().queue.lock().unwrap();
    while queue.len() > 0 {
        processor::process_transaction(queue.pop_front().unwrap().command);
    }
}
