use std::fmt;
use crate::command::constraint::Constraint;
use crate::command::createtable::ForeignKeyToken;
use crate::command::select::{JoinClause};
use crate::command::update::UpdateSet;
use crate::command::whereclause::WhereClause;
use crate::database::datatype::DataType;
/// # SqlCommand
///
/// This enum stores all the tokens of a sql command, after the user input has been tokenized
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
    /// command: CREATE_TABLE
    /// table: <name of new table>
    /// columns: Vector with tuple (column_name, datatype. Vec<column_constraints>)
    CreateTable {
        command: String,
        table: String,
        columns: Vec<(String, DataType, Vec<Constraint>)>,
        foreign_keys: Vec<ForeignKeyToken>,
    },
    /// command: CREATE_DATABASE
    /// database: <name of new database>
    /// comment: !ignored at the moment
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
    /// command is always INSERT
    /// table is the table name you want to insert it toto
    /// columns are the columns you want to insert it into
    /// values are the values of the columns, 2dim array because you can insert more than one row at a time
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
        columns: Vec<String>,
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
    
    Use {
        command: String,
        database: String,
    },
    
    Quit {command: String,},
    ShowDatabases { command: String,},
    ShowTables { command: String, database: String},
    Undefined ,
}
