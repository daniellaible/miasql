use sqlparser::ast::{ColumnOption, CreateTable, Ident, ObjectNamePart, Statement, TableConstraint};
use crate::command::constraint::Constraint;
use crate::command::sqlcommands::SqlCommand;


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

     let mut columns: Vec<(String, String, Vec<Constraint>)> = Vec::new();
     for col_def in iter_columns {
         let name = col_def.name.value.to_string();
         let data_type = col_def.data_type.to_string();

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
         let column_def:(String, String
                         , Vec<Constraint>) = (name, data_type, column_constraints);
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

        let result = match ast.into_iter().next().unwrap() {
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
        let result = match ast.into_iter().next().unwrap() {
            Statement::CreateTable(create) => parse(create),
            _ => panic!("expected query"),
        };
    }
}