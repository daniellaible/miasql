use regex::{Captures, Regex};
use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::database::database::Database;

#[derive(Clone, Debug, PartialEq)]
pub struct DropDatabase {}


impl Command for DropDatabase {
    fn parse(stmt: String, dbs: Vec<Database>) -> SqlCommand {
        let regex = Regex::new(r"(?i)DATABASE\s+(.+)").unwrap();
        let captures:Captures = regex.captures(&stmt).unwrap();
        let database = captures.get(1).unwrap().as_str();
        println!("{}", database);

        SqlCommand::DROP_DATABASE {
            command: String::from("DROP DATABASE"),
            database: database.to_string()
        }
    }
}

impl DropDatabase {
    pub fn default() -> Self {
        DropDatabase {}
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple_drop_db() {

    }
}