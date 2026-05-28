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
                    SqlCommand::DROP_DATABASE {
                        command: String::from("DROP DATABASE"),
                        database: names[0].to_string(),
                    }
                },
                ObjectType::Table => {
                    SqlCommand::DROP_TABLE {
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

#[cfg(test)]
mod tests {
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use crate::command::drop::parse;
    use crate::command::sqlcommands::SqlCommand;

    fn parse_sql(sql: &str) -> SqlCommand {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, sql).unwrap();
        parse(ast)
    }

    #[test]
    fn simple_drop_database() {
        match parse_sql("DROP DATABASE employee") {
            SqlCommand::DROP_DATABASE { database, .. } => {
                assert_eq!(database, "employee");
            }
            _ => panic!("Expected DROP DATABASE command"),
        }
    }

    #[test]
    fn simple_drop_table() {
        match parse_sql("DROP TABLE employee") {
            SqlCommand::DROP_TABLE { table, .. } => {
                assert_eq!(table, "employee");
            }
            _ => panic!("Expected DROP TABLE command"),
        }
    }
}

