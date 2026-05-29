use crate::command::constraint::Constraint;
use crate::command::createtable::ParsedForeignKey;
use crate::command::select::{JoinClause};
use crate::command::update::UpdateSet;
use crate::command::whereclause::WhereClause;

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
        limit: i32,
    },
    CREATE_TABLE {
        command: String,
        table: String,
        columns: Vec<(String, String, Vec<Constraint>)>,
        foreign_keys: Vec<ParsedForeignKey>,
    },
    CREATE_DATABASE {
        command: String,
        database: String,
        comment: String,
    },
    DROP_TABLE {
        command: String,
        table: String
    },
    DROP_DATABASE{
        command: String,
        database: String
    },
    DELETE {
        command: String,
        table: String,
        where_clause: WhereClause
    },
    TRUNCATE {
        command: String,
        tables: Vec<String>
    },
    UPDATE {
        command: String,
        table: String,
        sets: Vec<UpdateSet>,
        where_clause: WhereClause
    },
    ALTER {command: String, table: String, columns: Vec<String>, values: Vec<String>, where_clause: WhereClause },
    INSERT {command: String, table: String, columns: Vec<String>, values: Vec<Vec<String>>, where_clause: WhereClause },
    UNDEFINED
} 