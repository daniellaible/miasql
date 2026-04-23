use crate::command::sqloperator::Operator;
use crate::table::datatype::DataType;

#[derive(Clone, Debug)]
pub struct WhereClause {
    column: String,
    operator: Operator,
    value: DataType,
}

impl WhereClause {
    pub fn new(column: String, operator: Operator, value: DataType ) -> Self {
        WhereClause{column, operator, value}
    }
    
    pub fn default() -> Self{
        WhereClause{
            column: String::default(),
            operator: Operator::UNDEFINED,
            value: DataType::Undefined,
        }
    }
}
