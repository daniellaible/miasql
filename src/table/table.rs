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
//! - each table has a uuid
//! - each table consists at least of one B+Tree
//!

use std::fs;
use crate::bptree;
use crate::bptree::BPlusTree;
use datatype::DataType;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Write};
use std::time::Instant;
use uuid::Uuid;

mod datatype;

pub fn save_table_to_disc(table: &Table, path: &String, uuid: &Uuid) {

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // start fresh; remove if you truly want to append to existing file
        .open(&path)
        .unwrap();

    let version: f32 = 1.0;
    let number_of_columns:i16 = 5;
    let part:i16 = 1;
    let part_of:i16 = 1;
    let next_file_length: i16 = 6;
    let next_file_name: &str = "cities";


    file.write_all(&version.to_be_bytes()).unwrap();
    file.write_all(&number_of_columns.to_be_bytes()).unwrap();
    file.write_all(&part.to_be_bytes()).unwrap();
    file.write_all(&part_of.to_be_bytes()).unwrap();
    file.write_all(&next_file_length.to_be_bytes()).unwrap();
    file.write_all(&next_file_name.as_bytes()).unwrap();

}

pub fn read_table_from_disc(path: String, uuid: Uuid) -> Table {
    let start = Instant::now();
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut version_bytes = [0u8; 4];
    reader.read_exact(&mut version_bytes).unwrap();
    let version = f32::from_be_bytes(version_bytes);

    let mut number_of_columns_bytes = [0u8; 2];
    reader.read_exact(&mut number_of_columns_bytes).unwrap();
    let number_of_columns = i16::from_be_bytes(number_of_columns_bytes);

    let mut part_bytes = [0u8; 2];
    reader.read_exact(&mut part_bytes).unwrap();
    let part = i16::from_be_bytes(part_bytes);

    let mut part_of_bytes = [0u8; 2];
    reader.read_exact(&mut part_of_bytes).unwrap();
    let part_of = i16::from_be_bytes(part_of_bytes);

    let mut next_file_length_byte = [0u8; 2];
    reader.read_exact(&mut next_file_length_byte).unwrap();
    let next_file_len = i16::from_be_bytes(next_file_length_byte);

    let mut table_name_length_byte = [0u8; 2];
    reader.read_exact(&mut table_name_length_byte).unwrap();
    let table_name_len = i16::from_be_bytes(table_name_length_byte);
    let table_name_len: usize = table_name_len
        .try_into()
        .expect("table name size is negative");

    let mut table_name_byte = vec![0u8; table_name_len];
    reader.read_exact(&mut table_name_byte).unwrap();
    let table_name = String::from_utf8(table_name_byte).unwrap();
    let cleaned_name = table_name.trim_matches('"');

    let table_width: usize = match usize::try_from(number_of_columns) {
        Ok(v) => v,
        Err(_) => panic!("table width overflow"),
    };

    //read column names
    let mut column_names: Vec<String> = vec![String::new(); table_width];
    for i in 0..table_width {
        let mut column_name_len_byte = [0u8; 2];
        reader.read_exact(&mut column_name_len_byte).unwrap();
        let column_name_len = i16::from_be_bytes(column_name_len_byte);
        let column_name_size: usize = column_name_len
            .try_into()
            .expect("table name length was negative");

        let mut column_name_byte = vec![0u8; column_name_size];
        reader.read_exact(&mut column_name_byte).unwrap();
        let table_name = String::from_utf8(column_name_byte).unwrap();
        column_names[i] = table_name.clone();
    }

    //read datatype definition
    let mut column_types: Vec<DataType> = vec![DataType::Undefined; table_width];
    for i in 0..table_width {
        let mut column_type_len_byte = [0u8; 2];
        reader.read_exact(&mut column_type_len_byte).unwrap();
        let column_type_len_byte = i16::from_be_bytes(column_type_len_byte);
        let column_name_size: usize = column_type_len_byte
            .try_into()
            .expect("datatype name length was negative");

        let mut column_type_byte = vec![0u8; column_name_size];
        reader.read_exact(&mut column_type_byte).unwrap();
        let column_type = String::from_utf8(column_type_byte).unwrap();
        let dt: DataType = datatype::to_datatype(&*column_type);
        column_types[i] = dt;
    }

    let mut tree: BPlusTree<i64, Vec<DataType>, 3> = BPlusTree::default();
    let mut rows: Vec<Vec<DataType>> = Vec::new();

    'read_rows: loop {
        let mut row: Vec<DataType> = Vec::with_capacity(table_width);

        for i in 0..table_width {
            let dt = &column_types[i];

            // Helper macro to turn EOF into "stop reading rows"
            macro_rules! read_or_eof {
            ($buf:expr) => {
                match reader.read_exact(&mut $buf) {
                    Ok(()) => {}
                    Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                        if row.is_empty() {
                            break 'read_rows;
                        } else {
                            println!("EOF mid row");
                        }
                    }
                    Err(e) => println!("I/O error while reading row: {e}") ,
                }
            };
        }

            match dt {
                DataType::BigInt { .. } => {
                    let mut buf = [0u8; 8];
                    read_or_eof!(buf);
                    row.push(DataType::BigInt {
                        x: i64::from_be_bytes(buf),
                    });
                }

                DataType::Int { .. } => {
                    let mut buf = [0u8; 4];
                    read_or_eof!(buf);
                    row.push(DataType::Int {
                        x: i32::from_be_bytes(buf),
                    });
                }

                DataType::SmallInt { .. } => {
                    let mut buf = [0u8; 2];
                    read_or_eof!(buf);
                    row.push(DataType::SmallInt {
                        x: i16::from_be_bytes(buf),
                    });
                }

                DataType::TinyInt { .. } => {
                    let mut buf = [0u8; 1];
                    read_or_eof!(buf);
                    row.push(DataType::TinyInt {
                        x: i8::from_be_bytes(buf),
                    });
                }

                DataType::Decimal { .. } => {
                    let mut buf = [0u8; 4];
                    read_or_eof!(buf);
                    row.push(DataType::Decimal {
                        x: f32::from_be_bytes(buf),
                    });
                }

                DataType::VarChar { .. } => {
                    let mut len_buf = [0u8; 2];
                    read_or_eof!(len_buf);

                    let varchar_len = i16::from_be_bytes(len_buf);
                    let varchar_size: usize =
                        varchar_len.try_into().expect("varchar length was negative");

                    let mut data = vec![0u8; varchar_size];
                    // can't use macro directly because it expects an array; do the same logic:
                    match reader.read_exact(&mut data) {
                        Ok(()) => {}
                        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                            println!("Corrupted file");
                        }
                        Err(e) => println!("I/O error while reading varchar: {e}"),
                    }

                    let s = String::from_utf8(data).unwrap();
                    row.push(DataType::VarChar {
                        x: s,
                        y: varchar_size,
                    });
                }

                DataType::Undefined => {
                    println!("Column type Undefined in schema; cannot decode rows.");
                }
                other => {
                    println!("Decoding not implemented for datatype: {:?}", std::mem::discriminant(other));
                }
            }
        }

        // Successfully read a full row
        let id: i64 = row[0].as_i64().expect("row[0] needs to be a BigInt");
        tree.insert(id, row.clone());
        rows.push(row);
    }
    let mut table: Table = Table::new_empty();
    table.set_table_name(String::from(cleaned_name));
    table.set_uuid(Uuid::default());
    table.column_names = column_names;
    table.column_types = column_types;
    table.tree = tree;

    let duration = start.elapsed();
    println!("Total time taken: {:?}", duration);
    table
}

