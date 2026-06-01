use crate::command;
use crate::command::sqlcommands::SqlCommand;
use crate::database::database::Database;
use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::any::Any;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::server::config::ConfigSingelton;

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
            command = command::alter::parse(alter.clone());
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
            command = command::insert::parse(insert.clone());
        }
        Statement::Query(query) => {
            command = command::select::parse(query.clone());
        }
        Statement::Update(update) => {
            command = command::update::parse(update.clone());
        }
        Statement::Delete(delete) => {
            command = command::delete::parse(delete.clone());
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
        let result = tokenizer(command);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_create_table() {
        let command: &str = "CREATE TABLE Persons ( PersonID BigInt PRIMARY KEY, LastName VarChar(255) NOT NULL, FirstName VarChar(255), Address VarChar(255), City VarChar(255));";
        let result = tokenizer(command);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_create_database() {
        let command: &str = "CREATE DATABASE employee";
        let result = tokenizer(command);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_drop_database() {
        let command: &str = "DROP DATABASE employee";
        let result = tokenizer(command);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_delete_row() {
        let command: &str = "DELETE FROM employee WHERE id=1";
        let result = tokenizer(command);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_all_rows() {
        let command: &str = "DELETE FROM employee WHERE id = 1";
        let result = tokenizer(command);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_truncate() {
        let command: &str = "TRUNCATE TABLE employee";
        let result = tokenizer(command);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_update() {
        let command: &str = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt'WHERE CustomerID = 1;";
        let result = tokenizer(command);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_insert() {
        let command3: &str = "INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country) VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway'), ('Greasy Burger', 'Per Olsen', 'Gateveien 15', 'Sandnes', '4306', 'Norway'),('Tasty Tee', 'Finn Egan', 'Streetroad 19B', 'Liverpool', 'L1 0AA', 'UK');";
        let result3  = tokenizer(command3);
        println!("result: {:?}", result3);
    }

    #[test]
    fn test_tokenizer_alter() {
        let command: &str = "ALTER TABLE Customers ADD Email varchar(255) NOT NULL;";
        let command2: &str = "ALTER TABLE Customers DROP COLUMN Email;";
        let command3: &str = "ALTER TABLE Workforce RENAME COLUMN Worker TO Employee;";
        let command4: &str = "ALTER TABLE Customers MODIFY Email varchar(100) NOT NULL;";
        let command5: &str = "ALTER TABLE Customers RENAME TO Clients;";
        tokenizer(command);
        tokenizer(command2);
        tokenizer(command3);
        tokenizer(command4);
        tokenizer(command5);
    }
}
