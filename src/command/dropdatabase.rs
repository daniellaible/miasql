use crate::command::command::Command;
use crate::command::droptable::DropTable;
use crate::command::sqlcommands::SqlCommand;
use crate::database::database::Database;

#[derive(Clone, Debug, PartialEq)]
pub struct DropDatabase {}


impl Command for DropDatabase {
    fn parse(stmt: String, dbs: Vec<Database>) -> SqlCommand {
        todo!()
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