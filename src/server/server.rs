use crate::{server};
use crate::command::sqlcommands::SqlCommand;
use crate::database::database::Database;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::server::processor;

pub async fn handle_client(mut stream: TcpStream, mut dbs: &Vec<Database>) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }

        server::parser::tokenizer::tokeniz(std::str::from_utf8(&buf[..n]).unwrap());
        let mut input = str::from_utf8(&buf[..n]).unwrap();
        input = input.trim();

        let mut management_command = String::from(input);
        management_command = management_command.to_uppercase();

        if management_command == "QUIT" || management_command == "BYE" {
            return Ok(());
        } else if management_command == "SHOW DATABASES" {
            println!("SHOW DATABASES");
        } else if management_command.starts_with("USE ") {
            println!("User uses db");
        } else if management_command == "SHOW TABLES " {
            println!("Show tables");
        } else {
            let command: SqlCommand = server::parser::tokenizer::tokeniz(&*management_command);
            processor::processor::process_transaction(command);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::server;

    #[test]
    fn test_tokenizer_select() {
        let command: &str =
            "Select distinct avg(amount), name, lastname from employee where id='foo'";
        let result = server::parser::tokenizer::tokeniz(command);
        //println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_create_table() {
        let command: &str = "CREATE TABLE Persons ( PersonID BigInt PRIMARY KEY, LastName VarChar(255) NOT NULL, FirstName VarChar(255), Address VarChar(255), City VarChar(255));";
        let result = server::parser::tokenizer::tokeniz(command);
        //println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_create_database() {
        let command: &str = "CREATE DATABASE employee";
        let result = server::parser::tokenizer::tokeniz(command);
        //println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_drop_database() {
        let command: &str = "DROP DATABASE employee";
        let result = server::parser::tokenizer::tokeniz(command);
        //println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_delete_row() {
        let command: &str = "DELETE FROM employee WHERE id=1";
        let result = server::parser::tokenizer::tokeniz(command);
        //println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_all_rows() {
        let command: &str = "DELETE FROM employee WHERE id = 1";
        let result = server::parser::tokenizer::tokeniz(command);
        //println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_truncate() {
        let command: &str = "TRUNCATE TABLE employee";
        let result = server::parser::tokenizer::tokeniz(command);
        //println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_update() {
        let command: &str = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt'WHERE CustomerID = 1;";
        let result = server::parser::tokenizer::tokeniz(command);
        //println!("result: {:?}", result);
    }

    #[test]
    fn test_tokenizer_insert() {
        let command3: &str = "INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country) VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway'), ('Greasy Burger', 'Per Olsen', 'Gateveien 15', 'Sandnes', '4306', 'Norway'),('Tasty Tee', 'Finn Egan', 'Streetroad 19B', 'Liverpool', 'L1 0AA', 'UK');";
        let result3 = server::parser::tokenizer::tokeniz(command3);
    }

    #[test]
    fn test_tokenizer_alter() {
        let command: &str = "ALTER TABLE Customers ADD Email varchar(255) NOT NULL;";
        let command2: &str = "ALTER TABLE Customers DROP COLUMN Email;";
        let command3: &str = "ALTER TABLE Workforce RENAME COLUMN Worker TO Employee;";
        let command4: &str = "ALTER TABLE Customers MODIFY Email varchar(100) NOT NULL;";
        let command5: &str = "ALTER TABLE Customers RENAME TO Clients;";
        server::parser::tokenizer::tokeniz(command);
        server::parser::tokenizer::tokeniz(command2);
        server::parser::tokenizer::tokeniz(command3);
        server::parser::tokenizer::tokeniz(command4);
        server::parser::tokenizer::tokeniz(command5);
    }
}
