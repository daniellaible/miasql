use sqlparser::ast::AlterTable;
use crate::command::sqlcommands::SqlCommand;

pub fn parse(alter: AlterTable) -> SqlCommand {
    
    SqlCommand::UNDEFINED
}

#[cfg(test)]
mod tests {
    use sqlparser::ast::Statement;
    use sqlparser::parser::Parser;
    use crate::command::alter::parse;
    use crate::command::sqlcommands::SqlCommand;
    use sqlparser::dialect::GenericDialect;
    use crate::command::constraint::Constraint;
    use crate::database::datatype::DataType;

    fn parse_alter(statement: &str) -> SqlCommand {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, statement).unwrap();

        match ast.into_iter().next().unwrap() {
            Statement::AlterTable(alter) => parse(alter),
            _ => panic!("expected query"),
        }
    }

    #[test]
    fn test_alter_add_table() {
        let command = crate::command::alter::tests::parse_alter(
            "ALTER TABLE Customers ADD Email varchar(255) NOT NULL;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN {
                command,
                table,
                columns,
                
            } => {
                assert_eq!(command, "ALTER ADD COLUMN");
                assert_eq!(table, "Customers");
                assert_eq!(columns.len(), 1);
                assert_eq!(columns[0], (String::from("Email"), DataType::VarChar{x: String::new(),y:255}, vec![Constraint::NOT_NULL]));
            }
            _ => panic!("expected INSERT"),
        }
    }

    #[test]
    fn test_alter_table_drop_column() {
        let command = crate::command::alter::tests::parse_alter(
            "ALTER TABLE Customers DROP COLUMN Email;",
        );

        match command {
            SqlCommand::ALTER_DROP_COLUMN {
                command,
                table,
                columns,

            } => {
                assert_eq!(command, "ALTER DROP COLUMN");
                assert_eq!(table, "Customers");
                assert_eq!(columns.len(), 1);
                assert_eq!(columns[0], String::from("Email"));
            }
            _ => panic!("expected INSERT"),
        }
    }

    #[test]
    fn test_alter_table_rename_column() {
        let command = crate::command::alter::tests::parse_alter(
            "ALTER TABLE Workforce RENAME COLUMN Worker TO Employee;",
        );

        match command {
            SqlCommand::ALTER_RENAME_COLUMN {
                command,
                table,
                old,
                new

            } => {
                assert_eq!(command, "ALTER RENAME COLUMN");
                assert_eq!(table, "Workforce");
                assert_eq!(old, String::from("Worker"));
                assert_eq!(new, String::from("Employee"));
            }
            _ => panic!("expected INSERT"),
        }
    }

    #[test]
    fn test_alter_table_modify_column() {
        let command = crate::command::alter::tests::parse_alter(
            "ALTER TABLE Customers MODIFY Email varchar(100) NOT NULL;",
        );

        match command {
            SqlCommand::ALTER_MODIFY_COLUMN {
                command,
                table,
                column,
                data_type,
                constraints

            } => {
                assert_eq!(command, "ALTER MODIFY COLUMN");
                assert_eq!(table, "Customers");
                assert_eq!(column, String::from("Email"));
                assert_eq!(data_type, DataType::VarChar{x: String::new(),y:100});
                assert_eq!(constraints[0], Constraint::NOT_NULL );
            }
            _ => panic!("expected INSERT"),
        }
    }

    #[test]
    fn test_alter_table_rename_table() {
        let command = crate::command::alter::tests::parse_alter(
            "ALTER TABLE Customers RENAME TO Clients;",
        );

        match command {
            SqlCommand::ALTER_TABLE_RENAME {
                command,
                table,
                new_name,

            } => {
                assert_eq!(command, "ALTER RENAME TABLE");
                assert_eq!(table, "Customers");
                assert_eq!(new_name, String::from("Clients"));
            }
            _ => panic!("expected INSERT"),
        }
    }

}