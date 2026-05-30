use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use sqlparser::ast::{Expr, Insert, SetExpr, TableObject, Value};

pub fn parse(insert: Insert) -> SqlCommand {
    let table = match parse_table(&insert.table) {
        Some(table) => table,
        None => return SqlCommand::UNDEFINED,
    };

    let columns = parse_columns(&insert.columns);

    let values = match parse_values(&insert) {
        Some(values) => values,
        None => return SqlCommand::UNDEFINED,
    };

    SqlCommand::INSERT {
        command: String::from("INSERT"),
        table,
        columns,
        values,
    }
}

fn parse_table(table: &TableObject) -> Option<String> {
    match table {
        TableObject::TableName(name) => Some(
            name.0
                .iter()
                .map(|part| part.to_string())
                .collect::<Vec<_>>()
                .join("."),
        ),
        _ => None,
    }
}

fn parse_columns(columns: &Vec<sqlparser::ast::ObjectName>) -> Vec<String> {
    let mut result = vec![];

    for column in columns {
        let name = column
            .0
            .iter()
            .map(|part| part.to_string())
            .collect::<Vec<_>>()
            .join(".");
        result.push(name);
    }

    result
}

fn parse_values(insert: &Insert) -> Option<Vec<Vec<String>>> {
    let source = insert.source.as_ref()?;

    match source.body.as_ref() {
        SetExpr::Values(values) => {
            let mut result: Vec<Vec<String>> = vec![];

            for row in values.rows.iter() {
                let mut parsed_row: Vec<String> = vec![];

                for expr in row.iter() {
                    let value = parse_expr_value(expr)?;
                    parsed_row.push(value);
                }

                result.push(parsed_row);
            }

            Some(result)
        }
        _ => None,
    }
}

fn parse_expr_value(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Value(vws) => match &vws.value {
            Value::SingleQuotedString(s) => Some(s.clone()),
            Value::DoubleQuotedString(s) => Some(s.clone()),
            Value::Number(n, _) => Some(n.clone()),
            Value::Boolean(b) => Some(b.to_string()),
            Value::Null => Some(String::from("NULL")),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::command::insert::parse;
    use crate::command::sqlcommands::SqlCommand;
    use sqlparser::ast::Statement;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    fn parse_insert(statement: &str) -> SqlCommand {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, statement).unwrap();

        match ast.into_iter().next().unwrap() {
            Statement::Insert(insert) => parse(insert),
            _ => panic!("expected query"),
        }
    }

    #[test]
    fn test_insert_without_columns() {
        let command = crate::command::insert::tests::parse_insert(
            "INSERT INTO Customers VALUES ('Cardinal', 'Stavanger', 'Norway');",
        );

        match command {
            SqlCommand::INSERT {
                command,
                table,
                columns,
                values,
                ..
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "Customers");
                assert_eq!(columns.len(), 0);
                assert_eq!(values[0][0], "Cardinal");
                assert_eq!(values[0][1], "Stavanger");
                assert_eq!(values[0][2], "Norway");
            }
            _ => panic!("expected INSERT"),
        }
    }


    #[test]
    fn test_insert_with_columns() {
        let command = crate::command::insert::tests::parse_insert(
            "INSERT INTO Customers (CustomerName, City, Country) VALUES ('Cardinal', 'Stavanger', 'Norway');",
        );

        match command {
            SqlCommand::INSERT {
                command,
                table,
                columns,
                values,
                ..
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "Customers");
                assert_eq!(columns.len(), 3);
                assert_eq!(columns[0], "CustomerName");
                assert_eq!(columns[1], "City");
                assert_eq!(columns[2], "Country");
                assert_eq!(values[0][0], "Cardinal");
                assert_eq!(values[0][1], "Stavanger");
                assert_eq!(values[0][2], "Norway");
            }
            _ => panic!("expected INSERT"),
        }
    }

    #[test]
    fn test_insert_multirow() {
        let command = crate::command::insert::tests::parse_insert(
            "INSERT INTO Customers VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway'), ('Greasy Burger', 'Per Olsen', 'Gateveien 15', 'Sandnes', '4306', 'Norway'), ('Tasty Tee', 'Finn Egan', 'Streetroad 19B', 'Liverpool', 'L1 0AA', 'UK');",
        );

        match command {
            SqlCommand::INSERT {
                command,
                table,
                columns,
                values,
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "Customers");
                assert_eq!(values[0][0], "Cardinal");
                assert_eq!(values[0][1], "Tom B. Erichsen");
                assert_eq!(values[0][2], "Skagen 21");
                assert_eq!(values[0][3], "Stavanger");
                assert_eq!(values[0][4], "4006");
                assert_eq!(values[0][5], "Norway");
                assert_eq!(values[1][0], "Greasy Burger");
                assert_eq!(values[1][1], "Per Olsen");
                assert_eq!(values[1][2], "Gateveien 15");
                assert_eq!(values[1][3], "Sandnes");
                assert_eq!(values[1][4], "4306");
                assert_eq!(values[1][5], "Norway");
                assert_eq!(values[2][0], "Tasty Tee");
                assert_eq!(values[2][1], "Finn Egan");
                assert_eq!(values[2][2], "Streetroad 19B");
                assert_eq!(values[2][3], "Liverpool");
                assert_eq!(values[2][4], "L1 0AA");
                assert_eq!(values[2][5], "UK");
            }
            _ => panic!("expected INSERT"),
        }
    }

    #[test]
    fn test_insert_multirow_with_columns() {
        let command = crate::command::insert::tests::parse_insert(
            "INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country) VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway'), ('Greasy Burger', 'Per Olsen', 'Gateveien 15', 'Sandnes', '4306', 'Norway'), ('Tasty Tee', 'Finn Egan', 'Streetroad 19B', 'Liverpool', 'L1 0AA', 'UK');",
        );

        match command {
            SqlCommand::INSERT {
                command,
                table,
                columns,
                values,
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "Customers");
                assert_eq!(columns[0], "CustomerName");
                assert_eq!(columns[1], "ContactName");
                assert_eq!(columns[2], "Address");
                assert_eq!(columns[3], "City");
                assert_eq!(columns[4], "PostalCode");
                assert_eq!(columns[5], "Country");
                assert_eq!(values[0][0], "Cardinal");
                assert_eq!(values[0][1], "Tom B. Erichsen");
                assert_eq!(values[0][2], "Skagen 21");
                assert_eq!(values[0][3], "Stavanger");
                assert_eq!(values[0][4], "4006");
                assert_eq!(values[0][5], "Norway");
                assert_eq!(values[1][0], "Greasy Burger");
                assert_eq!(values[1][1], "Per Olsen");
                assert_eq!(values[1][2], "Gateveien 15");
                assert_eq!(values[1][3], "Sandnes");
                assert_eq!(values[1][4], "4306");
                assert_eq!(values[1][5], "Norway");
                assert_eq!(values[2][0], "Tasty Tee");
                assert_eq!(values[2][1], "Finn Egan");
                assert_eq!(values[2][2], "Streetroad 19B");
                assert_eq!(values[2][3], "Liverpool");
                assert_eq!(values[2][4], "L1 0AA");
                assert_eq!(values[2][5], "UK");
            }
            _ => panic!("expected INSERT"),
        }
    }


}
