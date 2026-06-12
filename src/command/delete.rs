use crate::command::sqlcommands::SqlCommand;
use crate::command::sqloperator::Operator;
use crate::command::whereclause::WhereClause;
use crate::database::datatype::DataType;
use sqlparser::ast::{BinaryOperator, Delete, Expr, FromTable, TableFactor, Value};

pub fn parse(del_stmt: Delete) -> SqlCommand {
    let table = extract_table_name(&del_stmt.from).unwrap();
    let where_clause = extract_where_clause(del_stmt.selection.as_ref()).unwrap();

    SqlCommand::Delete {
        command: String::from("DELETE"),
        table,
        where_clause,
    }
}

fn extract_where_clause(selection: Option<&Expr>) -> Result<WhereClause, String> {
    let expr = selection.ok_or_else(|| "DELETE statement has no WHERE clause".to_string())?;

    match expr {
        Expr::BinaryOp { left, op, right } => {
            let column = extract_identifier_as_string(left)?;
            let operator: Operator = extract_operator(op);
            let value = extract_expr_as_datatype(right)?;

            Ok(WhereClause {
                column,
                operator,
                value,
            })
        }
        _ => Err("Unsupported WHERE clause expression".to_string()),
    }
}

fn extract_operator(op: &BinaryOperator) -> Operator {
    match op {
        BinaryOperator::Eq => Operator::EQUAL,
        BinaryOperator::NotEq => Operator::NOTEQUAL,
        BinaryOperator::Gt => Operator::GREATER,
        BinaryOperator::Lt => Operator::LESSER,
        BinaryOperator::GtEq => Operator::GREATEROREQ,
        BinaryOperator::LtEq => Operator::LESSEROREQ,
        _ => Operator::UNDEFINED,
    }
}

fn extract_identifier_as_string(expr: &Expr) -> Result<String, String> {
    match expr {
        Expr::Identifier(ident) => Ok(ident.value.clone()),
        Expr::CompoundIdentifier(parts) => Ok(parts
            .iter()
            .map(|ident| ident.value.clone())
            .collect::<Vec<_>>()
            .join(".")),
        _ => Err(format!("Expected column identifier, got: {:?}", expr)),
    }
}

fn extract_expr_as_datatype(expr: &Expr) -> Result<DataType, String> {
    match expr {
        Expr::Value(v) => match &v.value {
            Value::Number(n, _) => {
                // choose the right numeric type for your Datatype
                let parsed = n
                    .parse::<i64>()
                    .map_err(|_| format!("Invalid integer literal: {}", n))?;
                Ok(DataType::BigInt(parsed))
            }
            Value::SingleQuotedString(s) => Ok(DataType::VarChar(255, s.clone())),
            Value::Boolean(boolean) => Ok(DataType::Bool(*boolean)),
            Value::Null => Ok(DataType::Null),
            other => Err(format!("Unsupported literal value: {:?}", other)),
        },
        _ => Err(format!("Expected literal value, got: {:?}", expr)),
    }
}

