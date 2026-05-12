use crate::command::sqloperator::Operator;
use crate::table::datatype::DataType;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub struct WhereClause {
    column: String,
    operator: Operator,
    value: DataType,
}

impl WhereClause {
    pub fn new(column: String, operator: Operator, value: DataType) -> Self {
        WhereClause { column, operator, value }
    }

    pub fn get_operator(&self) -> Operator {
        self.operator.clone()
    }

    pub fn get_column(&self) -> String {
        self.column.clone()
    }

    pub fn get_value(&self) -> DataType {
        self.value.clone()
    }

    pub fn default() -> Self {
        WhereClause {
            column: String::default(),
            operator: Operator::UNDEFINED,
            value: DataType::Undefined,
        }
    }

    pub fn parse(stmt: &String) -> WhereClause {
        let lower_case_stmt: String = stmt.to_lowercase();
        if !lower_case_stmt.contains("where"){
            return WhereClause::default();
        }
        let regex = Regex::new(r"(?i)\bwhere\b\s+([\s\S]*)").unwrap();
        let captures = regex.captures(stmt).unwrap();
        let where_as_string = captures.get(1).unwrap().as_str();

        let mut splits: Vec<&str> = vec!();
        let mut operator: Operator = Operator::UNDEFINED;
        if where_as_string.contains("=") {
            splits = where_as_string.split("=").collect();
            operator = Operator::EQUAL;
        } else if where_as_string.contains("<") {
            splits = where_as_string.split("<").collect();
            operator = Operator::LESSER;
        } else if where_as_string.contains(">") {
            splits = where_as_string.split(">").collect();
            operator = Operator::GREATER;
        } else if where_as_string.contains("<=") {
            splits = where_as_string.split("<=").collect();
            operator = Operator::LESSEROREQ;
        } else if where_as_string.contains(">=") {
            splits = where_as_string.split(">=").collect();
            operator = Operator::GREATEROREQ;
        } else if where_as_string.contains("!=") {
            splits = where_as_string.split("!=").collect();
            operator = Operator::NOTEQUAL;
        }

        splits[0] = splits[0].trim();
        let where_clause = WhereClause::new(String::from(splits[0]), operator, DataType::BigInt { x: splits[1].parse().unwrap() });
        where_clause
    }
}
