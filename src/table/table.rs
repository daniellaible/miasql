//! This class represents a table of the database
//!
//! Use this class to access tables in the database.
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
    pub fn new(tablename: String, tree: BPlusTree<i32, String, 3>, uuid: Uuid,
               names: Vec<String>, types: Vec<DataType>) -> Table {
        Table {
            table_name: tablename,
            tree,
            uuid,
            column_names: names,
            column_types: types,
        }
    }

    pub fn get_table_name(&self) -> String {
        return self.table_name.clone();
    }

    pub fn get_bptree(&self) -> &bptree::BPlusTree<i32, String, 3> {
        return &self.tree;
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
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
}
