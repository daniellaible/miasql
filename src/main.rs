use log::info;
use tokio::net::TcpListener;
use crate::database::table::Table;
use crate::file::mtdreader::{read_mtd_file, MtdFile};
use crate::server::config::config::ConfigSingelton;
use crate::server::config::configreader;
use crate::server::dbmem::DbMem;

/// # Datamanipulations
/// In this module you find all the files that do datamanipulation in the RAM.
mod database {
    pub mod bptree;
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

mod file{
    pub mod mtdreader;
    pub mod moihandler;
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
    pub mod dbmem;
    pub mod server;
    pub mod queue;
}

mod test{
    pub mod test;
}

fn main() {
    env_logger::init();
    import_config();
    import_system_tables();
    let _ = run_server();
}

fn import_system_tables() {
    let database_mtd: MtdFile = read_mtd_file("C:\\MiaSql\\system\\database.mtd");
    let tables_mtd: MtdFile = read_mtd_file("C:\\MiaSql\\system\\tables.mtd");
    let user_mtd: MtdFile = read_mtd_file("C:\\MiaSql\\system\\user.mtd");

    let db_table:Table = file::moihandler::load_moi_file(&database_mtd).unwrap();
    let tables_table:Table = file::moihandler::load_moi_file(&tables_mtd).unwrap();
    let user_table: Table = file::moihandler::load_moi_file(&user_mtd).unwrap();

    DbMem::init();
    DbMem::add_table(db_table);
    DbMem::add_table(tables_table);
    DbMem::add_table(user_table);
    DbMem::print_tables();
}

fn import_config() {
    let config = ConfigSingelton::instance().lock().unwrap();
    configreader::load_config_file(config);
}

#[tokio::main]
async fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("listening on 127.0.0.1:7878");

    loop {
        info!("We are looping in the main function");
        let (stream, addr) = listener.accept().await?;
        println!("client connected: {addr}");

        tokio::spawn(async move {
            if let Err(e) = crate::server::server::handle_client(stream).await {
                eprintln!("client error: {e}");
            }
        });
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn calculate_stack_overflow_time(){
        let max_id = i64::MAX;
        let millis_in_year = 1000 * 60 * 60 * 24 * 365;
        println!("Wieviele Jahre braucht es für einen StackOverflow bei 1000/s : {}", max_id / millis_in_year);
    }
}

