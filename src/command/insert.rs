use crate::command::whereclause::WhereClause;

pub struct Insert {
    table_name: String,
    columns: Vec<String>,
    data: Vec<String>,
    where_clause: WhereClause,
}