use crate::command::whereclause::WhereClause;

#[derive(Clone, Debug)]
pub enum SqlCommand {
    SELECT {table: String, columns: Vec<String>, values: vec![], where_clause: WhereClause },
    CREATE {table: String, columns: Vec<String>, values: vec![], where_clause: WhereClause },
    DROP {table: String, columns: Vec<String>, values: vec![], where_clause: WhereClause },
    ALTER {table: String, columns: Vec<String>, values: vec![], where_clause: WhereClause },
    INSERT {table: String, columns: Vec<String>, values: vec![], where_clause: WhereClause },
    UPDATE {table: String, columns: Vec<String>, values: vec![], where_clause: WhereClause },
    DELETE {table: String, columns: Vec<String>, values: vec![], where_clause: WhereClause },
    TRUNCATE {table: String, columns: Vec<String>, values: vec![], where_clause: WhereClause },
    GRANT,
    REVOKE,
}