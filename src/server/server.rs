use crate::command::command::Command;
use crate::command::insert::Insert;
use crate::command::select::Select;
use crate::command::sqlcommands::SqlCommand;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::command::delete::Delete;
use crate::command::sqlcommands::SqlCommand::DELETE;
use crate::command::update::Update;
use crate::database::database::Database;

pub async fn handle_client(mut stream: TcpStream, mut dbs: &Vec<Database> ) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }
        let mut input = str::from_utf8(&buf[..n]).unwrap();
        input = input.trim();
        let command: String = input.to_uppercase();

        if command == "QUIT" || command == "BYE" {
            return Ok(());
        } else if command == "SHUTDOWN" {
            return Ok(());
        } else if command == "HELP" {
        } else if command == "SHOW DATABASES" {
        } else if command == "SHOW TABLES" {
        } else {
            let mut sql:SqlCommand = SqlCommand::UNDEFINED;

            if command.starts_with("SELECT") {
                sql = Select::parse(String::from(command), dbs.clone());
                println!("{:?}", sql);

            } else if command.starts_with("INSERT") {
                sql = Insert::parse(String::from(command), dbs.clone());
                println!("{:?}", sql);

            } else if command.starts_with("UPDATE") {
                sql = Update::parse(String::from(command), dbs.clone());
                println!("{:?}", sql);

            } else if command.starts_with("DELETE") {
                sql = Delete::parse(String::from(command), dbs.clone());
                println!("{:?}", sql);

            } else if command.starts_with("CREATE") {
                println!("CREATE recognized");

            } else if command.starts_with("ALTER") {
                println!("ALTER recognized");

            } else if command.starts_with("DROP") {
                println!("DROP recognized");

            } else if command.starts_with("TRUNCATE") {
                println!("TRUNCATE recognized");

            } else if command.starts_with("GRANT") {
                println!("GRANT recognized");

            } else if command.starts_with("REVOKE") {
                println!("REVOKE recognized");

            } else if command.starts_with("USE") {
                println!("REVOKE recognized");
            }
        }

        stream.write_all(&buf[..n]).await?;
    }
}
