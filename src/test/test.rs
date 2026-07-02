
#[cfg(test)]
mod tests {
    use std::io;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpStream;

    #[test]
    fn test_connect() -> io::Result<()>{
        let stream = TcpStream::connect("127.0.0.1:7878")?;
        println!("Connected to {}", stream.peer_addr()?);

        let mut writer = stream.try_clone()?;
        let mut reader = BufReader::new(stream);

        let mut response = String::new();
        reader.read_line(&mut response)?;
        println!("{}", response);

        // Send a message
        writer.write_all(b"testuser")?;
        writer.flush()?;

        // Read the response
        let mut response = String::new();
        reader.read_line(&mut response)?;
        println!("{}", response);

        writer.write_all(b"use systems;")?;
        writer.flush()?;

        let mut response = String::new();
        reader.read_line(&mut response)?;
        println!("{}", response);

        println!("Server response: {}", response.trim());

        Ok(())

    }


}