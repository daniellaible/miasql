use crate::command::constraint::Constraint;
use crate::database::bptree::BPlusTree;
use crate::database::datatype::DataType;
use std::error::Error;
use std::fmt;
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Debug)]
pub struct Row {
    pub data: Vec<DataType>,
}


#[derive(Debug, Clone)]
pub struct Table {
    pub max_id: i64,
    pub db_name: String,
    pub table_name: String,
    pub mtd_path: String,
    pub tree: BPlusTree<i64, Vec<DataType>, 3>,
    pub uuid: Uuid,
    pub display_order: Vec<(u32, u32)>,
    pub column_names: Vec<String>,
    pub column_types: Vec<DataType>,
    pub constraint: Vec<(u32, Constraint)>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let table_name = &self.table_name;
        write!(f, "Something went wrong with table: {table_name}")
    }
}

impl Error for Table {}

impl Table {
    pub fn default() -> Self {
        Table {
            max_id: 0,
            db_name: "".to_string(),
            table_name: "".to_string(),
            tree: Default::default(),
            uuid: Default::default(),
            mtd_path: Default::default(),
            column_names: vec![],
            column_types: vec![],
            display_order: vec![],
            constraint: vec![],
        }
    }

    /// creates a new database with the params:
    /// - new() Constructor
    /// - get_table_name() - returns the human-readable name of teh database
    /// - get_bptree() - returns the tree
    /// - get_uuid() gets the uuid of the database///
    pub fn new(
        max_id: i64,
        db_name: String,
        table_name: String,
        tree: BPlusTree<i64, Vec<DataType>, 3>,
        uuid: Uuid,
        mtd_path: String,
        column_names: Vec<String>,
        display_order: Vec<(u32, u32)>,
        constraints: (u32, Vec<Constraint>),
    ) -> Table {
        // todo check if there are duplicates in the names

        Table {
            max_id,
            db_name: "".to_string(),
            table_name,
            tree,
            mtd_path,
            uuid,
            display_order: vec![],
            column_names,
            column_types: vec![],
            constraint: vec![],
        }
    }

    pub fn inc_max_id(&mut self) -> i64 {
        self.max_id += 1;
        self.max_id
    }
}


#[cfg(test)]
mod tests {

}
