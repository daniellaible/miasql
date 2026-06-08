use std::fs::{OpenOptions};
use std::path::Path;
use std::io::{BufWriter, Write};
use crate::command::sqlcommands::SqlCommand;

pub fn write_ledger(transaction_id: u64) -> Result<bool, std::io::Error> {

    let masterqueue = crate::server::queue::MasterQueueSingelton::instance();
    let mut queue = masterqueue.queue.lock().unwrap();

    if let Some(transaction_protocol) = queue
        .iter_mut()
        .find(|tp| tp.transaction_id == transaction_id)
    {
        let command = transaction_protocol.command.clone();
        let line: String = to_printable_line(&command);
        append_to_file(&line);
        transaction_protocol.is_ledger_updated = false;
    }
    Ok(true)
}

fn append_to_file(line: &String){
    let output_file_path = Path::new("C:\\MiaSql\\ledger\\ledger.mldg");

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_file_path);

    let mut writer = BufWriter::new(file.unwrap());
    writeln!(writer, "{}", line);
    writer.flush();
}

fn to_printable_line(command: &SqlCommand) -> String {
    let line: String;
    let mut counter: u64 = 123456;
    match command {

        SqlCommand::CreateTable {table, columns, foreign_keys,  .. } => {
            line = format!( "{counter}; CREATE_TABLE; TABLE={}; COLUMNS={:?}; FOREIGN_KEYS={:?}", table, columns, foreign_keys );
            line.replace("\"", "")
        }
        SqlCommand::CreateDatabase {database, comment, .. } => {
            line = format!("{counter}; CREATE_DATABASE; DATABASE={}; COMMENT={:?}",database, comment );
            line.replace("\"", "")
        }
        SqlCommand::DropTable { table, ..} => {
            line = format!( "{counter}; DROP_TABLE; TABLE={}",  table);
            line.replace("\"", "")
        }
        SqlCommand::DropDatabase {database, .. } => {
            line = format!( "{counter}; DROP_DATABASE; DATABASE={}",  database);
            line.replace("\"", "")
        }
        SqlCommand::Delete {table, where_clause, .. } => {
            line = format!( "{counter}; DELETE; TABLE={}; WHERE={:?}",table,  where_clause);
            line.replace("\"", "")
        }
        SqlCommand::Truncate {tables, .. } => {
            line = format!( "{counter}; TRUNCATE; TABLES={:?}",tables);
            line.replace("\"", "")
        }
        SqlCommand::Update {table, sets, where_clause, .. } => {
            line = format!( "{counter}; UPDATE; TABLE={}; SETS={:?}; WHERE={:?}",table, sets, where_clause);
            line.replace("\"", "")
        }
        SqlCommand::Insert {table, columns, values, .. } => {
            line = format!( "{counter}; INSERT; TABLE={}; COLUMNS={:?}; VALUES={:?}",table, columns, values);
            line.replace("\"", "")
        }
        SqlCommand::AlterAddColumn {table, columns, .. } => {
            line = format!( "{counter}; ALTER_ADD_COLUMN; TABLE={}; COLUMNS={:?}",table, columns);
            line.replace("\"", "")
        }
        SqlCommand::AlterDropColumn { table, columns, .. } => {
            line = format!( "{counter}; ALTER_DROP_COLUMN; TABLE={}; COLUMNS={:?}",table, columns);
            line.replace("\"", "")
        }
        SqlCommand::AlterRenameColumn {table, old, new,.. } => {
            line = format!( "{counter}; ALTER_RENAME_COLUMN; TABLE={}; OLD={}; NEW={}",table, old, new);
            line.replace("\"", "")
        }
        SqlCommand::AlterModifyColumn {table, column, data_type, constraints,.. } => {
            line = format!( "{counter}; ALTER_RENAME_COLUMN; TABLE={}; COLUMN={}; DATATYPE={:?}; CONSTRAINTS={:?}",table, column, data_type, constraints);
            line.replace("\"", "")
        }
        SqlCommand::AlterTableRename {table, new_name,.. } => {
            line = format!( "{counter}; ALTER_RENAME_TABLE; TABLE={}; NEW_NAME={}",table, new_name);
            line.replace("\"", "")
        }
        _ => {
            return String::new();
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::ledger::writer::to_printable_line;
    use crate::server;

    #[test]
    fn test_printable_line_create_table() {
        let command: &str = "CREATE TABLE Persons ( PersonID BigInt PRIMARY KEY, LastName VarChar(255) NOT NULL, FirstName VarChar(255), Address VarChar(255), City VarChar(255));";
        let sqlcommand = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&sqlcommand);
        assert_eq!(line, "123456; CREATE_TABLE; TABLE=Persons; COLUMNS=[(PersonID, BIGINT, [PRIMARY_KEY]), (LastName, VARCHAR(255), [NOT_NULL]), (FirstName, VARCHAR(255), []), (Address, VARCHAR(255), []), (City, VARCHAR(255), [])]; FOREIGN_KEYS=[]");
    }

    #[test]
    fn test_printable_line_create_database() {
        let command: &str = "CREATE DATABASE employee;";
        let sqlcommand = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&sqlcommand);
        assert_eq!(line, "123456; CREATE_DATABASE; DATABASE=employee; COMMENT=");
    }

    #[test]
    fn test_printable_line_drop_database() {
        let command: &str = "DROP DATABASE employee";
        let sqlcommand = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&sqlcommand);
        assert_eq!(line, "123456; DROP_DATABASE; DATABASE=employee");
    }

    #[test]
    fn test_printable_line_delete() {
        let command: &str = "DELETE FROM employee WHERE id=1";
        let sqlcommand = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&sqlcommand);
        assert_eq!(line, "123456; DELETE; TABLE=employee; WHERE=WhereClause { column: id, operator: EQUAL, value: BigInt { x: 1 } }");
    }

    #[test]
    fn test_printable_line_truncate() {
        let command: &str = "TRUNCATE TABLE employee";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result);
        assert_eq!(line, "123456; TRUNCATE; TABLES=[employee]");
    }

    #[test]
    fn test_printable_line_update() {
        let command: &str = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt'WHERE CustomerID = 1;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result);
        assert_eq!(line, "123456; UPDATE; TABLE=Customers; SETS=[UpdateSet { column: ContactName, value: Alfred Schmidt }, UpdateSet { column: City, value: Frankfurt }]; WHERE=WhereClause { column: CustomerID, operator: EQUAL, value: BigInt { x: 1 } }");
    }

    #[test]
    fn test_printable_line_insert() {
        let command: &str = "INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country) VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway'), ('Greasy Burger', 'Per Olsen', 'Gateveien 15', 'Sandnes', '4306', 'Norway'),('Tasty Tee', 'Finn Egan', 'Streetroad 19B', 'Liverpool', 'L1 0AA', 'UK');";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result);
        assert_eq!(line, "123456; INSERT; TABLE=Customers; COLUMNS=[CustomerName, ContactName, Address, City, PostalCode, Country]; VALUES=[[Cardinal, Tom B. Erichsen, Skagen 21, Stavanger, 4006, Norway], [Greasy Burger, Per Olsen, Gateveien 15, Sandnes, 4306, Norway], [Tasty Tee, Finn Egan, Streetroad 19B, Liverpool, L1 0AA, UK]]");
    }

    #[test]
    fn test_printable_line_alter_add_column() {
        let command: &str = "ALTER TABLE Customers ADD Email varchar(255) NOT NULL;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result);
        assert_eq!(line, "123456; ALTER_ADD_COLUMN; TABLE=Customers; COLUMNS=[(Email, VarChar { x: , y: 255 }, [NOT_NULL])]");
    }

    #[test]
    fn test_printable_line_alter_drop_column() {
        let command: &str = "ALTER TABLE Customers DROP COLUMN Email;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result);
        assert_eq!(line, "123456; ALTER_DROP_COLUMN; TABLE=Customers; COLUMNS=[Email]");
    }

    #[test]
    fn test_printable_line_alter_rename_column() {
        let command: &str = "ALTER TABLE Workforce RENAME COLUMN Worker TO Employee;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result);
        assert_eq!(line, "123456; ALTER_RENAME_COLUMN; TABLE=Workforce; OLD=Worker; NEW=Employee");
    }

    #[test]
    fn test_printable_line_alter_modify_column() {
        let command: &str = "ALTER TABLE Customers MODIFY Email varchar(100) NOT NULL;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result);
        assert_eq!(line, "123456; ALTER_RENAME_COLUMN; TABLE=Customers; COLUMN=Email; DATATYPE=VarChar { x: , y: 100 }; CONSTRAINTS=[NOT_NULL]");
    }

    #[test]
    fn test_printable_line_alter_rename_table() {
        let command: &str = "ALTER TABLE Customers RENAME TO Clients;";
        let result = server::parser::tokenizer::tokeniz(command);
        let line = to_printable_line(&result);
        assert_eq!(line, "123456; ALTER_RENAME_TABLE; TABLE=Customers; NEW_NAME=Clients");
    }

}
