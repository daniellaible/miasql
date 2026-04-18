//! This class represents a table of the database
//!
//! Use this class to access tables in the database.
//!
//! Parameters for creating a tabl are:
//! - table_name
//! - the B+Tree to store the data in
//! - unique uuid that two tables with the same name but within different databases can be accessed
//! - the name of the table columns
//! - the datatype of the different columns
//!
//! Features
//! - each table has a name
//! - each table hasa uuid
//! - each table consists at least of one B+Tree
//!
//! Functionality
//! - new() Constructor
//! - get_table_name() - returns the human-readable name of teh table
//! - get_bptree() - returns the tree
//! - get_uuid() gets the uuid of the table

use std::fs::File;
use std::io::{BufReader, Read};
use std::fmt;
use crate::bptree;
use crate::bptree::BPlusTree;
use uuid::Uuid;
use datatype::DataType;


mod datatype;

#[derive(Clone)]
pub struct Table {
    table_name: String,
    tree: BPlusTree<i32, String, 3>,
    uuid: Uuid,
    column_names: Vec<String>,
    column_types: Vec<DataType>,
}

impl Table {

    pub fn new_empty() -> Self {
        Table{
            table_name: "".to_string(),
            tree: Default::default(),
            uuid: Default::default(),
            column_names: vec![],
            column_types: vec![]    ,
        }
    }

    /// creates a new table with the params:
    /// - new() Constructor
    /// - get_table_name() - returns the human-readable name of teh table
    /// - get_bptree() - returns the tree
    /// - get_uuid() gets the uuid of the table///
    pub fn new(table_name: String, tree: BPlusTree<i32, String, 3>, uuid: Uuid,
               names: Vec<String>, types: Vec<DataType>) -> Table {

        if names.len() != types.len(){
            panic!("names length mismatch - unable to create such a mess");
        }

        // todo check if there are duplicates in the names

        Table {
            table_name,
            tree,
            uuid,
            column_names: names,
            column_types: types,
        }
    }

    /// returns the name of the table in a human-readable form
    pub fn get_table_name(&self) -> String {
        self.table_name.clone()
    }

    /// returns the B+Tree
    pub fn get_bptree(&self) -> &bptree::BPlusTree<i32, String, 3> {
        &self.tree
    }

    ///returns the Uuid of this table
    pub fn get_uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    pub fn read_table_from_disc(&self, path: String) -> (){
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);


        let mut version_bytes = [0u8; 4];
        reader.read_exact(&mut version_bytes).unwrap();
        let version = f32::from_be_bytes(version_bytes);
        println!("next 4 bytes as f32: {}", version);
    }
}

#[cfg(test)]
mod tests {
    use super::Table;
    use crate::bptree::BPlusTree;
    use uuid::Uuid;
    use crate::table::datatype::DataType;

    #[test]
    fn create_new_table() {
        let names:Vec<String> = vec![String::from("id"), String::from("first_name"), String::from("last_name"), String::from("age")];
        let types:Vec<DataType> = vec![DataType::BigInt{ x : 0}, DataType::VarChar {x : String::from(" "), y: 0}, DataType::VarChar {x : String::from(" "), y: 0}, DataType::Int {x: 50 }];
        let mut _bp_tree = BPlusTree::default();
        let table: Table = Table::new(String::from("test"), _bp_tree, Uuid::new_v4(), names, types);
        let name: String = table.get_table_name();
        assert_eq!(name, "test");
    }

    #[test]
    fn load_from_disc() {
        let table: Table = Table::new_empty();
        table.read_table_from_disc(String::from("C:/temp/moi/0e6bce68-99fa-3841-b790-24afbdf7db1d.moi"));
    }
}
