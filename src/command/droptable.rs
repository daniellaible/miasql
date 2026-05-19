use regex::{Captures, Regex};
use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::database::database::Database;

#[derive(Clone, Debug, PartialEq)]
pub struct DropTable {}


impl Command for DropTable {
    fn parse(stmt: String, dbs: Vec<Database>) -> SqlCommand {

        let regex = Regex::new(r"(?i)TABLE\s+(.+)").unwrap();
        let captures:Captures = regex.captures(&stmt).unwrap();
        let captured_table = captures.get(1).unwrap().as_str();
        println!("{}", captured_table);

       SqlCommand::DROP_TABLE {
           command: String::from("DROP TABLE"),
           table: captured_table.to_string()
       }
    }
}

impl DropTable {
    pub fn default() -> Self {
        DropTable {}
    }
}



#[cfg(test)]
mod tests {

    #[test]
    fn simple_drop_table() {

    }
}
