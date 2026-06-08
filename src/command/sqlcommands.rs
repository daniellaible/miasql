use crate::command::constraint::Constraint;
use crate::command::createtable::ParsedForeignKey;
use crate::command::select::{JoinClause};
use crate::command::update::UpdateSet;
use crate::command::whereclause::WhereClause;
use crate::database::datatype::DataType;

#[derive(Clone, Debug, PartialEq)]
pub enum SqlCommand {
    Select {
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
    CreateTable {
        command: String,
        table: String,
        columns: Vec<(String, String, Vec<Constraint>)>,
        foreign_keys: Vec<ParsedForeignKey>,
    },
    CreateDatabase {
        command: String,
        database: String,
        comment: String,
    },
    DropTable {
        command: String,
        table: String
    },
    DropDatabase {
        command: String,
        database: String
    },
    Delete {
        command: String,
        table: String,
        where_clause: WhereClause
    },
    Truncate {
        command: String,
        tables: Vec<String>
    },
    Update {
        command: String,
        table: String,
        sets: Vec<UpdateSet>,
        where_clause: WhereClause
    },
    Insert {
        command: String,
        table: String,
        columns: Vec<String>,
        values: Vec<Vec<String>>,
    },
    AlterAddColumn {
        command: String,
        table: String,
        columns: Vec<(String, DataType, Vec<Constraint>)>,
    },
    AlterDropColumn {
        command: String,
        table: String,
        columns: Vec<(String)>,
    },
    AlterRenameColumn {
        command: String,
        table: String,
        old: String,
        new: String,
    },
    AlterModifyColumn {
        command: String,
        table: String,
        column: String,
        data_type: DataType,
        constraints: Vec<Constraint>,
    },

    AlterTableRename {
        command: String,
        table: String,
        new_name: String,
    },
    Undefined
} 