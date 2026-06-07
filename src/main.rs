use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;
use tokio::net::TcpListener;
use crate::server::config::config::ConfigSingelton;
use crate::server::config::configreader;
use crate::server::queue::MasterQueueSingelton;

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

mod moi{
    pub mod filehandler;
}

mod ledger{
    pub mod writer;
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

fn load_system_db(){

}



#[tokio::main]
async fn run_server() -> std::io::Result<()> {
    //server::core::core::Core::start_core();
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

