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
        let table: String = get_table(&stmt);
        let columns_value_pairs: Vec<String> = get_column_value_pairs(&stmt);
        let columns:Vec<String> = get_columns(columns_value_pairs.clone());
        let values:Vec<String> = get_values(columns_value_pairs.clone());
        let clause: WhereClause = WhereClause::parse(&stmt);
        let command = SqlCommand::UPDATE {
            command: String::from("UPDATE"),
            table,
            columns,
            values,
            where_clause: clause,
        };
        command
    }
}

fn get_columns(column_values: Vec<String>) -> Vec<String> {
    let mut columns: Vec<String> = vec![];
    for entry in column_values.iter() {
        let splits: Vec<&str> = entry.split("=").collect();
        let cleaned = splits[0].replace("'", "").to_string();
        columns.push(cleaned.trim().to_string());
    }
    columns
}

fn get_values(column_values: Vec<String>) -> Vec<String> {
    let mut columns: Vec<String> = vec![];
    for entry in column_values.iter() {
        let splits: Vec<&str> = entry.split("=").collect();
        let cleaned = splits[1].replace("'", "").to_string();
        columns.push(cleaned.trim().to_string());
    }
    columns
}

fn get_column_value_pairs(mut stmt: &String) -> Vec<String> {

    if stmt.contains(" WHERE ") {
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
    use crate::command::sqlcommands::SqlCommand;
    use crate::command::sqloperator::Operator;
    use crate::command::update::Update;
    use crate::database::database::Database;
    use crate::database::datatype::DataType;

    #[test]
    fn simple_update_with_where() {
        let dbs: Vec<Database> = vec!();
        let stmt = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt' WHERE CustomerID = 1;";
        let command = Update::parse(stmt.to_string(), dbs);

        match command {
            SqlCommand::UPDATE {
                command,
                table,
                columns,
                values,
                where_clause,
            } => {
                assert_eq!(command, "UPDATE");
                assert_eq!(table, "Customers");
                assert_eq!(columns, vec!["ContactName", "City"]);
                assert_eq!(values, vec!["Alfred Schmidt", "Frankfurt"]);

                let clause = where_clause;
                assert_eq!(clause.get_operator(), Operator::EQUAL);
                assert_eq!(clause.get_column(), "CUSTOMERID");
                assert_eq!(clause.get_value(), DataType::BigInt {x:1});
            }
            _ => (),
        }
    }

    #[test]
    fn simple_update_without_where() {
        let dbs: Vec<Database> = vec!();
        let stmt = "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt';";
        let command = Update::parse(stmt.to_string(), dbs);

        match command {
            SqlCommand::UPDATE {
                command,
                table,
                columns,
                values,
                where_clause,
            } => {
                assert_eq!(command, "UPDATE");
                assert_eq!(table, "Customers");
                assert_eq!(columns, vec!["ContactName", "City"]);
                assert_eq!(values, vec!["Alfred Schmidt", "Frankfurt"]);

                let clause = where_clause;
                assert_eq!(clause.get_operator(), Operator::UNDEFINED);
                assert_eq!(clause.get_column(), "");
                assert_eq!(clause.get_value(), DataType::Undefined);
            }
            _ => (),
        }
    }
}