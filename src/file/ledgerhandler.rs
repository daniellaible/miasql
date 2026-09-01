use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{BufWriter, Write};
use std::time::SystemTime;
use anyhow::anyhow;
use crate::command::sqlcommands::SqlCommand;


pub fn append_to_file(user: &str, command: &SqlCommand, given_db:&str) -> anyhow::Result<()>{
    let db_name = match command {
        SqlCommand::CreateDatabase {database, ..} =>{
            database
        }
        _ => {given_db}
    };
    let path = "C:\\MiaSql\\ledger\\".to_owned() + db_name + ".mldg";
    let output_file_path = Path::new(&path);

    match command {
        SqlCommand::CreateDatabase {..} => {
            if output_file_path.exists() {
                return Err(anyhow!("There is already a database ledger with this name"));
            }
        }
        _ => {}
    };

    let counter: u64 = 0;
    if !output_file_path.exists() {
        match File::create(&path) {
            Err(why) => {
                return Err(anyhow!("couldn't create {}: {}", &path, why));
            },
            Ok(file) => file,
        };
    }

    let file = OpenOptions::new()
        .append(true)
        .open(output_file_path);

    let mut writer = BufWriter::new(file.unwrap());

    let printable = to_printable_line(command, db_name, counter, user);
    writer.write_all((&printable).as_ref()).expect("unable to write to ledger");
    let _ = writer.flush();
    Ok(())
}


fn to_printable_line(command: &SqlCommand, database: &str, counter: u64, user: &str) -> String {
    let timestamp = SystemTime::now();
    let line: String;
        match command {

        SqlCommand::CreateTable {table, columns, foreign_keys,  .. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; CREATE_TABLE; TABLE={}; COLUMNS={:?}; FOREIGN_KEYS={:?} \n",timestamp, table, columns, foreign_keys );
            line.replace("\"", "")
        }
        SqlCommand::CreateDatabase {database, comment, .. } => {
            line = format!("{:?}; {counter}; {user}; {database}; CREATE_DATABASE; DATABASE={}; COMMENT={:?}\n",timestamp,database, comment );
            line.replace("\"", "")
        }
        SqlCommand::DropTable { table, ..} => {
            line = format!( "{:?}; {counter}; {user}; {database}; DROP_TABLE; TABLE={}\n",timestamp, table);
            line.replace("\"", "")
        }
        SqlCommand::DropDatabase {database, .. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; DROP_DATABASE; DATABASE={}\n",timestamp, database);
            line.replace("\"", "")
        }
        SqlCommand::Delete {table, where_clause, .. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; DELETE; TABLE={}; WHERE={:?}\n",timestamp, table, where_clause);
            line.replace("\"", "")
        }
        SqlCommand::Truncate {tables, .. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; TRUNCATE; TABLES={:?}\n",timestamp, tables);
            line.replace("\"", "")
        }
        SqlCommand::Update {table, sets, where_clause, .. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; UPDATE; TABLE={}; SETS={:?}; WHERE={:?}\n",timestamp, table, sets, where_clause);
            line.replace("\"", "")
        }
        SqlCommand::Insert {table, columns, values, .. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; INSERT; TABLE={}; COLUMNS={:?}; VALUES={:?}\n",timestamp, table, columns, values);
            line.replace("\"", "")
        }
        SqlCommand::AlterAddColumn {table, columns, .. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; ALTER_ADD_COLUMN; TABLE={}; COLUMNS={:?}\n",timestamp, table, columns);
            line.replace("\"", "")
        }
        SqlCommand::AlterDropColumn { table, columns, .. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; ALTER_DROP_COLUMN; TABLE={}; COLUMNS={:?}\n",timestamp, table, columns);
            line.replace("\"", "")
        }
        SqlCommand::AlterRenameColumn {table, old, new,.. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; ALTER_RENAME_COLUMN; TABLE={}; OLD={}; NEW={}\n",timestamp, table, old, new);
            line.replace("\"", "")
        }
        SqlCommand::AlterModifyColumn {table, column, data_type, constraints,.. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; ALTER_RENAME_COLUMN; TABLE={}; COLUMN={}; DATATYPE={:?}; CONSTRAINTS={:?}\n",timestamp, table, column, data_type, constraints);
            line.replace("\"", "")
        }
        SqlCommand::AlterTableRename {table, new_name,.. } => {
            line = format!( "{:?}; {counter}; {user}; {database}; ALTER_RENAME_TABLE; TABLE={}; NEW_NAME={}\n",timestamp, table, new_name);
            line.replace("\"", "")
        }
        _ => {
            String::new()
        }
    }
}


#[cfg(test)]
mod tests {

}
