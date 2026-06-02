use crate::command::sqlcommands::SqlCommand;

pub(crate) fn write_ledger(transaction_id: u64) -> Result<bool, std::io::Error> {
    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();



    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        let command = transaction_protocol.command.clone();
        match command  {
            SqlCommand::SELECT {..} =>{}


            _ => {}
        };

        transaction_protocol.is_ledger_updated = false;
    }
    Ok(true)
}