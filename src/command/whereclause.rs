use crate::command::sqloperator::Operator;
use crate::database::datatype::DataType;

#[derive(Clone, Debug, PartialEq)]
pub struct WhereClause {
    pub column: String,
    pub operator: Operator,
    pub value: DataType,
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

    pub fn error() -> Self {
        WhereClause {
            column: String::default(),
            operator: Operator::UNDEFINED,
            value: DataType::Undefined,
        }
    }
}
