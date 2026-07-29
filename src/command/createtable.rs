use std::sync::Arc;
use anyhow::{anyhow, Error};
use sqlparser::ast::{ColumnOption, CreateTable, Ident, ObjectNamePart, TableConstraint};
use uuid::Uuid;
use crate::command::constraint::Constraint;
use crate::command::sqlcommands::SqlCommand;
use crate::database::datatype::DataType;
use crate::database::table;
use crate::database::table::Row;
use crate::{file};
use crate::file::{moihandler, mtdhandler};
use crate::file::moihandler::create_moi_file;
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionContext;

#[derive(Debug, Clone, PartialEq)]
pub struct ForeignKeyToken {
    pub name: Option<String>,
    pub columns: Vec<String>,
    pub foreign_table: String,
    pub referred_columns: Vec<String>,
}

pub fn parse(create: CreateTable) -> SqlCommand {
    let tablename = create.name.to_string();
    let iter_columns = create.columns.iter();

     let mut columns: Vec<(String, DataType, Vec<Constraint>)> = Vec::new();
     for col_def in iter_columns {
         let name = col_def.name.value.to_string();
         let data_type:DataType = match col_def.data_type{
             sqlparser::ast::DataType::Varchar(_) => {
                 DataType::VarChar(0, "".to_string())
             },
             sqlparser::ast::DataType::Decimal(_) => {
                 DataType::Decimal(0 as f32)
             }
             sqlparser::ast::DataType::Float(_) => {
                 DataType::Float(0 as f64)
             }
             sqlparser::ast::DataType::TinyInt(_) => {
                 DataType::TinyInt(0)
             }
             sqlparser::ast::DataType::SmallInt(_) => {
                 DataType::SmallInt(0)
             }
             sqlparser::ast::DataType::Int(_) => {
                 DataType::Int(0)
             }
             sqlparser::ast::DataType::BigInt(_) => {
                 DataType::BigInt(0)
             }
             sqlparser::ast::DataType::Bool => {
                 DataType::Bool(false)
             }
             sqlparser::ast::DataType::Date => {
                 DataType::Date(0)
             }
             sqlparser::ast::DataType::Time(_, _) => {
                 DataType::Time(0)
             }
             sqlparser::ast::DataType::Datetime(_) => {
                 DataType::DateTime(0)
             }
             _ => {DataType::Undefined}
         };

         let iter_options = col_def.options.iter();
         let mut column_constraints:Vec<Constraint> = Vec::new();
         for option in iter_options {
             let column_opt = &option.option;
             let constraint:Constraint = match column_opt {
                 ColumnOption::NotNull => Constraint::NotNull,
                 ColumnOption::Unique(_) => Constraint::Unique,
                 ColumnOption::PrimaryKey(_) => Constraint::PrimaryKey,
                 ColumnOption::Default(_) => Constraint::Default,
                 ColumnOption::Check(_) => Constraint::Check,
                 _ => Constraint::Undefined,
             };
             column_constraints.push(constraint.clone());
         }
         let column_def:(String, DataType, Vec<Constraint>) = (name, data_type, column_constraints);
         columns.push(column_def);
     }

    let foreign_keys = extract_foreign_keys(create);

    SqlCommand::CreateTable {
        command: String::from("CREATE TABLE"),
        table: String::from(tablename),
        columns,
        foreign_keys
    }
}

