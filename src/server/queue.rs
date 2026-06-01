use std::sync::{Mutex, OnceLock};
use std::collections::VecDeque;
use std::sync::atomic::AtomicU64;
use crate::command::sqlcommands::SqlCommand;
use crate::server::config::ConfigSingelton;

#[derive(Debug, Clone)]
pub struct TransactionProtocol{
    pub transaction_id: u64,
    pub command: SqlCommand,
    pub isMoiFileUpdated: bool,
    pub isLedgerUpdated: bool,
    pub isBTreeUpdated: bool,
    pub isClusterUpdated: bool,
    pub isShardUpdated: bool,
    pub isErrorDetected : bool,
    pub errorMsg: Option<String>,
}

#[derive(Debug)]
pub struct MasterQueue {
    pub queue: Mutex<VecDeque<TransactionProtocol>>,
}

pub static COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct MasterQueueSingelton;

static INSTANCE: OnceLock<Mutex<MasterQueue>> = OnceLock::new();

impl MasterQueueSingelton {

   pub fn instance() -> &'static Mutex<MasterQueue> {
       let config = ConfigSingelton::instance().lock().unwrap();
       let ringbuffer: VecDeque<TransactionProtocol> = VecDeque::with_capacity(config.masterqueue_capacity as usize);

       INSTANCE.get_or_init(
            || Mutex::new(MasterQueue
            {
                queue: Mutex::new(ringbuffer),
            }))
    }
}