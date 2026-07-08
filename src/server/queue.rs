use crate::command::sqlcommands::SqlCommand;
use crate::server::config::config::ConfigSingelton;
use crate::server::processor::processor;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::{thread, time};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TransactionProtocol {
    pub db_name: String,
    pub table_uuid: Uuid,
    pub row_id: i64,
    pub table_names: Vec<String>,
    pub is_processing: bool,
    pub is_finished: bool,
    pub transaction_id: u64,
    pub command: SqlCommand,
    pub is_moi_file_updated: bool,
    pub is_mtd_file_updated: bool,
    pub is_ledger_updated: bool,
    pub is_btree_updated: bool,
    pub is_cluster_updated: bool,
    pub is_shard_updated: bool,
    pub is_system_table_updated: bool,
    pub error: bool,
}

impl std::fmt::Display for TransactionProtocol<> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "id: {} command:{:?}  error:{:?}", self.transaction_id, self.command, self.error)
    }
}

#[derive(Debug)]
pub struct MasterQueue {
    pub is_working: AtomicBool,
    pub queue: Mutex<VecDeque<TransactionProtocol>>,
}

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
    pub fn add(&self, transaction: TransactionProtocol) -> Option<TransactionProtocol> {
        let mut wait_duration = time::Duration::from_millis(1);
        let mut is_transaction_completed = false;
        let mut transaction_result  = None;
        while !is_transaction_completed {
            if !MasterQueueSingelton::instance().is_working.load(Ordering::SeqCst) {
                transaction_result = do_transactions(transaction.clone());
                is_transaction_completed = true;
            } else {
                thread::sleep(wait_duration);
                if wait_duration.as_millis() <= 128 {
                    wait_duration = wait_duration * 2;
                }
            }
        }
        transaction_result

    }
}
pub fn do_transactions(tp: TransactionProtocol) -> Option<TransactionProtocol> {
    MasterQueueSingelton::instance().is_working.store(true, Ordering::SeqCst);
    //let mut queue = MasterQueueSingelton::instance().queue.lock().unwrap();
    let transaction_result = processor::process_transaction(tp);
    MasterQueueSingelton::instance().is_working.store(false, Ordering::SeqCst);
    transaction_result
}
