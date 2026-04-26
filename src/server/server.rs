use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};

pub async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }
        let mut input = str::from_utf8(&buf[..n]).unwrap();
        input = input.trim();
        let command:String = input.to_uppercase();

        if command == "QUIT" || command == "BYE" {
            return Ok(());
        }
        else if command == "SHUTDOWN" {
            
            return Ok(());
        }
        else if command == "HELP" {
        
        }
        else if command == "SHOW DATABASES" {

        }
        else if command == "SHOW TABLES" {

        }else {
            
        }

        stream.write_all(&buf[..n]).await?;
    }
}

