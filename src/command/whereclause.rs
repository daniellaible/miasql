use sqloperator::Operator;
mod sqloperator;

pub struct WhereClause {
    column: String,
    operator: Operator,
    value: String,
}
