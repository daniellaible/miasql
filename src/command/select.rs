use crate::command::whereclause::WhereClause;
use regex::Regex;
use crate::command::sqloperator::Operator;
use crate::table::datatype::DataType;

pub struct Select {
    table_name: String,
    columns: Vec<String>,
    where_clause: WhereClause,
}

pub fn parse_stmt(stmt: String) -> Select {
    let select: Select = Select::default();

    let columns: Vec<&str> = get_columns(&stmt);
    for mut column in columns {
        column = column.trim();
        println!("{}", column);
    }

    let tablename = match get_table_name(&stmt) {
        Some(x) => x,
        None => "doof",
    };

    if check_for_where(&stmt) {
        let clause: WhereClause = get_where_clause(&stmt);
        println!("{:?}", clause);
    }
    println!("{}", tablename);
    select
}

fn get_where_clause(stmt: &String) -> WhereClause {
    let regex = Regex::new(r"(?i)\bwhere\b\s+([\s\S]*)").unwrap();
    let captures = regex.captures(stmt).unwrap();
    let where_as_string = captures.get(1).unwrap().as_str();

    let x: Vec<&str> = where_as_string.split("=").collect();
    let where_clause = WhereClause::new(String::from(x[0]), Operator::EQUAL, DataType::Int {x: 1});
    println!("{}", where_as_string);
    where_clause
}

fn get_columns(stmt: &String) -> Vec<&str> {
    let regex = Regex::new(r"(?i)\bselect\b\s*([\s\S]*?)\s+\bfrom\b\s+").unwrap();
    let captures = regex.captures(stmt).unwrap();
    let columns_as_string = captures.get(1).unwrap().as_str();
 
    let parts: Vec<&str> = columns_as_string.split(",").collect();
    parts
}

fn get_table_name(stmt: &str) -> Option<&str> {
    let re = Regex::new(r"(?i)\bfrom\b\s+([^\s;]+)(?:\s+\bwhere\b\s+[\s\S]*)?$").unwrap();
    re.captures(stmt).and_then(|c| c.get(1).map(|m| m.as_str()))
}

fn check_for_where(stmt: &String) -> bool {
    let reg = Regex::new(r"(?i)\swhere\s").unwrap();
    if reg.is_match(stmt){
        return true;
    }
    return false;
}

impl Select {
    pub fn default() -> Self {
        Select {
            table_name: String::default(),
            columns: vec![],
            where_clause: WhereClause::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::select::parse_stmt;

    #[test]
    fn simple_select_with_where_clause() {
        let select = "SELECT name, country FROM population WHERE id=1";
        parse_stmt(String::from(select));
    }

    #[test]
    fn simple_select_with_where_clause_lowercase() {
        let select = "select name, country from population where id=1";
        parse_stmt(String::from(select));
    }

    #[test]
    fn simple_select_without_where_clause() {
        let select = "SELECT name, country FROM population";
        parse_stmt(String::from(select));
    }
}
