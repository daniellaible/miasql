use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::database::database::Database;

#[derive(Clone, Debug, PartialEq)]
pub struct DropTable {}


impl Command for DropTable {
    fn parse(stmt: String, dbs: Vec<Database>) -> SqlCommand {
        todo!()
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
