use tokio::net::TcpListener;

mod bptree;

mod database {
    pub mod database;
    pub mod datatype;
    pub mod table;
}

mod command {
    pub mod sqloperator;
    pub mod whereclause;
    pub mod select;
    pub mod insert;
    pub mod functions;
    pub mod command;
    pub mod sqlcommands;
    pub mod permissions;
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

