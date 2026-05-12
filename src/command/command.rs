use crate::command::sqlcommands::SqlCommand;

pub trait Command{
     fn parse(stmt: String) -> SqlCommand;
}