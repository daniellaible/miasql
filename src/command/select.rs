use crate::command::whereclause::WhereClause;

use regex::Regex;
use crate::command::sqloperator::Operator;
use crate::table::datatype::DataType;

#[derive(Debug)]
pub struct Select {
    table_name: String,
    columns: Vec<String>,
    where_clause: WhereClause,
}

pub fn parse_stmt(stmt: String) -> Select{
    let mut select: Select = Select::default();
    select.columns = get_columns(&stmt);
    select.table_name = get_table_name(&stmt).unwrap_or_else(|| "doof").parse().unwrap();
    if check_for_where(&stmt) {
        select.where_clause = WhereClause::parse(&stmt);
        println!("{:?}", select);
        select
    }else{
        println!("{:?}", select);
        select
    }
}

fn get_columns(stmt: &String) -> Vec<String> {
    let regex = Regex::new(r"(?i)\bselect\b\s*([\s\S]*?)\s+\bfrom\b\s+").unwrap();
    let captures = regex.captures(stmt).unwrap();
    let columns_as_string = captures.get(1).unwrap().as_str();
    let single_column:Vec<&str> = columns_as_string.split(",").collect();
    let mut parts: Vec<String> = Vec::new();
    for single_column in single_column {
        parts.push(single_column.to_string());
    }
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
    false
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
    use crate::command::select::{parse_stmt, Select};
    use crate::command::sqloperator::Operator;
    use crate::command::whereclause::WhereClause;

    #[test]
    fn simple_select_with_where_clause() {
        let select = "SELECT name, country FROM population WHERE id=1";
        let select: Select = parse_stmt(String::from(select));
        let clause: WhereClause = select.where_clause;
        assert_eq!(select.table_name, "population");
        assert_eq!(select.columns.len(), 2);
        assert_eq!(clause.get_operator(), Operator::EQUAL);
    }

    #[test]
    fn simple_select_with_where_clause_lowercase() {
        let select = "select name, country from population where id=1";
        let select: Select = parse_stmt(String::from(select));
        let clause: WhereClause = select.where_clause;
        assert_eq!(select.table_name, "population");
        assert_eq!(select.columns.len(), 2);
        assert_eq!(clause.get_operator(), Operator::EQUAL);
    }

    #[test]
    fn simple_select_with_where_clause_less_than() {
        let select = "select name, country from population where id<100";
        let select: Select = parse_stmt(String::from(select));
        let clause: WhereClause = select.where_clause;
        assert_eq!(select.table_name, "population");
        assert_eq!(select.columns.len(), 2);
        assert_eq!(clause.get_operator(), Operator::LESSER);
    }

    #[test]
    fn simple_select_with_where_clause_greater_than() {
        let select = "select name, country from population where id>100";
        let select: Select = parse_stmt(String::from(select));
        let clause: WhereClause = select.where_clause;
        assert_eq!(select.table_name, "population");
        assert_eq!(select.columns.len(), 2);
        assert_eq!(clause.get_operator(), Operator::GREATER);
    }

    #[test]
    fn simple_select_without_where_clause() {
        let select = "SELECT name, country FROM population";
        let select: Select = parse_stmt(String::from(select));
        let clause: WhereClause = select.where_clause;
        assert_eq!(select.table_name, "population");
        assert_eq!(select.columns.len(), 2);
        assert_eq!(clause.get_operator(), Operator::UNDEFINED);
    }
}
