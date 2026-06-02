use crate::database::database::Database;
use std::io::BufRead;
use tokio::net::TcpListener;
use crate::server::config::config::ConfigSingelton;
use crate::server::config::configreader;

/// # Datamanipulations
/// In this module you find all the files that do datamanipulation in the RAM.
mod database {
    mod bptree;
    pub mod database;
    pub mod datatype;
    pub mod table;
}

/// # Parsing modules
mod command {
    pub mod alter;
    pub mod constraint;
    /// This module handles the tokenization of the CREATE DATABASE command
    pub mod createdatabase;
    pub mod createtable;
    pub mod delete;
    /// This module handles the tokenization of the DROP DATABASE | TABLE  command
    pub mod drop;
    pub mod insert;
    pub mod permissions;
    pub mod select;
    pub mod sqlcommands;
    pub mod sqloperator;
    pub mod truncate;
    pub mod update;
    pub mod whereclause;
}

mod server {
    pub mod processor{
        pub mod processor;
    }
    pub mod config{
        pub mod config;
        pub mod configreader;
    }
    pub mod parser{
        pub mod tokenizer;
    }

    pub mod server;
    pub mod queue;
}

fn main() {
    import_config();
    let _ = run_server();
}

fn import_config() {
    let config = ConfigSingelton::instance().lock().unwrap();
    configreader::load_config_file(config);
}

#[tokio::main]
async fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("listening on 127.0.0.1:7878");

    let mut all_databases: Vec<Database> = Vec::new();

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("client connected: {addr}");

        all_databases = load_all_dbs();

        tokio::spawn(async move {
            if let Err(e) = crate::server::server::handle_client(stream, &all_databases).await {
                eprintln!("client error: {e}");
            }
        });
    }
}

fn load_all_dbs() -> Vec<Database> {
    Vec::new()
}
