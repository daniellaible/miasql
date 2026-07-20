use crate::command::sqlcommands::SqlCommand;

pub fn parse() -> SqlCommand {
    SqlCommand::ShowDatabases {
        command: String::from("SHOW DATABASES"),
    }
}