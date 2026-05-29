use crate::command;
use crate::command::sqlcommands::SqlCommand;
use crate::database::database::Database;
use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::any::Any;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_client(mut stream: TcpStream, mut dbs: &Vec<Database>) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }

        let mut input = str::from_utf8(&buf[..n]).unwrap();
        input = input.trim();

        let mut management_command = String::from(input.clone());
        management_command = management_command.to_uppercase();
        if management_command == "QUIT" || management_command == "BYE" {
            return Ok(());
        } else if management_command == "HELP" {
            //print to console
        } else if management_command == "SHOW DATABASES" {
            println!("SHOW DATABASES");
        } else if management_command == "USE " {
            println!("User uses db");
        } else if management_command == "SHOW TABLES " {
            println!("Show tables");
        } else {
            let command: SqlCommand = tokenizer(input);
        }
        stream.write_all(&buf[..n]).await?;
    }
}

fn tokenizer(stmt: &str) -> SqlCommand {
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, stmt).unwrap();
    println!("{:#?}", ast[0].clone());
    let mut command: SqlCommand = SqlCommand::UNDEFINED;

    match ast[0].clone() {
        Statement::AlterTable(alter) => {
            println!("table name: {}", alter.name);
            println!("if_exists: {}", alter.if_exists);
            println!("only: {}", alter.only);
            println!("operations: {:?}", alter.operations);
            println!("location: {:?}", alter.location);
            println!("on_cluster: {:?}", alter.on_cluster);
            println!("table_type: {:?}", alter.table_type);
            println!("end_token: {:?}", alter.end_token);
            command = SqlCommand::UNDEFINED;
        }
        Statement::CreateTable(create) => {
            command = command::createtable::parse(create.clone());
        }
        Statement::Truncate(truncate) => {
            command = command::truncate::parse(truncate);
        }
        Statement::CreateDatabase { .. } => {
            command = command::createdatabase::parse(ast);
        }
        Statement::Drop { .. } => {
            command = command::drop::parse(ast);
        }
        Statement::Insert(insert) => {
            command = SqlCommand::UNDEFINED;
            println!("table: {:?}", insert.table);
        }
        Statement::Query(query) => {
            command = command::select::parse(query.clone());
        }
        Statement::Update(update) => {
            command = command::update::parse(update.clone());
            println!("table: {:?}", update.table);
        }
        Statement::Delete(delete) => {
            command = command::delete::parse(delete.clone());
            println!("delete: {:?}", delete);
        }
        _ => println!("other statement"),
    }
    command
}

#[cfg(test)]
mod tests {
    use crate::server::server::tokenizer;

    #[test]
    fn test_tokenizer_select() {
        let command: &str =
            "Select distinct avg(amount), name, lastname from employee where id='foo'";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_create_table() {
        let command: &str = "CREATE TABLE Persons ( PersonID BigInt PRIMARY KEY, LastName VarChar(255) NOT NULL, FirstName VarChar(255), Address VarChar(255), City VarChar(255));";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_create_database() {
        let command: &str = "CREATE DATABASE employee";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_drop_database() {
        let command: &str = "DROP DATABASE employee";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_delete_row() {
        let command: &str = "DELETE FROM employee WHERE id=1";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_all_rows() {
        let command: &str = "DELETE FROM employee WHERE id = 1";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_truncate() {
        let command: &str = "TRUNCATE TABLE employee";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_update() {
        let command: &str = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt'WHERE CustomerID = 1;";
        tokenizer(command);
    }
}
