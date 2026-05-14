use crate::command::sqlcommands::SqlCommand;
use crate::database::database::Database;

pub trait Command{
     fn parse(stmt: String, dbs: Vec<Database>) -> SqlCommand;
}