fn extract_table_name(from: &FromTable) -> Result<String, String> {
    let tables = match from {
        FromTable::WithFromKeyword(tables) => tables,
        FromTable::WithoutKeyword(tables) => tables,
    };

    let first_table = tables
        .first()
        .ok_or_else(|| "DELETE statement has no table".to_string())?;

    match &first_table.relation {
        TableFactor::Table { name, .. } => {
            let table_name = name
                .0
                .iter()
                .map(|part| part.to_string())
                .collect::<Vec<_>>()
                .join(".");

            Ok(table_name)
        }
        _ => Err("Unsupported table factor in DELETE".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::command::sqlcommands::SqlCommand;
    use crate::command::sqloperator::Operator;
    use crate::database::datatype::DataType;
    use sqlparser::ast::Statement;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    fn parse_delete(sql: &str) -> Result<SqlCommand, String> {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, sql).unwrap();

        match ast.into_iter().next().unwrap() {
            Statement::Delete(delete) => Ok(parse(delete)),
            _ => panic!("expected delete statement"),
        }
    }

    #[test]
    fn delete_with_equal_numeric_where_clause() {
        let command = parse_delete("DELETE FROM employee WHERE id = 1").unwrap();

        match command {
            SqlCommand::Delete {
                command,
                table,
                where_clause,
            } => {
                assert_eq!(command, "DELETE");
                assert_eq!(table, "employee");
                assert_eq!(where_clause.column, "id");
                assert_eq!(where_clause.operator, Operator::EQUAL);
                assert_eq!(where_clause.value, DataType::BigInt(1));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_not_equal_numeric_where_clause() {
        let command = parse_delete("DELETE FROM employee WHERE id != 10").unwrap();

        match command {
            SqlCommand::Delete {
                table,
                where_clause,
                ..
            } => {
                assert_eq!(table, "employee");
                assert_eq!(where_clause.column, "id");
                assert_eq!(where_clause.operator, Operator::NOTEQUAL);
                assert_eq!(where_clause.value, DataType::BigInt(10));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_greater_than_where_clause() {
        let command = parse_delete("DELETE FROM employee WHERE id > 100").unwrap();

        match command {
            SqlCommand::Delete { where_clause, .. } => {
                assert_eq!(where_clause.column, "id");
                assert_eq!(where_clause.operator, Operator::GREATER);
                assert_eq!(where_clause.value, DataType::BigInt(100));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_less_than_where_clause() {
        let command = parse_delete("DELETE FROM employee WHERE id < 5").unwrap();

        match command {
            SqlCommand::Delete { where_clause, .. } => {
                assert_eq!(where_clause.column, "id");
                assert_eq!(where_clause.operator, Operator::LESSER);
                assert_eq!(where_clause.value, DataType::BigInt(5));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_greater_or_equal_where_clause() {
        let command = parse_delete("DELETE FROM employee WHERE id >= 7").unwrap();

        match command {
            SqlCommand::Delete { where_clause, .. } => {
                assert_eq!(where_clause.column, "id");
                assert_eq!(where_clause.operator, Operator::GREATEROREQ);
                assert_eq!(where_clause.value, DataType::BigInt(7));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_less_or_equal_where_clause() {
        let command = parse_delete("DELETE FROM employee WHERE id <= 3").unwrap();

        match command {
            SqlCommand::Delete { where_clause, .. } => {
                assert_eq!(where_clause.column, "id");
                assert_eq!(where_clause.operator, Operator::LESSEROREQ);
                assert_eq!(where_clause.value, DataType::BigInt(3));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_string_where_clause() {
        let command = parse_delete("DELETE FROM employee WHERE lastname = 'Miller'").unwrap();

        match command {
            SqlCommand::Delete { where_clause, .. } => {
                assert_eq!(where_clause.column, "lastname");
                assert_eq!(where_clause.operator, Operator::EQUAL);
                assert_eq!(
                    where_clause.value,
                    DataType::VarChar(255, "Miller".to_string()));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_boolean_true_where_clause() {
        let command = parse_delete("DELETE FROM employee WHERE active = true").unwrap();

        match command {
            SqlCommand::Delete { where_clause, .. } => {
                assert_eq!(where_clause.column, "active");
                assert_eq!(where_clause.operator, Operator::EQUAL);
                assert_eq!(where_clause.value, DataType::Bool(true));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_boolean_false_where_clause() {
        let command = parse_delete("DELETE FROM employee WHERE active = false").unwrap();

        match command {
            SqlCommand::Delete { where_clause, .. } => {
                assert_eq!(where_clause.column, "active");
                assert_eq!(where_clause.operator, Operator::EQUAL);
                assert_eq!(where_clause.value, DataType::Bool(false));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_null_where_clause() {
        let command = parse_delete("DELETE FROM employee WHERE deleted_at = NULL").unwrap();

        match command {
            SqlCommand::Delete { where_clause, .. } => {
                assert_eq!(where_clause.column, "deleted_at");
                assert_eq!(where_clause.operator, Operator::EQUAL);
                assert_eq!(where_clause.value, DataType::Null);
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_qualified_table_name() {
        let command = parse_delete("DELETE FROM mydb.employee WHERE id = 1").unwrap();

        match command {
            SqlCommand::Delete {
                table,
                where_clause,
                ..
            } => {
                assert_eq!(table, "mydb.employee");
                assert_eq!(where_clause.column, "id");
                assert_eq!(where_clause.value, DataType::BigInt(1));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_with_qualified_column_name() {
        let command = parse_delete("DELETE FROM employee WHERE employee.id = 1").unwrap();

        match command {
            SqlCommand::Delete { where_clause, .. } => {
                assert_eq!(where_clause.column, "employee.id");
                assert_eq!(where_clause.operator, Operator::EQUAL);
                assert_eq!(where_clause.value, DataType::BigInt(1));
            }
            _ => panic!("expected DELETE"),
        }
    }

    #[test]
    fn delete_preserves_uppercase_identifiers() {
        let command = parse_delete("DELETE FROM EMPLOYEE WHERE ID = 1").unwrap();

        match command {
            SqlCommand::Delete {
                table,
                where_clause,
                ..
            } => {
                assert_eq!(table, "EMPLOYEE");
                assert_eq!(where_clause.column, "ID");
                assert_eq!(where_clause.value, DataType::BigInt(1));
            }
            _ => panic!("expected DELETE"),
        }
    }
}
