use whereclause::WhereClause;

mod whereclause;

pub struct Select {
    table_name: String,
    columns: Vec<String>,
    data: Vec<String>,
    where_clause: WhereClause,
}