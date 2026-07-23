use anyhow::Error;
use crate::command::resultset::ResultSet;
use crate::command::sqlcommands::SqlCommand;
use crate::server::queue::TransactionContext;

pub fn parse() -> SqlCommand {
    SqlCommand::ShowDatabases {
        command: String::from("SHOW DATABASES"),
    }
}

pub fn show_databases(transaction: TransactionContext) -> anyhow::Result<ResultSet, Error>{
    todo!()
}