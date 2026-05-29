use crate::command::sqlcommands::SqlCommand;
use sqlparser::ast::Truncate;

pub fn parse(truncate: Truncate) -> SqlCommand {
    let mut tables: Vec<String> = Vec::new();
    for table_target in truncate.table_names.iter() {
        tables.push(table_target.name.to_string());
    }

    SqlCommand::TRUNCATE {
        command: String::from("TRUNCATE"),
        tables,
    }
}
