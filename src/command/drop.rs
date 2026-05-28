use sqlparser::ast::{ObjectType, Statement};
use crate::command::sqlcommands::SqlCommand;

pub fn parse(ast: Vec<Statement>) -> SqlCommand{
    let stmt = match ast.into_iter().next() {
        Some(s) => s,
        None => return SqlCommand::UNDEFINED,
    };

    match stmt {
        Statement::Drop { object_type, names, .. } => {
             match object_type {
                ObjectType::Database => {
                    return SqlCommand::DROP_DATABASE {
                        command: String::from("DROP DATABASE"),
                        database: names[0].to_string(),
                    }
                },
                ObjectType::Table => {
                    return SqlCommand::DROP_TABLE {
                        command: String::from("DROP TABLE"),
                        table: names[0].to_string(),
                    }
                }
                _ => SqlCommand::UNDEFINED,
            }
        }
        _ => SqlCommand::UNDEFINED
    }
}