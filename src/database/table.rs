use crate::command::constraint::Constraint;
use crate::command::createtable::ForeignKeyToken;
use crate::command::sqlcommands::SqlCommand;
use crate::database::bptree::BPlusTree;
use crate::database::datatype::DataType;
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionContext;
use std::error::Error;
use std::{fmt};
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
    pub column_names: Vec<String>,
    pub column_types: Vec<DataType>,
    pub constraint: Vec<(u32, Constraint)>,
    pub foreign_keys: Vec<ForeignKeyToken>,
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
            foreign_keys: vec![],
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
        column_types: Vec<DataType>,
        constraint: Vec<(u32, Constraint)>,
        foreign_keys: Vec<ForeignKeyToken>,
    ) -> Table {
        // todo check if there are duplicates in the names

        Table {
            max_id,
            db_name,
            table_name,
            tree,
            uuid,
            mtd_path,
            column_names,
            column_types,
            constraint,
            foreign_keys,
        }
    }

    pub fn inc_max_id(&mut self) -> i64 {
        self.max_id += 1;
        self.max_id
    }
}

pub fn update_table(mut tp: TransactionContext) -> anyhow::Result<TransactionContext> {

    match tp.command.clone() {
        //here we create a new table in memory
        SqlCommand::CreateTable {
            table,
            columns,
            foreign_keys,
            ..
        } => {
            let column_names = parse_to_names(columns.clone());
            let datatypes: Vec<DataType> = parse_to_datatypes(columns.clone());
            let constraints: Vec<(u32, Constraint)> = parse_to_constraints(columns.clone());

            let tree: BPlusTree<i64, Vec<DataType>, 3> = BPlusTree::default();
            let table = Table::new(
                0,
                tp.db_name.clone(),
                table,
                tree,
                tp.table_uuid,
                "".to_string(),
                column_names,
                datatypes,
                constraints,
                foreign_keys,
            );
            DbMem::add_table(table);
        }
        _ => {
        }
    }
    tp.is_btree_updated = true;
    Ok(tp)
}

fn parse_to_constraints(columns: Vec<(String, DataType, Vec<Constraint>)>) -> Vec<(u32, Constraint)> {
    let mut result:Vec<(u32, Constraint)> = vec![];

    for i in 0.. columns.len(){
        let column = columns[i].clone();
        let constraints = column.2;

        for j in 0 .. constraints.len(){
            let constraint:(u32, Constraint) = (i as u32, constraints[j].clone());
            result.push(constraint);
        }
    }
    result
}

fn parse_to_datatypes(columns: Vec<(String, DataType, Vec<Constraint>)>) -> Vec<DataType> {
    let mut result:Vec<DataType> = vec![];
    for i in 0.. columns.len(){
        let column = columns[i].clone();
        result.push(column.1);
    }
    result
}

fn parse_to_names(columns: Vec<(String, DataType, Vec<Constraint>)>) -> Vec<String> {
    let mut result:Vec<String> = vec![];
    for i in 0.. columns.len(){
        let column = columns[i].clone();
        result.push(column.0);
    }
    result
}

#[cfg(test)]
mod tests {}
