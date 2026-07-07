use sqlparser::ast::{ColumnOption, CreateTable, Ident, ObjectNamePart, TableConstraint};
use crate::command::constraint::Constraint;
use crate::command::sqlcommands::SqlCommand;
use crate::database::database::Database;
use crate::database::datatype::DataType;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedForeignKey {
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

pub fn extract_foreign_keys(create_table: CreateTable) -> Vec<ParsedForeignKey> {

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

            foreign_keys.push(ParsedForeignKey {
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