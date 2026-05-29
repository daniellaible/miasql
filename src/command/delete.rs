use sqlparser::ast::{BinaryOperator, Delete, Expr, FromTable, TableFactor, Value};
use crate::command::sqlcommands::SqlCommand;
use crate::command::sqloperator::Operator;
use crate::command::whereclause::WhereClause;

pub fn parse(del_stmt: Delete) -> SqlCommand {
    let table = extract_table_name(&del_stmt.from)?;
    let where_clause = extract_where_clause(del_stmt.selection.as_ref())?;

    SqlCommand::DELETE {
        command: String::from("DELETE"),
        table,
        where_clause,
    }
}

fn extract_where_clause(selection: Option<&Expr>) -> Result<WhereClause, String> {
    let expr = selection.ok_or_else(|| "DELETE statement has no WHERE clause".to_string())?;

    match expr {
        Expr::BinaryOp { left, op, right } => {
            let column = extract_expr_as_string(left)?;
            let operator:Operator = extract_operator(op);
            let value = extract_expr_as_string(right)?;

/*            return WhereClause {
                column,
                operator: operator,
                BigInt{x:1},
            };*/
        }
        _ => Err("Unsupported WHERE clause expression".to_string()),
    }
}

fn extract_operator(op: &BinaryOperator) -> Operator {
    let operator = match op {
        BinaryOperator::Eq => Operator::EQUAL,
        BinaryOperator::NotEq => Operator::NOTEQUAL,
        BinaryOperator::Gt => Operator::GREATER,
        BinaryOperator::Lt => Operator::LESSER,
        BinaryOperator::GtEq => Operator::GREATEROREQ,
        BinaryOperator::LtEq => Operator::LESSEROREQ,
        _ => {Operator::UNDEFINED}
    };
    operator
}

fn extract_expr_as_string(expr: &Expr) -> Result<String, String> {
    match expr {
        Expr::Identifier(ident) => Ok(ident.value.clone()),

        Expr::CompoundIdentifier(parts) => Ok(parts
            .iter()
            .map(|ident| ident.value.clone())
            .collect::<Vec<_>>()
            .join(".")),

        Expr::Value(v) => match &v.value {
            Value::Number(n, _) => Ok(n.clone()),
            Value::SingleQuotedString(s) => Ok(s.clone()),
            Value::Boolean(b) => Ok(b.to_string()),
            Value::Null => Ok("NULL".to_string()),
            other => Err(format!("Unsupported literal value: {:?}", other)),
        },

        _ => Err(format!("Unsupported expression: {:?}", expr)),
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
                .map(|ident| ident.value.clone())
                .collect::<Vec<_>>()
                .join(".");

            Ok(table_name)
        }
        _ => Err("Unsupported table factor in DELETE".to_string()),
    }
}