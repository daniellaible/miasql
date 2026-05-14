use regex::Regex;
use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::command::whereclause::WhereClause;
use crate::database::database::Database;
use crate::database::datatype::DataType;

#[derive(Debug)]
pub struct Update {
    table_name: String,
    columns: Vec<String>,
    values: Vec<Vec<DataType>>,
    where_clause: WhereClause,
}

impl Command for Update {
    fn parse(stmt: String, dbs: Vec<Database>) -> SqlCommand {
        let expr = stmt.to_uppercase();
        let table: String = get_table(&expr);
        let columns_value_pairs: Vec<String> = get_column_value_pairs(&expr);
        let columns:Vec<String> = get_columns(columns_value_pairs.clone());
        let values:Vec<String> = get_values(columns_value_pairs.clone());
        let clause: WhereClause = WhereClause::parse(&expr);
        println!("{:?}", clause);
        SqlCommand::UPDATE {
            command: String::from("UPDATE"),
            table,
            columns: vec![],
            values: vec![],
            where_clause: WhereClause::default(),
        }
    }
}

fn get_values(p0: Vec<String>) -> Vec<String> {
    todo!()
}

fn get_columns(p0: Vec<String>) -> Vec<String> {
    todo!()
}

fn get_column_value_pairs(mut stmt: &String) -> Vec<String> {

    if stmt.contains("WHERE") {
        let regex = Regex::new(r"(?i)SET\s+(.*?)\s+WHERE").unwrap();
        let captures = regex.captures(&stmt).unwrap();
        let column_value_pairs = captures.get(1).unwrap().as_str();
        let column_values:Vec<&str> = column_value_pairs.split(",").collect();
        let result: Vec<String> = column_values.iter().map(|s| s.to_string()).collect();
        result
    }else{
        let mut cleaned_stmt = String::from(stmt);
        if stmt.contains(";"){
            cleaned_stmt = stmt.replace(";", "" );
        }
        let regex = Regex::new(r"(?i)SET\s+(.+)").unwrap();
        let captures = regex.captures(&cleaned_stmt).unwrap();
        let column_value_pairs = captures.get(1).unwrap().as_str();
        let column_values:Vec<&str> = column_value_pairs.split(",").collect();
        let result: Vec<String> = column_values.iter().map(|s| s.to_string()).collect();
        result
    }
}

fn get_table(stmt: &String) -> String {
    let regex_table = Regex::new(r"(?i)UPDATE\s+(.*?)\s+SET").unwrap();
    let captures_table = regex_table.captures(stmt).unwrap();
    let table = captures_table.get(1).unwrap().as_str();
    println!("table: {:?}", table);
    table.to_string()
}

impl Update {
    pub fn default() -> Self {
        Update {
            table_name: String::default(),
            columns: vec![],
            values: vec![vec![]],
            where_clause: WhereClause::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::command::Command;
    use crate::command::insert::Insert;
    use crate::command::sqlcommands::SqlCommand;
    use crate::command::sqloperator::Operator;
    use crate::command::update::Update;
    use crate::database::database::Database;
    use crate::database::datatype::DataType;

    #[test]
    fn simple_update_with_where() {
        let dbs: Vec<Database> = vec!();
        let stmt = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt' WHERE CustomerID = 1;";
        Update::parse(stmt.to_string(), dbs);
    }

    #[test]
    fn simple_update_without_where() {
        let dbs: Vec<Database> = vec!();
        let stmt = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt';";
        Update::parse(stmt.to_string(), dbs);
    }
}