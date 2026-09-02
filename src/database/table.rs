use crate::command::constraint::Constraint;
use crate::command::createtable::ForeignKeyToken;
use crate::database::datatype::DataType;
use std::{fmt};
use crate::database::mapstructure::HashmapStructure;
use crate::database::memstruct::MemoryStructure;

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub data: Vec<DataType>,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub max_id: i64,
    pub db_name: String,
    pub table_name: String,
    pub mtd_path:String,
    pub data: HashmapStructure,
    pub index_structures: Vec<Box<dyn MemoryStructure>>,
    pub column_names: Vec<String>,
    pub column_types: Vec<DataType>,
    pub constraint: Vec<(u32, Constraint)>,
    pub foreign_keys: Vec<ForeignKeyToken>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Something went wrong with table: {}", self.table_name)
    }
}

impl Table {
    pub fn new(
        max_id: i64,
        db_name: String,
        table_name: String,
        mtd_path:String,
        data: HashmapStructure,
        index_structures: Vec<Box<dyn MemoryStructure>>,

        column_names: Vec<String>,
        column_types: Vec<DataType>,
        constraint: Vec<(u32, Constraint)>,
        foreign_keys: Vec<ForeignKeyToken>,
    ) -> Self {
        Self {
            max_id,
            db_name,
            table_name,
            mtd_path,
            data,
            index_structures,
            column_names,
            column_types,
            constraint,
            foreign_keys,
        }
    }

    pub fn add_index_structure<T: MemoryStructure + 'static>(&mut self, s: T) {
        self.index_structures.push(Box::new(s));
    }

    pub fn inc_max_id(&mut self) -> i64 {
        self.max_id += 1;
        self.max_id
    }
}


/*pub fn create_table_in_mem(mut tp: TransactionContext) -> anyhow::Result<TransactionContext> {
    match tp.command.clone() {
        SqlCommand::CreateTable {
            table,
            columns,
            foreign_keys,
            ..
        } => {
            let column_names = parse_to_names(columns.clone());
            let datatypes: Vec<DataType> = parse_to_datatypes(columns.clone());
            let constraints: Vec<(u32, Constraint)> = parse_to_constraints(columns.clone());


            let table = Table::new(
                0,
                tp.db_name.clone(),
                table,
                HashmapStructure::create(),
                vec![],
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
}*/

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
