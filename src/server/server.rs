use crate::command::sqlcommands::SqlCommand;
use crate::server;
use crate::server::queue::{MasterQueueSingelton, TransactionProtocol};
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];

    let mut is_logged_in = false;
    let mut is_use_command = false;
    let mut username = String::from("");
    let mut db_used = String::from("");
    loop {
        if !is_logged_in {
            let login_prompt = String::from("login:");
            stream
                .write_all((&login_prompt).as_ref())
                .await
                .expect("Unable to write login prompts");

            let n = stream.read(&mut buf).await?;
            if n == 0 {
                return Ok(());
            }

            username = std::str::from_utf8(&buf[..n]).unwrap().to_string();
            username = username.replace("\r\n", "");
            is_logged_in = true;

        } else {
            let n = stream.read(&mut buf).await?;
            if n == 0 {
                return Ok(());
            }
            let mut answer: String = String::new();
            server::parser::tokenizer::tokeniz(std::str::from_utf8(&buf[..n]).unwrap());

            let mut input = match str::from_utf8(&buf[..n]) {
                Ok(x) => {
                    x.to_string()
                },
                Err(_) => {
                    error!("Unable to parse the buffer into a str");
                    String::new()
                }
            };

            input = input.trim().to_string();
            input = input.replace(";", "");
            let sql_command: SqlCommand = parse_incomming(&input);

            let command_string = match sql_command.clone() {
                SqlCommand::Select { command, .. } => command,
                SqlCommand::CreateTable { command, .. } => command,
                SqlCommand::CreateDatabase { command, .. } => command,
                SqlCommand::DropTable { command, .. } => command,
                SqlCommand::DropDatabase { command, .. } => command,
                SqlCommand::Delete { command, .. } => command,
                SqlCommand::Truncate { command, .. } => command,
                SqlCommand::Update { command, .. } => command,
                SqlCommand::Insert { command, .. } => command,
                SqlCommand::AlterAddColumn { command, .. } => command,
                SqlCommand::AlterDropColumn { command, .. } => command,
                SqlCommand::AlterRenameColumn { command, .. } => command,
                SqlCommand::AlterModifyColumn { command, .. } => command,
                SqlCommand::AlterTableRename { command, .. } => command,
                SqlCommand::Use { command, .. } => command,
                SqlCommand::Quit { command, .. } => command,
                SqlCommand::ShowDatabases { command, .. } => command,
                SqlCommand::ShowTables { command, .. } => command,
                SqlCommand::Undefined {} => String::new(),
            };

            let table_names:Vec<String> = match sql_command.clone(){
                SqlCommand::Select {table, .. } => {
                    let mut tv = Vec::new();
                    tv.push(table);
                    tv
                }
                SqlCommand::DropTable {table, .. } => {
                    let mut tv = Vec::new();
                    tv.push(table);
                    tv
                }
                SqlCommand::Delete {table, .. } => {
                    let mut tv = Vec::new();
                    tv.push(table);
                    tv
                }
                SqlCommand::Truncate {tables, .. } => {
                    tables
                }
                SqlCommand::Update {table, .. } => {
                    let mut tv = Vec::new();
                    tv.push(table);
                    tv
                }
                SqlCommand::Insert {table, .. } => {
                    let mut tv = Vec::new();
                    tv.push(table);
                    tv
                }
                SqlCommand::AlterAddColumn {table, .. } => {
                    let mut tv = Vec::new();
                    tv.push(table);
                    tv
                }
                SqlCommand::AlterDropColumn {table, .. } => {
                    let mut tv = Vec::new();
                    tv.push(table);
                    tv
                }
                SqlCommand::AlterRenameColumn {table, .. } => {
                    let mut tv = Vec::new();
                    tv.push(table);
                    tv
                }
                SqlCommand::AlterModifyColumn {table, .. } => {
                    let mut tv = Vec::new();
                    tv.push(table);
                    tv
                }
                SqlCommand::AlterTableRename {table, .. } => {
                    let mut tv = Vec::new();
                    tv.push(table);
                    tv
                }
                _ => {Vec::new()}
            };
            
            is_use_command = false;
            match sql_command.clone(){
                SqlCommand::Use{database, ..} => {
                    answer = format!("using:  {database} \r\n");
                    is_use_command = true;
                    db_used = database;
                },
                _ => {}
            };

            match sql_command{
                SqlCommand::Undefined => {answer = format!("I didn't understand your last command \r\n")},
                _ => {}
            };

            let mut transaction_result: Option<TransactionProtocol> = None;
            if !db_used.is_empty() && !is_use_command {
                if sql_command != SqlCommand::Undefined {
                    let transaction: TransactionProtocol = TransactionProtocol {
                        db_name: db_used.clone(),
                        row_id: -1,
                        table_names: table_names,
                        is_processing: false,
                        is_finished: false,
                        transaction_id: 0,
                        command: sql_command.clone(),
                        is_moi_file_updated: false,
                        is_ledger_updated: false,
                        is_btree_updated: false,
                        is_cluster_updated: false,
                        is_shard_updated: false,
                        is_error_detected: false,
                        is_system_table_updated: false,
                        error: false,
                    };
                    transaction_result = MasterQueueSingelton.add(transaction);
                    match transaction_result{
                        None => error!("something went clearly wrong with your transaction"),
                        Some(tp) => info!("{}", tp)
                    }

                }else{
                    answer = format!("I don't understand: {command_string}");
                }
                
            }else{
                if !is_use_command {
                    answer = format!("Please tell me which database to use!");
                }
            }

            //write transaction_result to user
            stream.write_all((&answer).as_ref()).await.expect("Something wrong with the in-/output stream");
        }
    }
}

