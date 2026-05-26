use sqlparser::ast::{ColumnOption, CreateTable};
use crate::command::constraint::Constraint;
use crate::command::sqlcommands::SqlCommand;
use crate::database::datatype::DataType;

pub fn parse(create: CreateTable) -> SqlCommand {
    //println!("create: {:?}", create);

    let tablename = create.name.to_string();
     let iter_columns = create.columns.iter();

     let mut columns: Vec<(String, String, Vec<Constraint>)> = Vec::new();
     for col_def in iter_columns {
         //println!("column: {:?}", col_def);

         let name = col_def.name.value.to_string();
         println!("column_name: {:?}", name);

         let dataType = col_def.data_type.to_string();

         println!("dataType: {:?}", dataType);

         let iter_options = col_def.options.iter();
         let mut column_constraints:Vec<Constraint> = Vec::new();
         for option in iter_options {
             //println!("option: {:?}", option.option);
             let column_opt = &option.option;
             let constraint:Constraint = match column_opt {
                 ColumnOption::NotNull => Constraint::NOT_NULL,
                 ColumnOption::Unique(_) => Constraint::UNIQUE,
                 ColumnOption::PrimaryKey(_) => Constraint::PRIMARY_KEY,
                 _ => Constraint::UNDEFINED,
             };
             column_constraints.push(constraint.clone());
             println!("constraint: {:?}", constraint);

         }
         let column_def:(String, String
                         , Vec<Constraint>) = (name, dataType, column_constraints);
         columns.push(column_def);
         println!("----");
     }
    SqlCommand::CREATE_TABLE {
        command: String::from("CREATE TABLE"),
        table: String::from(tablename),
        columns
    }
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
        println!("{:?}", result);
    }

    #[test]
    fn working_test_create_foreign_key_table(){
        let dialect = GenericDialect {};

        let command: &str = "CREATE TABLE Orders (  OrderID int PRIMARY KEY,  OrderNumber int NOT NULL, PersonID int, CONSTRAINT fk_Person FOREIGN KEY (PersonID) REFERENCES Persons(PersonID));";
        let ast = Parser::parse_sql(&dialect, command).unwrap();

        match ast.into_iter().next().unwrap() {
            Statement::CreateTable(create) => parse(create),
            _ => panic!("expected query"),
        };
    }
}