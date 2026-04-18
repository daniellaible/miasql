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
use std::time::{Duration, Instant};
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

    /// sets the table name of the table
    pub fn set_table_name(&mut self, table_name: String)  {
        self.table_name = table_name;
    }

    /// returns the B+Tree
    pub fn get_bptree(&self) -> &bptree::BPlusTree<i32, String, 3> {
        &self.tree
    }

    /// if the tree has been changed or the tree has been loaded from disc
    pub fn set_bptree(&mut self, tree: BPlusTree<i32, String, 3>) {
        self.tree = tree;
    }

    ///returns the Uuid of this table
    pub fn get_uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }

    pub fn set_column_names(&mut self, column_names: Vec<String>) {
        self.column_names = column_names;
    }

    pub fn get_column_names(&self) -> &Vec<String> {
        &self.column_names
    }

    pub fn set_column_types(&mut self, column_types: Vec<DataType>) {
        self.column_types = column_types;
    }

    pub fn get_column_types(&self) -> &Vec<DataType> {
        &self.column_types
    }

    pub fn read_table_from_disc(&self, path: String) -> (){
        let start = Instant::now();
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);


        let mut version_bytes = [0u8; 4];
        reader.read_exact(&mut version_bytes).unwrap();
        let version = f32::from_be_bytes(version_bytes);
        println!("next 4 bytes as f32 (version): {}", &version);

        let mut number_of_columns_bytes = [0u8; 2];
        reader.read_exact(&mut number_of_columns_bytes).unwrap();
        let number_of_columns = i16::from_be_bytes(number_of_columns_bytes);
        let columns_usize: usize = number_of_columns.try_into().expect("table name size is negative");
        println!("next 2 bytes as i16 (number_of_columns): {}", &number_of_columns);

        let mut part_bytes = [0u8; 2];
        reader.read_exact(&mut part_bytes).unwrap();
        let part = i16::from_be_bytes(part_bytes);
        println!("next 2 bytes as i16 (part): {}", &part);

        let mut part_of_bytes = [0u8; 2];
        reader.read_exact(&mut part_of_bytes).unwrap();
        let part_of = i16::from_be_bytes(part_of_bytes);
        println!("next 2 bytes as i16 (part_of): {}", &part_of);

        let mut next_file_length_byte = [0u8; 2];
        reader.read_exact(&mut next_file_length_byte).unwrap();
        let next_file_len = i16::from_be_bytes(next_file_length_byte);
        println!("next 2 bytes as i16 (next file len): {}", &next_file_len);

        if next_file_len != 0 {
            println!("Next file was not implemented yet - however the length seems to be > 0");
        }

        let mut table_name_length_byte = [0u8; 2];
        reader.read_exact(&mut table_name_length_byte).unwrap();
        let table_name_len = i16::from_be_bytes(table_name_length_byte);
        println!("next 2 bytes as i16 (table name len): {}", &table_name_len);
        let table_name_len: usize = table_name_len.try_into().expect("table name size is negative");

        let mut table_name_byte = vec![0u8; table_name_len];
        reader.read_exact(&mut table_name_byte).unwrap();
        let mut table_name = String::from_utf8(table_name_byte).unwrap();
        let cleaned_name = table_name.trim_matches('"');
        println!("next: (table name): {}", cleaned_name);

        let table_width: usize = match usize::try_from(number_of_columns) {
            Ok(v) => v,
            Err(_) => return,
        };
        let mut column_names: Vec<String> = vec![String::new(); table_width ];
        for i in 0..table_width {
            let mut column_name_len_byte = [0u8; 2];
            reader.read_exact(&mut column_name_len_byte).unwrap();
            let column_name_len = i16::from_be_bytes(column_name_len_byte);
            let column_name_size: usize = column_name_len.try_into().expect("table name length was negative");

            let mut column_name_byte = vec![0u8; column_name_size];
            reader.read_exact(&mut column_name_byte).unwrap();
            let mut table_name = String::from_utf8(column_name_byte).unwrap();
            column_names[i] = table_name.clone();
        }
        println!("Column names: {:?}", column_names);


        let duration = start.elapsed();
        println!("Total time taken: {:?}", duration);
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
        let bp_tree = BPlusTree::default();
        let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap();
        let table: Table = Table::new(String::from("test"), bp_tree, uuid, names, types);

        let name: String = table.get_table_name();
        assert_eq!(name, "test");

        let uuid: Uuid = table.get_uuid();
        assert_eq!(String::from(uuid), "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8");
    }

    #[test]
    fn load_from_disc() {
        let table: Table = Table::new_empty();
        table.read_table_from_disc(String::from("C:/temp/moi/0e6bce68-99fa-3841-b790-24afbdf7db1d.moi"));
    }
}
