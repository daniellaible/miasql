use crate::command::sqloperator::Operator;

pub struct WhereClause {
    column: String,
    operator: Operator,
    value: String,
}

impl WhereClause {
    pub fn default() -> Self{
        WhereClause{
            column: String::default(),
            operator: Operator::UNDEFINED,
            value: String::default(),
        }
    }
}
