use anyhow::Error;
use crate::command::resultset::ResultSet;
use crate::server::queue::TransactionContext;

pub fn show_tables(transaction: TransactionContext) -> anyhow::Result<ResultSet, Error>{
    todo!()
}