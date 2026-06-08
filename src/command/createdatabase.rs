use sqlparser::ast::Statement;
use crate::command::sqlcommands::SqlCommand;

pub fn parse(ast: Vec<Statement>) -> SqlCommand{
    let stmt = match ast.into_iter().next() {
        Some(s) => s,
        None => return SqlCommand::Undefined,
    };

    match stmt {
        Statement::CreateDatabase { db_name, comment, .. } => {
            SqlCommand::CreateDatabase {
                command: String::from("CREATE DATABASE"),
                database: db_name.to_string(),
                comment: comment.unwrap_or_default(),
            }
        }
        _ => SqlCommand::Undefined,
    }
}


#[cfg(test)]
mod tests {
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use crate::command::createdatabase::parse;
    use crate::command::sqlcommands::SqlCommand;

    fn parse_sql(sql: &str) -> SqlCommand {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, sql).unwrap();
        parse(ast)
    }

    #[test]
    fn simple_create_database() {
        match parse_sql("CREATE DATABASE employee") {
            SqlCommand::CreateDatabase { command, database, comment } => {
                assert_eq!(command, "CREATE DATABASE");
                assert_eq!(database, "employee");
                assert_eq!(comment, "");
            }
            _ => panic!("expected CREATE_DATABASE"),
        }
    }

    #[test]
    fn create_database_uppercase() {
        match parse_sql("CREATE DATABASE MYDB") {
            SqlCommand::CreateDatabase { database, .. } => {
                assert_eq!(database, "MYDB");
            }
            _ => panic!("expected CREATE_DATABASE"),
        }
    }

    #[test]
    fn create_database_lowercase_keyword() {
        match parse_sql("create database mydb") {
            SqlCommand::CreateDatabase { command, database, .. } => {
                assert_eq!(command, "CREATE DATABASE");
                assert_eq!(database, "mydb");
            }
            _ => panic!("expected CREATE_DATABASE"),
        }
    }

    #[test]
    fn create_database_mixed_case_keyword() {
        match parse_sql("Create Database MyDb") {
            SqlCommand::CreateDatabase { database, .. } => {
                assert_eq!(database, "MyDb");
            }
            _ => panic!("expected CREATE_DATABASE"),
        }
    }

    #[test]
    fn create_database_if_not_exists() {
        // IF NOT EXISTS is ignored in our output but must not crash
        match parse_sql("CREATE DATABASE IF NOT EXISTS employee") {
            SqlCommand::CreateDatabase { command, database, .. } => {
                assert_eq!(command, "CREATE DATABASE");
                assert_eq!(database, "employee");
            }
            _ => panic!("expected CREATE_DATABASE"),
        }
    }

    #[test]
    fn create_database_command_field_is_always_create_database() {
        match parse_sql("CREATE DATABASE foo") {
            SqlCommand::CreateDatabase { command, .. } => {
                assert_eq!(command, "CREATE DATABASE");
            }
            _ => panic!("expected CREATE_DATABASE"),
        }
    }

    #[test]
    fn create_database_comment_defaults_to_empty_string() {
        match parse_sql("CREATE DATABASE foo") {
            SqlCommand::CreateDatabase { comment, .. } => {
                assert_eq!(comment, "");
            }
            _ => panic!("expected CREATE_DATABASE"),
        }
    }

    #[test]
    fn create_database_with_underscore_name() {
        match parse_sql("CREATE DATABASE my_database") {
            SqlCommand::CreateDatabase { database, .. } => {
                assert_eq!(database, "my_database");
            }
            _ => panic!("expected CREATE_DATABASE"),
        }
    }

    #[test]
    fn create_database_with_numeric_suffix() {
        match parse_sql("CREATE DATABASE db2") {
            SqlCommand::CreateDatabase { database, .. } => {
                assert_eq!(database, "db2");
            }
            _ => panic!("expected CREATE_DATABASE"),
        }
    }

    // --- Negative / edge cases ---

    #[test]
    fn empty_ast_returns_undefined() {
        // Passing an empty vec directly
        let result = parse(vec![]);
        assert_eq!(result, SqlCommand::Undefined);
    }

    #[test]
    fn non_create_database_statement_returns_undefined() {
        let dialect = sqlparser::dialect::GenericDialect {};
        let ast = sqlparser::parser::Parser::parse_sql(&dialect, "SELECT 1").unwrap();
        let result = parse(ast);
        assert_eq!(result, SqlCommand::Undefined);
    }

    #[test]
    fn create_table_statement_returns_undefined() {
        let dialect = sqlparser::dialect::GenericDialect {};
        let ast = sqlparser::parser::Parser::parse_sql(
            &dialect,
            "CREATE TABLE foo (id INT)",
        ).unwrap();
        let result = parse(ast);
        assert_eq!(result, SqlCommand::Undefined);
    }
}