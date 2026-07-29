use crate::command::resultset::ResultSet;
use crate::command::sqlcommands::SqlCommand;
use crate::server::config::config::ConfigSingelton;
use crate::server::processor::processor;
use anyhow::anyhow;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TransactionContext {
    pub db_name: String,
    pub user: String,
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

impl std::fmt::Display for TransactionContext {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "id: {} command:{:?}  error:{:?}",
            self.transaction_id, self.command, self.error
        )
    }
}

#[derive(Debug)]
pub struct TableGuard {
    pub is_working: AtomicBool,
}

pub struct MasterQueueSingelton;

static INSTANCE: OnceLock<TableGuard> = OnceLock::new();

impl MasterQueueSingelton {
    pub fn instance() -> &'static TableGuard {
        INSTANCE.get_or_init(|| TableGuard {
            is_working: AtomicBool::new(false),
        })
    }

    // TODO: here we could end up in a race condition or is it actually impossible since there is just one queue
    // and do_all_transactions is not public
    // High frequency parallel testing needed
    pub fn add(&self, transaction: TransactionContext) -> anyhow::Result<ResultSet, anyhow::Error> {
        let mut wait_duration = time::Duration::from_millis(1);
        let mut is_transaction_completed = false;
        while !is_transaction_completed {
            if MasterQueueSingelton::instance()
                .is_working
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                let result: anyhow::Result<ResultSet> = processor::process_transaction(transaction);
                MasterQueueSingelton::instance()
                    .is_working
                    .store(false, Ordering::SeqCst);
                is_transaction_completed = true;
                return result;
            } else {
                thread::sleep(wait_duration);
                if wait_duration.as_millis() <= 128 {
                    wait_duration = wait_duration * 2;
                }
            }
        }
        Err(anyhow!("This is weird- this should be unreachable"))
    }
}
