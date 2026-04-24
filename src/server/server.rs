use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};

pub async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }
        stream.write_all(&buf[..n]).await?;
    }
}

