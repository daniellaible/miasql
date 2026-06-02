use crate::command::constraint::Constraint;
use crate::command::sqlcommands::SqlCommand;
use crate::database::datatype::DataType;
use sqlparser::ast::{
    AlterTable, AlterTableOperation, ColumnOption, DataType as SqlDataType, ObjectName,
    ObjectNamePart, RenameTableNameKind,
};

pub fn parse(alter: AlterTable) -> SqlCommand {
    let table = parse_object_name(&alter.name);

    if alter.operations.len() != 1 {
        return SqlCommand::UNDEFINED;
    }

    match &alter.operations[0] {
        AlterTableOperation::AddColumn { column_def, .. } => {
            let column_name = column_def.name.value.clone();
            let data_type = parse_data_type(&column_def.data_type);
            let constraints = parse_column_constraints(&column_def.options);

            SqlCommand::ALTER_ADD_COLUMN {
                command: String::from("ALTER ADD COLUMN"),
                table,
                columns: vec![(column_name, data_type, constraints)],
            }
        }

        AlterTableOperation::DropColumn { column_names, .. } => {
            let columns = column_names
                .iter()
                .map(|ident| ident.value.clone())
                .collect();

            SqlCommand::ALTER_DROP_COLUMN {
                command: String::from("ALTER DROP COLUMN"),
                table,
                columns,
            }
        }

        AlterTableOperation::RenameColumn {
            old_column_name,
            new_column_name,
        } => SqlCommand::ALTER_RENAME_COLUMN {
            command: String::from("ALTER RENAME COLUMN"),
            table,
            old: old_column_name.value.clone(),
            new: new_column_name.value.clone(),
        },

        AlterTableOperation::ModifyColumn {
            col_name,
            data_type,
            options,
            ..
        } => SqlCommand::ALTER_MODIFY_COLUMN {
            command: String::from("ALTER MODIFY COLUMN"),
            table,
            column: col_name.value.clone(),
            data_type: parse_data_type(data_type),
            constraints: parse_column_option_vec(options),
        },

        AlterTableOperation::RenameTable { table_name } => SqlCommand::ALTER_TABLE_RENAME {
            command: String::from("ALTER RENAME TABLE"),
            table,
            new_name: parse_rename_table_name(table_name),
        },

        _ => SqlCommand::UNDEFINED,
    }
}

fn parse_rename_table_name(name: &RenameTableNameKind) -> String {
    match name {
        RenameTableNameKind::To(object_name) => parse_object_name(object_name),
        RenameTableNameKind::As(object_name) => parse_object_name(object_name),
    }
}

fn parse_object_name(name: &ObjectName) -> String {
    name.0
        .iter()
        .map(|part| match part {
            ObjectNamePart::Identifier(ident) => ident.value.clone(),
            _ => part.to_string(),
        })
        .collect::<Vec<_>>()
        .join(".")
}

fn parse_character_length(length: &sqlparser::ast::CharacterLength) -> usize {
    match length {
        sqlparser::ast::CharacterLength::IntegerLength { length, .. } => *length as usize,
        _ => 0,
    }
}

fn parse_data_type(data_type: &SqlDataType) -> DataType {
    match data_type {
        SqlDataType::Varchar(Some(length)) => DataType::VarChar {
            x: String::new(),
            y: parse_character_length(length),
        },
        SqlDataType::Varchar(None) => DataType::VarChar {
            x: String::new(),
            y: 0,
        },
        SqlDataType::CharVarying(Some(length)) => DataType::VarChar {
            x: String::new(),
            y: parse_character_length(length),
        },
        SqlDataType::CharVarying(None) => DataType::VarChar {
            x: String::new(),
            y: 0,
        },
        SqlDataType::Int(_) | SqlDataType::Integer(_) => DataType::Int { x: 0 },
        SqlDataType::BigInt(_) => DataType::BigInt { x: 0 },
        SqlDataType::SmallInt(_) => DataType::SmallInt { x: 0 },
        SqlDataType::TinyInt(_) => DataType::TinyInt { x: 0 },
        SqlDataType::Boolean => DataType::Bool { x: false },
        SqlDataType::Float(_) => DataType::Float { x: 0.0 },
        SqlDataType::Double(_) => DataType::Float { x: 0.0 },
        SqlDataType::Real => DataType::Float { x: 0.0 },
        _ => DataType::Undefined,
    }
}

