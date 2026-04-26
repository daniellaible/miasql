use tokio::net::TcpListener;
use crate::database::Database;

mod bptree;
#[path= "database/table.rs"]
mod table;
#[path= "database/database.rs"]
mod database;

mod command {
    pub mod sqloperator;
    pub mod whereclause;
    pub mod select;
    pub mod insert;
    pub mod functions;
}
mod server{
    pub mod server;
}



fn main() {
    run_server();
}

#[tokio::main]
async fn run_server() -> std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("listening on 127.0.0.1:7878");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("client connected: {addr}");

        tokio::spawn(async move {
            if let Err(e) = crate::server::server::handle_client(stream).await {
                eprintln!("client error: {e}");
            }
        });
    }
}

