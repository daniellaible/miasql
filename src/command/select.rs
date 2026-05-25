use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::command::sqloperator::Operator;
use crate::command::whereclause::WhereClause;
use crate::database::datatype;
use sqlparser::ast::{BinaryOperator, Expr, Function as SqlFunction, FunctionArg, FunctionArgExpr, FunctionArgumentList, FunctionArguments, Ident, ObjectName, ObjectNamePart, Query, Select, SelectItem, TableFactor, TableWithJoins, Value, ValueWithSpan};
use sqlparser::tokenizer::Token;


#[derive(Clone, Debug, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JoinClause {
    pub join_type: JoinType,
    pub table: String,
    pub left_column: String,
    pub right_column: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Projection {
    Column(String),
    Function {
        name: String,
        column: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionKind {
    Avg,
    Sum,
    Count,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedFunction {
    pub kind: FunctionKind,
    pub column: String,
}

pub fn parse(query: Box<Query>) -> SqlCommand {
    let body = *query.body.clone();
    let select = body.as_select();

    let select_stmt = match select {
        Some(x) => x,
        _ => {
            println!("Unable to parse Select command");
            return SqlCommand::UNDEFINED;
        }
    };

    let ident = retrieve_identifier(&select_stmt);

    let tablename_opt: Option<&str> = match select_stmt.from.as_slice() {
        [TableWithJoins { relation, joins }] if joins.is_empty() => match relation {
            TableFactor::Table {
                name: ObjectName(parts),
                ..
            } => match parts.as_slice() {
                [ObjectNamePart::Identifier(Ident { value, .. })] => Some(value.as_str()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    };

    let (tablename, joins) = match extract_table_and_joins(select_stmt) {
        Some(x) => x,
        None => {
            println!("Unable to parse table / joins");
            return SqlCommand::UNDEFINED;
        }
    };

    let where_clause = match &select_stmt.selection {
        Some(expr) => match retrieve_where_clause(expr) {
            Some(x) => x,
            None => {
                println!("Unable to parse where clause");
                return SqlCommand::UNDEFINED;
            }
        },
        None => WhereClause {
            column: String::new(),
            operator: Operator::UNDEFINED,
            value: datatype::DataType::Undefined,
        },
    };

    let foo = &select_stmt.projection;
    let bar = foo.iter();
    for val in bar {
        let parsed = parse_projection(val);
    }

    let distinct = extract_distinct(select_stmt);
    let group_by = extract_group_by(select_stmt);
    let order_by = extract_order_by(&query);
    let columns = extract_columns(select_stmt);

    let command = SqlCommand::SELECT {
        command: String::from(ident),
        table: String::from(tablename),
        columns,
        distinct,
        group_by,
        order_by,
        joins,
        where_clause,
    };
    println!("command: {:?}", command);
    command
}

fn extract_table_and_joins(select_stmt: &Select) -> Option<(String, Vec<JoinClause>)> {
    let [table_with_joins] = select_stmt.from.as_slice() else {
        return None;
    };

    let base_table = match &table_with_joins.relation {
        TableFactor::Table {
            name: ObjectName(parts),
            ..
        } => match parts.as_slice() {
            [ObjectNamePart::Identifier(Ident { value, .. })] => value.clone(),
            _ => return None,
        },
        _ => return None,
    };

    let mut joins = Vec::new();

    for join in &table_with_joins.joins {
        let joined_table = match &join.relation {
            TableFactor::Table {
                name: ObjectName(parts),
                ..
            } => match parts.as_slice() {
                [ObjectNamePart::Identifier(Ident { value, .. })] => value.clone(),
                _ => return None,
            },
            _ => return None,
        };

        let (join_type, constraint) = match &join.join_operator {
            sqlparser::ast::JoinOperator::Inner(c) => (JoinType::Inner, c),
            sqlparser::ast::JoinOperator::LeftOuter(c) => (JoinType::Left, c),
            sqlparser::ast::JoinOperator::RightOuter(c) => (JoinType::Right, c),
            sqlparser::ast::JoinOperator::FullOuter(c) => (JoinType::Full, c),
            _ => return None,
        };

        let (left_column, right_column) = match constraint {
            sqlparser::ast::JoinConstraint::On(Expr::BinaryOp { left, op, right }) => {
                if *op != BinaryOperator::Eq {
                    return None;
                }

                let left_column = extract_expr_identifier(left)?;
                let right_column = extract_expr_identifier(right)?;
                (left_column, right_column)
            }
            _ => return None,
        };

        joins.push(JoinClause {
            join_type,
            table: joined_table,
            left_column,
            right_column,
        });
    }

    Some((base_table, joins))
}

fn parse_projection(item: &SelectItem) -> Option<Projection> {
    match item {
        SelectItem::UnnamedExpr(Expr::Identifier(ident)) => {
            Some(Projection::Column(ident.value.clone()))
        }

        SelectItem::UnnamedExpr(Expr::Function(SqlFunction {
                                                   name,
                                                   args: FunctionArguments::List(FunctionArgumentList {
                                                                                     duplicate_treatment: None,
                                                                                     args,
                                                                                     clauses,
                                                                                 }),
                                                   filter: None,
                                                   null_treatment: None,
                                                   over: None,
                                                   within_group,
                                                   ..
                                               })) if clauses.is_empty() && within_group.is_empty() => {
            let ident = match &args[..] {
                [FunctionArg::Unnamed(FunctionArgExpr::Expr(Expr::Identifier(ident)))] => ident,
                _ => return None,
            };

            Some(Projection::Function {
                name: name.to_string(),
                column: ident.value.clone(),
            })
        }

        _ => None,
    }
}

fn parse_function(item: &SelectItem) -> Option<ParsedFunction> {
    let func = match item {
        SelectItem::UnnamedExpr(Expr::Function(func)) => func,
        _ => return None,
    };

    let args = match &func.args {
        FunctionArguments::List(FunctionArgumentList {
                                    duplicate_treatment: None,
                                    args,
                                    clauses,
                                }) if clauses.is_empty() => args,
        _ => return None,
    };

    if func.filter.is_some() || func.over.is_some() || !func.within_group.is_empty() {
        return None;
    }

    let ident = match &args[..] {
        [FunctionArg::Unnamed(FunctionArgExpr::Expr(Expr::Identifier(ident)))] => ident,
        _ => return None,
    };

    let kind = match func.name.to_string().to_ascii_lowercase().as_str() {
        "avg" => FunctionKind::Avg,
        "sum" => FunctionKind::Sum,
        "count" => FunctionKind::Count,
        _ => return None,
    };

    Some(ParsedFunction {
        kind,
        column: ident.value.clone(),
    })
}

fn retrieve_identifier(select_stmt: &&Select) -> String {
    let ident: &str = match &select_stmt.select_token.0.token {
        Token::Word(w) => w.value.as_str(),

        _ => {return String::new(); }
    };
    String::from(ident)
}

fn extract_distinct(select_stmt: &Select) -> bool {
    select_stmt.distinct.is_some()
}

fn extract_group_by(select_stmt: &Select) -> Vec<String> {
    let mut result = Vec::new();

    match &select_stmt.group_by {
        sqlparser::ast::GroupByExpr::Expressions(exprs, _) => {
            for expr in exprs {
                if let Some(name) = extract_expr_identifier(expr) {
                    result.push(name);
                }
            }
        }
        sqlparser::ast::GroupByExpr::All(_) => {
            result.push("ALL".to_string());
        }
    }

    result
}

fn extract_order_by(query: &Query) -> Vec<String> {
    let mut result = Vec::new();

    for order_by in &query.order_by {
        match &order_by.kind {
            sqlparser::ast::OrderByKind::Expressions(exprs) => {
                for order_expr in exprs {
                    if let Some(name) = extract_order_expr(order_expr) {
                        result.push(name);
                    }
                }
            }
            _ => {}
        }
    }

    result
}

fn extract_order_expr(order_expr: &sqlparser::ast::OrderByExpr) -> Option<String> {
    let name = extract_expr_identifier(&order_expr.expr)?;

    match order_expr.options.asc {
        Some(true) => Some(format!("{} ASC", name)),
        Some(false) => Some(format!("{} DESC", name)),
        None => Some(name),
    }
}

fn extract_expr_identifier(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Identifier(ident) => Some(ident.value.clone()),
        Expr::CompoundIdentifier(idents) => {
            Some(
                idents
                    .iter()
                    .map(|ident| ident.value.clone())
                    .collect::<Vec<_>>()
                    .join("."),
            )
        }
        _ => None,
    }
}

fn retrieve_where_clause(where_ast: &Expr) -> Option<WhereClause> {
    if let Expr::BinaryOp {
        left, op, right, ..
    } = where_ast
    {
        let col_name = match left.as_ref() {
            Expr::Identifier(ident) => ident.value.as_str(),
            _ => "unable to find column",
        };

        let operator = match op {
            BinaryOperator::Gt => Operator::GREATER,
            BinaryOperator::Lt => Operator::LESSER,
            BinaryOperator::Eq => Operator::EQUAL,
            BinaryOperator::NotEq => Operator::NOTEQUAL,
            BinaryOperator::GtEq => Operator::GREATEROREQ,
            BinaryOperator::LtEq => Operator::LESSEROREQ,
            _ => Operator::UNDEFINED,
        };

        let binopt_value: Option<&ValueWithSpan> = match right.as_ref() {
            Expr::Value(vws) => Some(vws),
            _ => None,
        };
        let value_with_span = binopt_value?;

        let datatype: datatype::DataType = match &value_with_span.value {
            Value::Number(num_str, _) => {
                let n: i64 = match num_str.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        return None;
                    }
                };
                datatype::DataType::BigInt { x: n }
            }
            Value::SingleQuotedString(ident) => datatype::DataType::VarChar {
                x: String::from(ident),
                y: ident.len(),
            },
            Value::DoubleQuotedString(ident) => datatype::DataType::VarChar {
                x: String::from(ident),
                y: ident.len(),
            },
            _ => datatype::DataType::Undefined,
        };

        Some(WhereClause {
            column: String::from(col_name),
            operator,
            value: datatype,
        })
    } else {
        None
    }
}

fn extract_columns(select_stmt: &Select) -> Vec<String> {
    let mut columns = Vec::new();

    for item in &select_stmt.projection {
        match parse_projection(item) {
            Some(Projection::Column(name)) => columns.push(name),
            Some(Projection::Function { name, column }) => {
                columns.push(format!("{}({})", name, column));
            }
            None => {}
        }
    }

    columns
}

#[cfg(test)]
mod tests {
    use crate::command::select::parse;
    use crate::command::sqlcommands::SqlCommand;
    use crate::command::sqloperator::Operator;
    use crate::database::datatype;
    use sqlparser::ast::Statement;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    fn parse_select(sql: &str) -> SqlCommand {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, sql).unwrap();

        match ast.into_iter().next().unwrap() {
            Statement::Query(query) => parse(query),
            _ => panic!("expected query"),
        }
    }

    #[test]
    fn select_with_distinct_group_by_and_order_by() {
        let command = parse_select(
            "SELECT DISTINCT avg(amount), sum(name), lastname
             FROM employee
             WHERE id = 'foo'
             GROUP BY lastname
             ORDER BY lastname"
        );

        match command {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                distinct,
                group_by,
                order_by,
                where_clause,
                ..
            } => {
                assert_eq!(command, "SELECT");
                assert_eq!(table, "employee");
                assert_eq!(
                    columns,
                    vec![
                        "avg(amount)".to_string(),
                        "sum(name)".to_string(),
                        "lastname".to_string()
                    ]
                );
                assert!(distinct);
                assert_eq!(group_by, vec!["lastname".to_string()]);
                assert_eq!(order_by, vec!["lastname".to_string()]);

                assert_eq!(where_clause.column, "id");
                assert_eq!(where_clause.operator, Operator::EQUAL);
                assert_eq!(
                    where_clause.value,
                    datatype::DataType::VarChar {
                        x: "foo".to_string(),
                        y: 3,
                    }
                );
            }
            _ => panic!("expected SELECT"),
        }
    }

    #[test]
    fn select_with_order_by_desc() {
        let command = parse_select(
            "SELECT lastname
             FROM employee
             WHERE id = 1
             ORDER BY lastname DESC"
        );

        match command {
            SqlCommand::SELECT {
                columns,
                distinct,
                group_by,
                order_by,
                where_clause,
                ..
            } => {
                assert_eq!(columns, vec!["lastname".to_string()]);
                assert!(!distinct);
                assert!(group_by.is_empty());
                assert_eq!(order_by, vec!["lastname DESC".to_string()]);
                assert_eq!(where_clause.column, "id");
                assert_eq!(where_clause.operator, Operator::EQUAL);
                assert_eq!(where_clause.value, datatype::DataType::BigInt { x: 1 });
            }
            _ => panic!("expected SELECT"),
        }
    }

    #[test]
    fn select_with_order_by_asc() {
        let command = parse_select(
            "SELECT lastname
             FROM employee
             WHERE id = 1
             ORDER BY lastname ASC"
        );

        match command {
            SqlCommand::SELECT { order_by, .. } => {
                assert_eq!(order_by, vec!["lastname ASC".to_string()]);
            }
            _ => panic!("expected SELECT"),
        }
    }

    #[test]
    fn select_with_multiple_group_by_columns() {
        let command = parse_select(
            "SELECT firstname, lastname
             FROM employee
             WHERE id = 1
             GROUP BY firstname, lastname"
        );

        match command {
            SqlCommand::SELECT { columns, group_by, .. } => {
                assert_eq!(
                    columns,
                    vec!["firstname".to_string(), "lastname".to_string()]
                );
                assert_eq!(
                    group_by,
                    vec!["firstname".to_string(), "lastname".to_string()]
                );
            }
            _ => panic!("expected SELECT"),
        }
    }

    #[test]
    fn select_with_multiple_order_by_columns() {
        let command = parse_select(
            "SELECT firstname, lastname
             FROM employee
             WHERE id = 1
             ORDER BY lastname DESC, firstname ASC"
        );

        match command {
            SqlCommand::SELECT { order_by, .. } => {
                assert_eq!(
                    order_by,
                    vec![
                        "lastname DESC".to_string(),
                        "firstname ASC".to_string()
                    ]
                );
            }
            _ => panic!("expected SELECT"),
        }
    }

    #[test]
    fn select_with_numeric_where_clause() {
        let command = parse_select(
            "SELECT name
             FROM employee
             WHERE id >= 100"
        );

        match command {
            SqlCommand::SELECT { where_clause, .. } => {
                assert_eq!(where_clause.column, "id");
                assert_eq!(where_clause.operator, Operator::GREATEROREQ);
                assert_eq!(where_clause.value, datatype::DataType::BigInt { x: 100 });
            }
            _ => panic!("expected SELECT"),
        }
    }
}