pub fn extract_foreign_keys(create_table: CreateTable) -> Vec<ForeignKeyToken> {
    let mut foreign_keys = Vec::new();

    for constraint in &create_table.constraints {
        if let TableConstraint::ForeignKey(fk) = constraint {
            let foreign_table = match &fk.foreign_table.0.as_slice() {
                [ObjectNamePart::Identifier(Ident { value, .. })] => value.clone(),
                parts => parts
                    .iter()
                    .filter_map(|part| match part {
                        ObjectNamePart::Identifier(ident) => Some(ident.value.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("."),
            };

            foreign_keys.push(ForeignKeyToken {
                name: fk.name.as_ref().map(|ident| ident.value.clone()),
                columns: fk.columns.iter().map(|ident| ident.value.clone()).collect(),
                foreign_table,
                referred_columns: fk
                    .referred_columns
                    .iter()
                    .map(|ident| ident.value.clone())
                    .collect(),
            });
        }
    }
    foreign_keys
}

pub fn create_table(mut transaction: TransactionContext, tablename: String, columns:Vec<(String, DataType, Vec<Constraint>)>) -> anyhow::Result<TransactionContext, Error>{
    let ledger_clone_file = transaction.clone();
    let ledger_result = file::ledgerhandler::append_to_file(
        &ledger_clone_file.user,
        &ledger_clone_file.command,
        &ledger_clone_file.db_name,
    );
    match ledger_result {
        Ok(_) => {
            let trans_clone_btree = transaction.clone();
            let btree_update_result = table::create_table_in_mem(trans_clone_btree);
            match btree_update_result {
                Ok(_) => {
                    transaction.is_btree_updated = true;

                    let last_id = moihandler::get_max_id("C:\\MiaSql\\system\\tables.moi");
                    transaction.row_id = last_id + 1;
                    let uuid = Uuid::new_v4();
                    transaction.table_uuid = uuid;

                    let tab_name = tablename.clone();
                    let table_name_mtd = tab_name.clone();
                    let system_table_result = update_system_table_in_mem(transaction.row_id, Arc::from(transaction.db_name.as_str()), Arc::from(tablename), transaction.table_uuid.to_string());
                    match system_table_result {
                        Ok(_) => {
                            transaction.is_system_table_updated= true;
                            let row = create_system_table_row(transaction.row_id, Arc::from(transaction.db_name.as_str()), Arc::from(tab_name), transaction.table_uuid.to_string());
                            let add_row_result = moihandler::add_row("C:\\MiaSql\\system\\tables.moi", row);
                            match add_row_result{
                                Ok(_) => {
                                    let create_mtd_file_result = mtdhandler::new_mtd_file(&transaction.db_name, &table_name_mtd, &columns, &vec![], transaction.table_uuid);
                                    match create_mtd_file_result {
                                        Ok(_) => {
                                            transaction.is_mtd_file_updated = true;
                                            let moi_path = "C:\\MiaSql\\tables\\".to_owned() + uuid.to_string().as_str() + ".moi";
                                            let moi_creating_result = create_moi_file(&moi_path);
                                            match moi_creating_result{
                                                Ok(_) => {
                                                    transaction.is_moi_file_updated = true;
                                                    Ok(transaction)

                                                }
                                                Err(_)=> {
                                                    transaction.error = true;
                                                    Err(anyhow!("unable to create mtd file"))
                                                }
                                            }
                                        }
                                        _ => {
                                            transaction.error = true;
                                            Err(anyhow!("unable to create mtd file"))
                                        }
                                    }
                                }
                                _ => {
                                    Err(anyhow!("unable to add row to system.tables"))
                                }
                            }
                        }
                        Err(why) => {
                            transaction.is_system_table_updated = false;
                            transaction.error = true;
                            Err(anyhow!("unable to update system table in memory:{}", why))
                        }
                    }
                }
                Err(why) => {
                    transaction.is_btree_updated = false;
                    transaction.error = true;
                    Err(anyhow!("unable to update tree because:{}", why))
                }
            }
        }
        Err(why) => {
            Err(anyhow!("unable to update ledger file because: {}", why))
        }
    }
}

pub fn update_system_table_in_mem(id: i64, db_name: Arc<str>, table_name:Arc<str>, table_uuid: String) -> anyhow::Result<()> {
    let row = create_system_table_row(id, db_name, table_name, table_uuid);
    DbMem::insert_row("system", "tables", row);
    Ok(())
}

fn create_system_table_row(id: i64,db_name: Arc<str>, table_name:Arc<str>, tableuuid: String) -> Row {
    let mut row: Row = Row{
        data: Vec::new(),
    };
    let db = &*db_name;
    let table = &*table_name;

    let mut location = "C:\\MiaSql\\tables\\".to_owned();
    location = location + tableuuid.as_str();
    location = location + ".mtd";

    row.data.push(DataType::BigInt(id));
    row.data.push(DataType::VarChar(db.len() as u8, String::from(db)));
    row.data.push(DataType::VarChar(table.len() as u8, String::from(table)));
    row.data.push(DataType::VarChar(location.len() as u8, String::from(location)));
    row
}


#[cfg(test)]
mod tests {
    use sqlparser::ast::Statement;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use crate::command::createtable::parse;

    #[test]
    fn working_test_create_table(){
        let dialect = GenericDialect {};

        let command: &str = "CREATE TABLE Persons ( PersonID BigInt PRIMARY KEY, LastName VarChar(255) NOT NULL UNIQUE, FirstName VarChar(255), Address VarChar(255), City VarChar(255));";
        let ast = Parser::parse_sql(&dialect, command).unwrap();

         match ast.into_iter().next().unwrap() {
            Statement::CreateTable(create) => parse(create),
            _ => panic!("expected query"),
        };
    }

    #[test]
    fn working_test_create_foreign_key_table(){
        let dialect = GenericDialect {};

        let command: &str = "CREATE TABLE Orders (  OrderID int PRIMARY KEY,  OrderNumber int NOT NULL, PersonID int, CONSTRAINT fk_Person FOREIGN KEY (PersonID) REFERENCES Persons(PersonID));";
        let ast = Parser::parse_sql(&dialect, command).unwrap();
        println!("{:?}", ast[0]);
        match ast.into_iter().next().unwrap() {
            Statement::CreateTable(create) => parse(create),
            _ => panic!("expected query"),
        };
    }
}