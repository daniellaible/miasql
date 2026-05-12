use crate::command::whereclause::WhereClause;
use crate::table::datatype::DataType;
use regex::Regex;
use crate::command::command::Command;
use crate::table::datatype::DataType::BigInt;



#[derive(Debug)]
pub struct Insert {
    table_name: String,
    columns: Vec<String>,
    values: Vec<Vec<DataType>>,
    where_clause: WhereClause,
}

impl Command for Insert{
    fn parse(stmt: String) -> () {
        let mut insert: Insert = Insert::default();
        let table: String = get_table(&stmt);
        insert.table_name = table;

        let columns: Vec<String> = get_columns(&stmt);
        insert.columns = columns;

        insert.where_clause = WhereClause::parse(&stmt);

        println!("table name: {:?}", insert.table_name);
        println!("columns: {:?}", insert.columns);
        println!("where_clause: {:?}", insert.where_clause);
    }
}

fn find_values(stmt: &String)-> Vec<Vec<DataType>> {
    vec![vec![BigInt {x:1}]]
}

fn get_columns(stmt: &String) -> Vec<String> {
    let table_n_columns: &str = retrieve_table_n_columns(&stmt);
    let re = Regex::new(r"\((.*)\)").unwrap();
    let mut temp_columns = Vec::new();
    if let Some(caps) = re.captures(table_n_columns) {
        let inside = caps.get(1).unwrap().as_str();
        temp_columns = inside.split(',').collect::<Vec<&str>>();
    }

    let mut columns: Vec<String> = Vec::new();
    for  column in temp_columns.iter() {
        let c = column.trim();
        columns.push(c.to_string());
    }
    columns
}

fn get_table(stmt: &String) -> String {
    let table_n_columns: &str = retrieve_table_n_columns(&stmt);

    //This regex gets you all until the first opening brace
    let regex_table = Regex::new(r"^\s*([^(]*?)\s*(?:\(|$)").unwrap();
    let captures_table = regex_table.captures(table_n_columns).unwrap();
    let table_name = captures_table.get(1).unwrap().as_str();
    String::from(table_name)
}

fn retrieve_table_n_columns(stmt: &String) -> &str{
    let regex_table_n_columns =
        Regex::new(r"(?i)\binsert\b\s*into\b\s*([\s\S]*?)\s+\bvalues\b\s+").unwrap();
    let captures_table_n_columns = regex_table_n_columns.captures(stmt).unwrap();
    let table_n_columns = captures_table_n_columns.get(1).unwrap().as_str();
    table_n_columns
}

impl Insert {
    pub fn default() -> Self {
        Insert {
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

    #[test]
    fn simple_select_without_where_clause() {
        let select =
            "INSERT INTO user (first_name, last_name, age) VALUES ('daniel', 'mayer', '35')";
       Insert::parse(select.to_string());
    }

    #[test]
    fn simple_select_with_where_clause() {
        let select =
            "INSERT INTO user (first_name, last_name, age) VALUES ('daniel', 'mayer', '35') where id=1";
        Insert::parse(String::from(select));
    }

    // This should be possible as well
    // Copy all columns from one table to another table:
    //      INSERT INTO target_table
    //          SELECT * FROM source_table
    //          WHERE condition;
    //
    //      INSERT INTO
    //          table_name (column1, column2, column3)
    //      VALUES
    //          (value11, value12, value13),
    //          (value21, value22, value23),
    //          (value31, value32, value33);
    //
    // Copy only some columns from one table to another table:
    //      INSERT INTO target_table (column1, column2, column3, ...)
    //          SELECT column1, column2, column3, ...
    //          FROM source_table
    //      WHERE condition;
}