pub fn parse_incomming(incomming: &str) -> SqlCommand {
    let mut management_command = String::from(incomming);
    management_command = management_command.to_uppercase();

    if management_command == "QUIT" || management_command == "BYE" {
        SqlCommand::Quit {
            command: String::from("QUIT"),
        }
    } else if management_command == "SHOW DATABASES" {
        SqlCommand::ShowDatabases {
            command: String::from("SHOW DATABASE"),
        }
    } else if management_command.starts_with("USE ") {
        let splits = management_command.split(" ");
        let mut db_name = splits.collect::<Vec<&str>>()[1];
        db_name = db_name.trim();
        SqlCommand::Use {
            command: String::from("USE"),
            database: db_name.to_string(),
        }
    } else if management_command == "SHOW TABLES " {
        SqlCommand::ShowTables {
            command: String::from("SHOW TABLES"),
            database: "".to_string(),
        }
    } else {
        let command = server::parser::tokenizer::tokeniz(&management_command);
        info!("tokenized command: {:?}", command);
        command
    }
}

#[cfg(test)]
mod tests {
    use crate::server;

    #[test]
    fn test_tokenizer_select() {
        let command: &str =
            "Select distinct avg(amount), name, lastname from employee where id='foo'";
        server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_create_table() {
        let command: &str = "CREATE TABLE Persons ( PersonID BigInt PRIMARY KEY, LastName VarChar(255) NOT NULL, FirstName VarChar(255), Address VarChar(255), City VarChar(255));";
        server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_create_database() {
        let command: &str = "CREATE DATABASE employee;";
        server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_drop_database() {
        let command: &str = "DROP DATABASE employee;";
        server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_delete_row() {
        let command: &str = "DELETE FROM employee WHERE id=1;";
        server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_all_rows() {
        let command: &str = "DELETE FROM employee WHERE id = 1";
        server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_truncate() {
        let command: &str = "TRUNCATE TABLE employee;";
        server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_update() {
        let command: &str = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt'WHERE CustomerID = 1;";
        server::parser::tokenizer::tokeniz(command);
    }

    #[test]
    fn test_tokenizer_insert() {
        let command3: &str = "INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country) VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway'), ('Greasy Burger', 'Per Olsen', 'Gateveien 15', 'Sandnes', '4306', 'Norway'),('Tasty Tee', 'Finn Egan', 'Streetroad 19B', 'Liverpool', 'L1 0AA', 'UK');";
        server::parser::tokenizer::tokeniz(command3);
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
