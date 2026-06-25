use log::info;
use crate::server;
use crate::server::queue::{MasterQueueSingelton, TransactionProtocol};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::command::sqlcommands::SqlCommand;

pub async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }

        server::parser::tokenizer::tokeniz(std::str::from_utf8(&buf[..n]).unwrap());
        let mut input = str::from_utf8(&buf[..n]).unwrap();
        input = input.trim();

        parse_incomming(&input, &stream);
        let answer: String = String::from("This is the life");
        stream.write_all((&answer).as_ref()).await.expect("Doof");
    }
}

pub fn parse_incomming(incomming: &str, stream: &TcpStream) {
    let mut management_command = String::from(incomming);
    management_command = management_command.to_uppercase();

    if management_command == "QUIT" || management_command == "BYE" {
        return;
    } else if management_command == "SHOW DATABASES" {

        info!("SHOW DATABASES");
    } else if management_command.starts_with("USE ") {
        let splits =  management_command.split(" ");
        let mut db_name = splits.collect::<Vec<&str>>()[1];
        db_name = db_name.trim();
        info!("User uses db: {}", db_name);
    } else if management_command == "SHOW TABLES " {
        info!("Show tables");
    } else {
        let command = server::parser::tokenizer::tokeniz(&*management_command);
        info!("tokenized command: {:?}", command);
        if command != SqlCommand::Undefined {
            let transaction: TransactionProtocol = TransactionProtocol {
                is_processing: false,
                is_finished: false,
                transaction_id: 1000,
                command: server::parser::tokenizer::tokeniz(&*management_command),
                is_moi_file_updated: false,
                is_ledger_updated: false,
                is_btree_updated: false,
                is_cluster_updated: false,
                is_shard_updated: false,
                is_error_detected: false,
                error_msg: None,
            };
            MasterQueueSingelton.add(transaction);
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
    }

    #[test]
    fn test_tokenizer_create_table() {
        let command: &str = "CREATE TABLE Persons ( PersonID BigInt PRIMARY KEY, LastName VarChar(255) NOT NULL, FirstName VarChar(255), Address VarChar(255), City VarChar(255));";
        let result = server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_create_database() {
        let command: &str = "CREATE DATABASE employee;";
        let result = server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_drop_database() {
        let command: &str = "DROP DATABASE employee;";
        let result = server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_delete_row() {
        let command: &str = "DELETE FROM employee WHERE id=1;";
        let result = server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_all_rows() {
        let command: &str = "DELETE FROM employee WHERE id = 1";
        let result = server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_truncate() {
        let command: &str = "TRUNCATE TABLE employee;";
        let result = server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_update() {
        let command: &str = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt'WHERE CustomerID = 1;";
        let result = server::parser::tokenizer::tokeniz(command);
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
