use crate::command::constraint::Constraint;
use crate::command::select::JoinClause;
use crate::command::whereclause::WhereClause;
use crate::database::datatype::DataType;

#[derive(Clone, Debug, PartialEq)]
pub enum SqlCommand {
    SELECT {
        command: String,
        table: String,
        columns: Vec<String>,
        where_clause: WhereClause,
        distinct: bool,
        group_by: Vec<String>,
        order_by: Vec<String>,
        joins: Vec<JoinClause>,
    },
    /// command is always CREATE_DATABASE, in table the table name is stored and in columns is a vector stored that contains tupels
    /// that are structured like Vec<(column_name, datatype, Vec<constraint1, constraint2 ...>)>
    CREATE_TABLE {
        command: String,
        table: String,
        columns: Vec<(String, String, Vec<Constraint>)>
    },
    CREATE_DATABASE {command: String, database: String},
    DROP_TABLE {command: String, table: String},
    DROP_DATABASE{command: String, database: String},
    ALTER {command: String, table: String, columns: Vec<String>, values: Vec<String>, where_clause: WhereClause },
    INSERT {command: String, table: String, columns: Vec<String>, values: Vec<Vec<String>>, where_clause: WhereClause },
    UPDATE {command: String, table: String, columns: Vec<String>, values: Vec<String>, where_clause: WhereClause },
    DELETE {command: String, table: String, where_clause: WhereClause },
    TRUNCATE {command: String, table: String, columns: Vec<String>, values: Vec<String>, where_clause: WhereClause },
    UNDEFINED
} 