#[derive(Clone)]
pub struct Table {
    table_name: String,
    tree: BPlusTree<i64, Vec<DataType>, 3>,
    uuid: Uuid,
    column_names: Vec<String>,
    column_types: Vec<DataType>,
}

impl Table {
    pub fn new_empty() -> Self {
        Table {
            table_name: "".to_string(),
            tree: Default::default(),
            uuid: Default::default(),
            column_names: vec![],
            column_types: vec![],
        }
    }

    /// creates a new table with the params:
    /// - new() Constructor
    /// - get_table_name() - returns the human-readable name of teh table
    /// - get_bptree() - returns the tree
    /// - get_uuid() gets the uuid of the table///
    pub fn new(
        table_name: String,
        tree: BPlusTree<i64, Vec<DataType>, 3>,
        uuid: Uuid,
        names: Vec<String>,
        types: Vec<DataType>,
    ) -> Table {
        assert!(names.len() > 0);
        if names.len() != types.len() {
            print!("names length mismatch - unable to create such a mess");
        }
        if names[0].eq("ID") || names[0].eq("id") || names[0].eq("Id") {
            println!("first column needs to be an column named id | ID || Id")
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
    pub fn set_table_name(&mut self, table_name: String) {
        self.table_name = table_name;
    }

    /// returns the B+Tree
    pub fn get_bptree(&self) -> &bptree::BPlusTree<i64, Vec<DataType>, 3> {
        &self.tree
    }

    /// if the tree has been changed or the tree has been loaded from disc
    pub fn set_bptree(&mut self, tree: BPlusTree<i64, Vec<DataType>, 3>) {
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
}

#[cfg(test)]
mod tests {
    use super::{Table, read_table_from_disc, save_table_to_disc};
    use crate::bptree::BPlusTree;
    use crate::table::datatype::DataType;
    use uuid::Uuid;

    #[test]
    fn create_new_table() {
        let names: Vec<String> = vec![
            String::from("id"),
            String::from("first_name"),
            String::from("last_name"),
            String::from("age"),
        ];
        let types: Vec<DataType> = vec![
            DataType::BigInt { x: 0 },
            DataType::VarChar {
                x: String::from(" "),
                y: 0,
            },
            DataType::VarChar {
                x: String::from(" "),
                y: 0,
            },
            DataType::Int { x: 50 },
        ];
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
        read_table_from_disc(
            String::from("C:/temp/moi/0e6bce68-99fa-3841-b790-24afbdf7db1d.moi"),
            Uuid::parse_str("0e6bce68-99fa-3841-b790-24afbdf7db1d").unwrap(),
        );
    }


    #[test]
    fn write_to_disc(){
        let table: Table =         read_table_from_disc(
            String::from("C:/temp/moi/0e6bce68-99fa-3841-b790-24afbdf7db1d.moi"),
            Uuid::parse_str("0e6bce68-99fa-3841-b790-24afbdf7db1d").unwrap(),
        );
        save_table_to_disc(&table,
                           &String::from("C:/temp/moi/0e6bce68-99fa-3841-b790-24afbdf7db1f.moi"),
                           &Uuid::parse_str("0e6bce68-99fa-3841-b790-24afbdf7db1f").unwrap()
        );
    }
}