fn parse_column_constraints(options: &[sqlparser::ast::ColumnOptionDef]) -> Vec<Constraint> {
    let mut constraints = vec![];

    for option in options {
        match &option.option {
            ColumnOption::NotNull => constraints.push(Constraint::NOT_NULL),
            ColumnOption::Unique(_) => constraints.push(Constraint::UNIQUE),
            _ => {}
        }
    }

    constraints
}

fn parse_column_option_vec(options: &[ColumnOption]) -> Vec<Constraint> {
    let mut constraints = vec![];

    for option in options {
        match option {
            ColumnOption::NotNull => constraints.push(Constraint::NOT_NULL),
            ColumnOption::Unique(_) => constraints.push(Constraint::UNIQUE),
            _ => {}
        }
    }

    constraints
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
                assert_eq!(constraints[0], Constraint::NOT_NULL);
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

    #[test]
    fn test_alter_add_table1() {
        let command = parse_alter(
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
                assert_eq!(
                    columns[0],
                    (
                        String::from("Email"),
                        DataType::VarChar {
                            x: String::new(),
                            y: 255
                        },
                        vec![Constraint::NOT_NULL]
                    )
                );
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_add_column_without_constraints() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD Email varchar(255);",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { table, columns, .. } => {
                assert_eq!(table, "Customers");
                assert_eq!(columns.len(), 1);
                assert_eq!(
                    columns[0],
                    (
                        String::from("Email"),
                        DataType::VarChar {
                            x: String::new(),
                            y: 255
                        },
                        vec![]
                    )
                );
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_add_column_with_unique_constraint() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD Email varchar(255) UNIQUE;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { table, columns, .. } => {
                assert_eq!(table, "Customers");
                assert_eq!(columns.len(), 1);
                assert_eq!(columns[0].0, "Email");
                assert_eq!(
                    columns[0].1,
                    DataType::VarChar {
                        x: String::new(),
                        y: 255
                    }
                );
                assert_eq!(columns[0].2, vec![Constraint::UNIQUE]);
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_add_column_int() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD Age INT;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { table, columns, .. } => {
                assert_eq!(table, "Customers");
                assert_eq!(columns.len(), 1);
                assert_eq!(columns[0].0, "Age");
                assert_eq!(columns[0].1, DataType::Int { x: 0 });
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_add_column_bigint() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD CustomerId BIGINT;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { table, columns, .. } => {
                assert_eq!(table, "Customers");
                assert_eq!(columns.len(), 1);
                assert_eq!(columns[0].0, "CustomerId");
                assert_eq!(columns[0].1, DataType::BigInt { x: 0 });
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_add_column_smallint() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD Rating SMALLINT;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { table, columns, .. } => {
                assert_eq!(table, "Customers");
                assert_eq!(table, "Customers");
                assert_eq!(columns[0].0, "Rating");
                assert_eq!(columns[0].1, DataType::SmallInt { x: 0 });
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_add_column_tinyint() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD Score TINYINT;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { columns, .. } => {
                assert_eq!(columns[0].0, "Score");
                assert_eq!(columns[0].1, DataType::TinyInt { x: 0 });
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_add_column_boolean() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD Active BOOLEAN;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { columns, .. } => {
                assert_eq!(columns[0].0, "Active");
                assert_eq!(columns[0].1, DataType::Bool { x: false });
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_add_column_float() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD Score FLOAT;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { columns, .. } => {
                assert_eq!(columns[0].0, "Score");
                assert_eq!(columns[0].1, DataType::Float { x: 0.0 });
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_add_column_real() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD Score REAL;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { columns, .. } => {
                assert_eq!(columns[0].0, "Score");
                assert_eq!(columns[0].1, DataType::Float { x: 0.0 });
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_add_column_double() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD Score DOUBLE;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { columns, .. } => {
                assert_eq!(columns[0].0, "Score");
                assert_eq!(columns[0].1, DataType::Float { x: 0.0 });
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_drop_column1() {
        let command = parse_alter(
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
            _ => panic!("expected ALTER_DROP_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_drop_column_uppercase() {
        let command = parse_alter(
            "ALTER TABLE CUSTOMERS DROP COLUMN EMAIL;",
        );

        match command {
            SqlCommand::ALTER_DROP_COLUMN {
                command,
                table,
                columns,
            } => {
                assert_eq!(command, "ALTER DROP COLUMN");
                assert_eq!(table, "CUSTOMERS");
                assert_eq!(columns.len(), 1);
                assert_eq!(columns[0], String::from("EMAIL"));
            }
            _ => panic!("expected ALTER_DROP_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_rename_column3() {
        let command = parse_alter(
            "ALTER TABLE Workforce RENAME COLUMN Worker TO Employee;",
        );

        match command {
            SqlCommand::ALTER_RENAME_COLUMN {
                command,
                table,
                old,
                new,
            } => {
                assert_eq!(command, "ALTER RENAME COLUMN");
                assert_eq!(table, "Workforce");
                assert_eq!(old, String::from("Worker"));
                assert_eq!(new, String::from("Employee"));
            }
            _ => panic!("expected ALTER_RENAME_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_rename_column_uppercase() {
        let command = parse_alter(
            "ALTER TABLE WORKFORCE RENAME COLUMN WORKER TO EMPLOYEE;",
        );

        match command {
            SqlCommand::ALTER_RENAME_COLUMN {
                command,
                table,
                old,
                new,
            } => {
                assert_eq!(command, "ALTER RENAME COLUMN");
                assert_eq!(table, "WORKFORCE");
                assert_eq!(old, String::from("WORKER"));
                assert_eq!(new, String::from("EMPLOYEE"));
            }
            _ => panic!("expected ALTER_RENAME_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_modify_column2() {
        let command = parse_alter(
            "ALTER TABLE Customers MODIFY Email varchar(100) NOT NULL;",
        );

        match command {
            SqlCommand::ALTER_MODIFY_COLUMN {
                command,
                table,
                column,
                data_type,
                constraints,
            } => {
                assert_eq!(command, "ALTER MODIFY COLUMN");
                assert_eq!(table, "Customers");
                assert_eq!(column, String::from("Email"));
                assert_eq!(
                    data_type,
                    DataType::VarChar {
                        x: String::new(),
                        y: 100
                    }
                );
                assert_eq!(constraints[0], Constraint::NOT_NULL);
            }
            _ => panic!("expected ALTER_MODIFY_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_modify_column_without_constraints() {
        let command = parse_alter(
            "ALTER TABLE Customers MODIFY Email varchar(100);",
        );

        match command {
            SqlCommand::ALTER_MODIFY_COLUMN {
                table,
                column,
                data_type,
                constraints,
                ..
            } => {
                assert_eq!(table, "Customers");
                assert_eq!(column, String::from("Email"));
                assert_eq!(
                    data_type,
                    DataType::VarChar {
                        x: String::new(),
                        y: 100
                    }
                );
                assert!(constraints.is_empty());
            }
            _ => panic!("expected ALTER_MODIFY_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_modify_column_unique() {
        let command = parse_alter(
            "ALTER TABLE Customers MODIFY Email varchar(100) UNIQUE;",
        );

        match command {
            SqlCommand::ALTER_MODIFY_COLUMN {
                table,
                column,
                constraints,
                ..
            } => {
                assert_eq!(table, "Customers");
                assert_eq!(column, "Email");
                assert_eq!(constraints, vec![Constraint::UNIQUE]);
            }
            _ => panic!("expected ALTER_MODIFY_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_modify_column_int() {
        let command = parse_alter(
            "ALTER TABLE Customers MODIFY Age INT;",
        );

        match command {
            SqlCommand::ALTER_MODIFY_COLUMN {
                column,
                data_type,
                ..
            } => {
                assert_eq!(column, "Age");
                assert_eq!(data_type, DataType::Int { x: 0 });
            }
            _ => panic!("expected ALTER_MODIFY_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_modify_column_bigint() {
        let command = parse_alter(
            "ALTER TABLE Customers MODIFY Id BIGINT;",
        );

        match command {
            SqlCommand::ALTER_MODIFY_COLUMN {
                column,
                data_type,
                ..
            } => {
                assert_eq!(column, "Id");
                assert_eq!(data_type, DataType::BigInt { x: 0 });
            }
            _ => panic!("expected ALTER_MODIFY_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_modify_column_boolean() {
        let command = parse_alter(
            "ALTER TABLE Customers MODIFY Active BOOLEAN;",
        );

        match command {
            SqlCommand::ALTER_MODIFY_COLUMN {
                column,
                data_type,
                ..
            } => {
                assert_eq!(column, "Active");
                assert_eq!(data_type, DataType::Bool { x: false });
            }
            _ => panic!("expected ALTER_MODIFY_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_rename_table1() {
        let command = parse_alter(
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
            _ => panic!("expected ALTER_TABLE_RENAME"),
        }
    }

    #[test]
    fn test_alter_table_rename_table_uppercase() {
        let command = parse_alter(
            "ALTER TABLE CUSTOMERS RENAME TO CLIENTS;",
        );

        match command {
            SqlCommand::ALTER_TABLE_RENAME {
                command,
                table,
                new_name,
            } => {
                assert_eq!(command, "ALTER RENAME TABLE");
                assert_eq!(table, "CUSTOMERS");
                assert_eq!(new_name, String::from("CLIENTS"));
            }
            _ => panic!("expected ALTER_TABLE_RENAME"),
        }
    }

    #[test]
    fn test_alter_table_multiple_operations_returns_undefined() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD Email varchar(255), ADD City varchar(255);",
        );

        assert_eq!(command, SqlCommand::UNDEFINED);
    }

    #[test]
    fn test_alter_table_unsupported_operation_returns_undefined() {
        let command = parse_alter(
            "ALTER TABLE Customers ALTER COLUMN Email SET DEFAULT 'x';",
        );

        assert_eq!(command, SqlCommand::UNDEFINED);
    }

    #[test]
    fn test_alter_table_add_unknown_datatype_becomes_undefined_datatype() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD CreatedAt DATETIME;",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { columns, .. } => {
                assert_eq!(columns[0].0, "CreatedAt");
                assert_eq!(columns[0].1, DataType::Undefined);
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_modify_unknown_datatype_becomes_undefined_datatype() {
        let command = parse_alter(
            "ALTER TABLE Customers MODIFY CreatedAt DATETIME;",
        );

        match command {
            SqlCommand::ALTER_MODIFY_COLUMN {
                column,
                data_type,
                ..
            } => {
                assert_eq!(column, "CreatedAt");
                assert_eq!(data_type, DataType::Undefined);
            }
            _ => panic!("expected ALTER_MODIFY_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_add_preserves_identifier_case() {
        let command = parse_alter(
            "ALTER TABLE Customers ADD EmailAddress varchar(64);",
        );

        match command {
            SqlCommand::ALTER_ADD_COLUMN { table, columns, .. } => {
                assert_eq!(table, "Customers");
                assert_eq!(columns[0].0, "EmailAddress");
            }
            _ => panic!("expected ALTER_ADD_COLUMN"),
        }
    }

    #[test]
    fn test_alter_table_modify_preserves_identifier_case() {
        let command = parse_alter(
            "ALTER TABLE Customers MODIFY EmailAddress varchar(64);",
        );

        match command {
            SqlCommand::ALTER_MODIFY_COLUMN { table, column, .. } => {
                assert_eq!(table, "Customers");
                assert_eq!(column, "EmailAddress");
            }
            _ => panic!("expected ALTER_MODIFY_COLUMN"),
        }
    }

}