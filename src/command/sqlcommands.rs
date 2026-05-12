use crate::command::whereclause::WhereClause;
use crate::table::datatype::DataType;

#[derive(Clone, Debug)]
pub enum SqlCommand {
    SELECT {command: String, table: String, columns: Vec<String>, values: Vec<DataType>, where_clause: WhereClause },
    CREATE {command: String, table: String, columns: Vec<String>, values: Vec<DataType>, where_clause: WhereClause },
    DROP {command: String, table: String, columns: Vec<String>, values: Vec<DataType>, where_clause: WhereClause },
    ALTER {command: String, table: String, columns: Vec<String>, values: Vec<DataType>, where_clause: WhereClause },
    INSERT {command: String, table: String, columns: Vec<String>, values: Vec<Vec<DataType>>, where_clause: WhereClause },
    UPDATE {command: String, table: String, columns: Vec<String>, values: Vec<DataType>, where_clause: WhereClause },
    DELETE {command: String, table: String, columns: Vec<String>, values: Vec<DataType>, where_clause: WhereClause },
    TRUNCATE {command: String, table: String, columns: Vec<String>, values: Vec<DataType>, where_clause: WhereClause },
}