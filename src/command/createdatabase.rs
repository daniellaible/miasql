use crate::command::sqlcommands::SqlCommand;
use crate::database::datatype::DataType;
use crate::database::table::Row;
use crate::file;
use crate::file::moihandler;
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionContext;
use anyhow::{Error, anyhow};
use sqlparser::ast::Statement;

pub fn parse(ast: Vec<Statement>) -> SqlCommand {
    let stmt = match ast.into_iter().next() {
        Some(s) => s,
        None => return SqlCommand::Undefined,
    };

    match stmt {
        Statement::CreateDatabase {
            db_name, comment, ..
        } => SqlCommand::CreateDatabase {
            command: String::from("CREATE DATABASE"),
            database: db_name.to_string(),
            comment: comment.unwrap_or_default(),
        },
        _ => SqlCommand::Undefined,
    }
}

pub fn create_database(
    mut transaction: TransactionContext,
    dbname: &str,
) -> anyhow::Result<TransactionContext, Error> {
    let ledger_clone_file = transaction.clone();
    let result = file::ledgerhandler::append_to_file(
        &ledger_clone_file.user,
        &ledger_clone_file.command,
        &ledger_clone_file.db_name,
    );
    match result {
        Ok(_) => {
            let id = transaction.row_id.clone();
            let mut result_system_table_update = update_system_table(transaction, id);
            match result_system_table_update {
                Ok(mut t) => {
                    t.is_system_table_updated = true;

                    let result_moi_update: anyhow::Result<TransactionContext> =
                        update_database_moi(t.clone());
                    match result_moi_update {
                        Ok(mut t) => {
                            t.is_moi_file_updated = true;
                            Ok(t)
                        }
                        Err(why) => Err(anyhow!("Unable to create database {:?}", t)),
                    }
                }
                Err(why) => Err(anyhow!("Unableto update system table because: {}", why)),
            }
        }
        Err(why) => Err(anyhow!("unable to update ledger file because: {}", why)),
    }
}

pub fn update_database_moi(
    mut transaction: TransactionContext,
) -> anyhow::Result<TransactionContext, Error> {
    let mut row: Row = Row { data: Vec::new() };
    row.data.push(DataType::BigInt(transaction.row_id));
    row.data.push(DataType::VarChar(
        transaction.db_name.len().clone() as u8,
        String::from(transaction.db_name.clone()),
    ));
    moihandler::add_row("C:\\MiaSql\\system\\database.moi", row)
        .expect("Unable to update database moi file");
    transaction.is_moi_file_updated = true;
    Ok(transaction)
}

pub fn update_system_table(
    mut transaction: TransactionContext,
    id: i64,
) -> anyhow::Result<(TransactionContext)> {
    let mut row: Row = Row { data: Vec::new() };
    row.data.push(DataType::BigInt(id));
    row.data.push(DataType::VarChar(
        transaction.db_name.len().clone() as u8,
        String::from(transaction.db_name.clone()),
    ));
    DbMem::insert_row("system", "database", row);
    transaction.is_system_table_updated = true;
    anyhow::Ok(transaction)
}

#[cfg(test)]
mod tests {
    use crate::command::createdatabase::parse;
    use crate::command::sqlcommands::SqlCommand;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    fn parse_sql(sql: &str) -> SqlCommand {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, sql).unwrap();
        parse(ast)
    }

    #[test]
    fn simple_create_database() {
        match parse_sql("CREATE DATABASE employee") {
            SqlCommand::CreateDatabase {
                command,
                database,
                comment,
            } => {
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
            SqlCommand::CreateDatabase {
                command, database, ..
            } => {
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
            SqlCommand::CreateDatabase {
                command, database, ..
            } => {
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
        let ast =
            sqlparser::parser::Parser::parse_sql(&dialect, "CREATE TABLE foo (id INT)").unwrap();
        let result = parse(ast);
        assert_eq!(result, SqlCommand::Undefined);
    }
}
