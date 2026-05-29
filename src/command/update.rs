use sqlparser::ast::Update;
use crate::command::sqlcommands::SqlCommand;


#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSet {
    pub column: String,
    pub value: String,
}

pub fn parse(update: Update) -> SqlCommand {


    SqlCommand::UNDEFINED
}

#[cfg(test)]
mod tests {
    use sqlparser::ast::Statement;
    use sqlparser::parser::Parser;
    use crate::command::update::parse;
    use crate::command::sqlcommands::SqlCommand;
    use sqlparser::dialect::GenericDialect;
    use crate::command::sqloperator::Operator;
    use crate::database::datatype;

    fn parse_select(statement: &str) -> SqlCommand {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, statement).unwrap();

        match ast.into_iter().next().unwrap() {
            Statement::Update(update) => parse(update),
            _ => panic!("expected query"),
        }
    }

    #[test]
    fn test_update(){
        let command = parse_select(
            "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt' WHERE CustomerID = 1;"
        );
        println!("command: {:?}", command);
        match command {
            SqlCommand::UPDATE {
                command ,
                sets,
                where_clause

            } => {
                assert_eq!(command, "UPDATE");
            }
            _ => panic!("expected SELECT"),
        }
    }

    #[test]
    fn test_update_second(){
        let command = parse_select(
            "UPDATE Customers SET ContactName = 'Alfred Schmidt' WHERE CustomerID = 1;"
        );
        println!("command: {:?}", command);
        match command {
            SqlCommand::UPDATE {
                command ,
                sets,
                where_clause

            } => {
                assert_eq!(command, "UPDATE");
            }
            _ => panic!("expected SELECT"),
        }
    }


    #[test]
    fn test_update_without_where(){
        let command = parse_select(
            "UPDATE Customers SET ContactName = 'Alfred Schmidt'"
        );
        println!("command: {:?}", command);
        match command {
            SqlCommand::UPDATE {
                command ,
                sets,
                where_clause

            } => {
                assert_eq!(command, "UPDATE");
            }
            _ => panic!("expected SELECT"),
        }
    }


}
