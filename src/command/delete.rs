use sqlparser::ast::{BinaryOperator, Delete, Expr, FromTable, TableFactor, Value};
use crate::command::sqlcommands::SqlCommand;
use crate::command::sqloperator::Operator;
use crate::command::whereclause::WhereClause;
use crate::database::datatype::DataType;

pub fn parse(del_stmt: Delete) -> Result<SqlCommand, String> {
    let table = extract_table_name(&del_stmt.from)?;
    let where_clause = extract_where_clause(del_stmt.selection.as_ref())?;

    Ok(SqlCommand::DELETE {
        command: String::from("DELETE"),
        table,
        where_clause,
    })
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
                let parsed = n.parse::<i64>()
                    .map_err(|_| format!("Invalid integer literal: {}", n))?;
                Ok(DataType::BigInt { x: parsed })
            }
            Value::SingleQuotedString(s) => Ok(DataType::VarChar { x: s.clone(), y: 255 }),
            Value::Boolean(b) => Ok(DataType::Bool{ x: *b }),
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