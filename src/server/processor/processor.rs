
use crate::command::sqlcommands::SqlCommand;
use crate::file::{moihandler, mtdhandler};
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionContext;
use crate::{command, file};
use anyhow::{anyhow, Error};
use log::info;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use tokio::net::TcpStream;
use uuid::Uuid;
use crate::command::createdatabase::create_database;
use crate::command::createtable::create_table;
use crate::command::showdatabases::show_databases;
use crate::command::showtables::show_tables;
use crate::database::bptree::Node;

pub static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn process_transaction(stream: &TcpStream, mut transaction: TransactionContext) -> anyhow::Result<TransactionContext> {
    info!("In the processor: {:?}", transaction.command);

    let transaction_id = get_transaction_counter();
    transaction.is_processing = true;
    transaction.transaction_id = transaction_id;
    load_table_to_ram(transaction.clone());

    match &transaction.command {
        SqlCommand::Select { .. } => {
            todo!()
        }
        SqlCommand::DropTable {..} => {
            todo!()
        }
        SqlCommand::DropDatabase {..} => {
            todo!()
        }
        SqlCommand::Delete {..} =>{
            todo!()
        }
        SqlCommand::Truncate {..} => {

        }
        SqlCommand::Update {..} => {

        }
        SqlCommand::Insert {..} => {

        }
        SqlCommand::AlterAddColumn { .. } => {

        }
        SqlCommand::AlterDropColumn { .. } => {

        }
        SqlCommand::AlterRenameColumn {..} => {

        }
        SqlCommand::AlterModifyColumn {..} => {

        }
        SqlCommand::AlterTableRename {..} => {

        }
        SqlCommand::ShowDatabases {..} => {
            let resultset = show_databases(transaction.clone());
        }
        SqlCommand::ShowTables { ..} => {
            let resultset = show_tables(transaction.clone());
        }

        SqlCommand::CreateDatabase {database, .. } => {
            let last_id = moihandler::get_max_id("C:\\MiaSql\\system\\database.moi");
            transaction.row_id = last_id + 1;
            let result  = create_database(transaction.clone(), database);
            match result{
                Ok(context) => {
                    if !context.error {
                        let line = format!("{} was created\n", database);
                        if let Err(e) = stream.try_write(line.as_bytes()){
                            eprintln!("write failed: {e}");
                        }
                        return Ok(context);
                    }else{
                        let line = format!("There was an errir while {} was created\n", database);
                        if let Err(e) = stream.try_write(line.as_bytes()){
                            eprintln!("write failed: {e} {context}");
                        }
                        return Ok(context);
                    }
                }
                Err(why) => {
                    panic!("Something strange happend here {:?}", why);
                }
            }
        }
        SqlCommand::CreateTable { .. } => {
            let last_id = moihandler::get_max_id("C:\\MiaSql\\system\\tables.moi");
            transaction.row_id = last_id + 1;
            let uuid = Uuid::new_v4();
            transaction.table_uuid = uuid;
            //Todo Error Matching
            transaction = create_table(transaction.clone()).unwrap();
        }
        _ => {}
    }


    //update system table if necessary
    let trans_clone_sys_tab = transaction.clone();

    let system_table_update_result = update_system_table(trans_clone_sys_tab);
    match system_table_update_result {
        Ok(_) => {
            transaction.is_system_table_updated = true;
            info!("System Table updated");
        }
        Err(why) => {
            transaction.is_system_table_updated = false;
            transaction.error = true;
            return Err(anyhow!("unable to update system table because:{}", why));
        }
    }

    //update/create mtd file
    let trans_clone_mtd_file = transaction.clone();
    let mtd_file_result = update_mtd_file(trans_clone_mtd_file);
    match mtd_file_result {
        Ok(_) => {
            transaction.is_mtd_file_updated = true;
            info!("MTD file  updated");
        }
        Err(why) => {
            transaction.is_mtd_file_updated = false;
            transaction.error = true;
            return Err(anyhow!("unable to update mtd file because:{}", why));
        }
    }

    let trans_select_show = transaction.clone();
    select_and_show(&stream, trans_select_show);


    //update moi file
    let trans_clone_moi_file = transaction.clone();

        let moi_file_result = moihandler::update(trans_clone_moi_file);
        match moi_file_result {
            Ok(_) => {
                transaction.is_moi_file_updated = true;
                info!("Moi file  updated");
            }
            Err(why) => {
                transaction.is_moi_file_updated = false;
                transaction.error = true;
                return Err(anyhow!("unable to update moi file because:{}", why));
            }
        }

    Ok(transaction)
}

fn select_and_show(stream: &TcpStream, tc: TransactionContext)  {
    match tc.command {
        SqlCommand::Select { .. } => {}
        SqlCommand::ShowDatabases { .. } => {
            if let Some(table_arc) = DbMem::find_table("system", "database") {
                let table_guard = table_arc.lock().unwrap();
                let tree = &table_guard.tree;

                let mut cur = Some(tree.leftmost_leaf(tree.root.clone()));

                while let Some(node_arc) = cur {
                    let (rows_to_send, next_leaf) = {
                        let node_guard = node_arc.lock().unwrap();
                        let Node::Leaf(leaf) = &*node_guard else {
                            unreachable!("leftmost_leaf/next chain must be leaves");
                        };
                        (leaf.values.clone(), leaf.next.clone())
                    };

                    for row in rows_to_send {
                        let line = format!("{:?}\n", row);
                        if let Err(e) = stream.try_write(line.as_bytes()){
                            eprintln!("write failed: {e}");
                            return;
                        }
                    }
                    cur = next_leaf;
                }
            }
        }
        SqlCommand::ShowTables { .. } => {}
        _ => {}
    }
}

fn update_mtd_file(mut tp: TransactionContext) -> anyhow::Result<()> {
    match &tp.command {
        SqlCommand::CreateTable {
            table,
            columns,
            foreign_keys,
            ..
        } => {
            match mtdhandler::new_mtd_file(
                &tp.db_name,
                table,
                columns,
                foreign_keys,
                tp.table_uuid.clone(),
            ) {
                Ok(_) => tp.is_mtd_file_updated = true,
                Err(_) => tp.error = true,
            }
        }
        _ => {}
    }
    Ok(())
}

fn load_table_to_ram(tp: TransactionContext) {
    for i in 0..tp.table_names.len() {
        let is_table_loaded = DbMem::is_table_loaded(tp.db_name.clone(), tp.table_names[i].clone());

        if is_table_loaded == false {
            println!("You need to load the table that isn't loaded");
            //todo: DbMem::load_table()
        }
    }
}

fn update_system_table(mut tp: TransactionContext) -> anyhow::Result<TransactionContext> {
    match &tp.command {
        SqlCommand::CreateTable { table, .. } => {
            let result = command::createtable::update_system_table(
                tp.row_id,
                Arc::from(tp.db_name.as_str()),
                Arc::from(table.as_str()),
            );
            match result {
                Ok(_) => tp.is_system_table_updated = true,
                Err(_) => {
                    tp.error = true;
                }
            }
        }
        _ => { }
    }

    Ok(tp)
}

fn get_transaction_counter() -> u64 {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
