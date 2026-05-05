use crate::command::whereclause::WhereClause;
use regex::Regex;
use crate::command::select::Select;
use crate::table::datatype::DataType;

#[derive(Debug)]
pub struct Insert {
    table_name: String,
    columns: Vec<String>,
    values: Vec<Vec<DataType>>,
    where_clause: WhereClause,
}

pub fn parse_stmt(stmt: String) -> Insert {
    let mut insert: Insert = Insert::default();
    let table:String = get_table(&stmt);
    insert
}

fn get_table(stmt: &String) -> String {
    let regex = Regex::new(r"(?i)\binsert\b\s*into\b\s*([\s\S]*?)\s+\bvalues\b\s+").unwrap();
    let captures = regex.captures(stmt).unwrap();
    let what_we_got = captures.get(1).unwrap().as_str();
    println!("{:?}", &what_we_got);
    String::new()
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
    use crate::command::insert::parse_stmt;

    #[test]
    fn simple_select_with_where_clause() {
        let select = "INSERT INTO user (first_name, last_name, age) VALUES ('daniel', 'mayer', '35') ";
        parse_stmt(String::from(select));
    }
}