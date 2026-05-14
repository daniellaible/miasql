use crate::command::sqloperator::Operator;
use crate::database::datatype::DataType;
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
        let mut upper_case_stmt: String = stmt.to_uppercase();
        if upper_case_stmt.contains(";"){
            upper_case_stmt = upper_case_stmt.replace(";", "");
        }

        if !upper_case_stmt.contains("WHERE"){
            return WhereClause::default();
        }
        let regex = Regex::new(r"(?i)\bWHERE\b\s+([\s\S]*)").unwrap();
        let captures = regex.captures(&upper_case_stmt).unwrap();
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
        let column = String::from(splits[0]);
        let id:i64 = splits[1].trim().parse().unwrap();

        let where_clause = WhereClause::new(column, operator, DataType::BigInt { x: id });
        where_clause
    }
}
