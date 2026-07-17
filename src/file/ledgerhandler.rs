use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{BufWriter, Write};
use std::time::SystemTime;

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
    let mut counter: u64 = 0;
    if !output_file_path.exists() {
        match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", &path, why),
            Ok(file) => file,
        };
    }

    let file = OpenOptions::new()
        .append(true)
        .open(output_file_path);

    let mut writer = BufWriter::new(file.unwrap());

    let printable = to_printable_line(command, db_name, counter, user);
    writer.write_all((&printable).as_ref()).expect("unable to write to ledger");
    writer.flush();
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
    use crate::file::ledgerhandler::to_printable_line;
    use crate::server;

    #[test]
    fn test_printable_line_create_table() {
        let command: &str = "CREATE TABLE Persons ( PersonID BigInt PRIMARY KEY, LastName VarChar(255) NOT NULL, FirstName VarChar(255), Address VarChar(255), City VarChar(255));";
        let sqlcommand = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&sqlcommand, "testdb", 123456, "testuser");
        assert_eq!(line, "123456; CREATE_TABLE; TABLE=Persons; COLUMNS=[(PersonID, BIGINT, [PrimaryKey]), (LastName, VARCHAR(255), [NotNull]), (FirstName, VARCHAR(255), []), (Address, VARCHAR(255), []), (City, VARCHAR(255), [])]; FOREIGN_KEYS=[]");
    }

    #[test]
    fn test_printable_line_create_database() {
        let command: &str = "CREATE DATABASE employee;";
        let sqlcommand = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&sqlcommand,"testdb",123456, "testuser");
        assert_eq!(line, "123456; CREATE_DATABASE; DATABASE=employee; COMMENT=");
    }

    #[test]
    fn test_printable_line_drop_database() {
        let command: &str = "DROP DATABASE employee";
        let sqlcommand = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&sqlcommand, "testdb", 123456, "testuser");
        assert_eq!(line, "123456; DROP_DATABASE; DATABASE=employee");
    }

    #[test]
    fn test_printable_line_delete() {
        let command: &str = "DELETE FROM employee WHERE id=1";
        let sqlcommand = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&sqlcommand, "testdb",123456, "testuser");
        assert_eq!(line, "123456; DELETE; TABLE=employee; WHERE=WhereClause { column: id, operator: EQUAL, value: BigInt(1) }");
    }

    #[test]
    fn test_printable_line_truncate() {
        let command: &str = "TRUNCATE TABLE employee";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result,"testdb", 123456, "testuser");
        assert_eq!(line, "123456; TRUNCATE; TABLES=[employee]");
    }

    #[test]
    fn test_printable_line_update() {
        let command: &str = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt'WHERE CustomerID = 1;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result,"testdb", 123456, "testuser");
        assert_eq!(line, "123456; UPDATE; TABLE=Customers; SETS=[UpdateSet { column: ContactName, value: Alfred Schmidt }, UpdateSet { column: City, value: Frankfurt }]; WHERE=WhereClause { column: CustomerID, operator: EQUAL, value: BigInt(1) }");
    }

    #[test]
    fn test_printable_line_insert() {
        let command: &str = "INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country) VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway'), ('Greasy Burger', 'Per Olsen', 'Gateveien 15', 'Sandnes', '4306', 'Norway'),('Tasty Tee', 'Finn Egan', 'Streetroad 19B', 'Liverpool', 'L1 0AA', 'UK');";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result,"testdb", 123456, "testuser");
        assert_eq!(line, "123456; INSERT; TABLE=Customers; COLUMNS=[CustomerName, ContactName, Address, City, PostalCode, Country]; VALUES=[[Cardinal, Tom B. Erichsen, Skagen 21, Stavanger, 4006, Norway], [Greasy Burger, Per Olsen, Gateveien 15, Sandnes, 4306, Norway], [Tasty Tee, Finn Egan, Streetroad 19B, Liverpool, L1 0AA, UK]]");
    }

    #[test]
    fn test_printable_line_alter_add_column() {
        let command: &str = "ALTER TABLE Customers ADD Email varchar(255) NOT NULL;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result,"testdb", 123456 , "testuser");
        assert_eq!(line, "123456; ALTER_ADD_COLUMN; TABLE=Customers; COLUMNS=[(Email, VarChar(255, ), [NotNull])]");
    }

    #[test]
    fn test_printable_line_alter_drop_column() {
        let command: &str = "ALTER TABLE Customers DROP COLUMN Email;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result, "testdb",123456, "testuser");
        assert_eq!(line, "123456; ALTER_DROP_COLUMN; TABLE=Customers; COLUMNS=[Email]");
    }

    #[test]
    fn test_printable_line_alter_rename_column() {
        let command: &str = "ALTER TABLE Workforce RENAME COLUMN Worker TO Employee;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result, "testdb",123456, "testuser");
        assert_eq!(line, "123456; ALTER_RENAME_COLUMN; TABLE=Workforce; OLD=Worker; NEW=Employee");
    }

    #[test]
    fn test_printable_line_alter_modify_column() {
        let command: &str = "ALTER TABLE Customers MODIFY Email varchar(100) NOT NULL;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result, "testdb",123456, "testuser");
        assert_eq!(line, "123456; ALTER_RENAME_COLUMN; TABLE=Customers; COLUMN=Email; DATATYPE=VarChar(100, ); CONSTRAINTS=[NotNull]");
    }

    #[test]
    fn test_printable_line_alter_rename_table() {
        let command: &str = "ALTER TABLE Customers RENAME TO Clients;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result, "testdb",123456, "testuser");
        assert_eq!(line, "123456; ALTER_RENAME_TABLE; TABLE=Customers; NEW_NAME=Clients");
    }
}
