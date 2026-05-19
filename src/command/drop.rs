use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::command::update::Update;
use crate::database::database::Database;

#[derive(Clone, Debug, PartialEq)]
pub struct Drop {}


impl Command for Drop {
    fn parse(stmt: String, dbs: Vec<Database>) -> SqlCommand {
        todo!()
    }
}

impl Drop {
    pub fn default() -> Self {
        Drop {}
    }
}



#[cfg(test)]
mod tests {

    #[test]
    fn simple_drop_table() {

    }
}
