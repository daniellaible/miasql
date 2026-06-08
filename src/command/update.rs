use crate::command::sqlcommands::SqlCommand;
use crate::command::sqloperator::Operator;
use crate::command::whereclause::WhereClause;
use crate::database::datatype;
use sqlparser::ast::{Assignment, AssignmentTarget, BinaryOperator, Expr, TableFactor, TableWithJoins, Update, ValueWithSpan};

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSet {
    pub column: String,
    pub value: String,
}

pub fn parse(update: Update) -> SqlCommand {
    let table = match parse_table(&update.table) {
        Some(table) => table,
        None => return SqlCommand::Undefined,
    };


    let where_clause = match &update.selection {
        Some(expr) => match retrieve_where_clause(expr) {
            Some(clause) => clause,
            None => return SqlCommand::Undefined,
        },
        None => return SqlCommand::Undefined,
    };



    let mut sets: Vec<UpdateSet> = vec![];

    for assignment in update.assignments.iter() {
        match parse_assignment(assignment) {
            Some(set) => sets.push(set),
            None => return SqlCommand::Undefined,
        }
    }

    SqlCommand::Update {
        command: String::from("UPDATE"),
        table,
        sets,
        where_clause,
    }
}

fn parse_assignment(assignment: &Assignment) -> Option<UpdateSet> {
    let column = match &assignment.target {
        AssignmentTarget::ColumnName(object_name) => object_name
            .0
            .iter()
            .map(|part| part.to_string())
            .collect::<Vec<_>>()
            .join("."),
        _ => return None,
    };

    let value = match &assignment.value {
        Expr::Value(vws) => match &vws.value {
            sqlparser::ast::Value::SingleQuotedString(s) => s.clone(),
            sqlparser::ast::Value::DoubleQuotedString(s) => s.clone(),
            sqlparser::ast::Value::Number(n, _) => n.clone(),
            sqlparser::ast::Value::Boolean(b) => b.to_string(),
            sqlparser::ast::Value::Null => String::from("NULL"),
            _ => return None,
        },
        _ => return None,
    };

    Some(UpdateSet { column, value })
}

fn parse_table(table_with_joins: &TableWithJoins) -> Option<String> {
    match &table_with_joins.relation {
        TableFactor::Table { name, .. } => Some(
            name.0
                .iter()
                .map(|part| part.to_string())
                .collect::<Vec<_>>()
                .join("."),
        ),
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
            Expr::CompoundIdentifier(idents) => {
                let joined = idents
                    .iter()
                    .map(|ident| ident.value.clone())
                    .collect::<Vec<_>>()
                    .join(".");
                return build_where_clause(joined, op, right);
            }
            _ => return None,
        };

        build_where_clause(String::from(col_name), op, right)
    } else {
        None
    }
}

fn build_where_clause(
    column: String,
    op: &BinaryOperator,
    right: &Box<Expr>,
) -> Option<WhereClause> {
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
        sqlparser::ast::Value::Number(num_str, _) => {
            let n: i64 = match num_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    return None;
                }
            };
            datatype::DataType::BigInt { x: n }
        }
        sqlparser::ast::Value::SingleQuotedString(ident) => datatype::DataType::VarChar {
            x: String::from(ident),
            y: ident.len(),
        },
        sqlparser::ast::Value::DoubleQuotedString(ident) => datatype::DataType::VarChar {
            x: String::from(ident),
            y: ident.len(),
        },
        sqlparser::ast::Value::Boolean(b) => datatype::DataType::Bool { x: *b },
        sqlparser::ast::Value::Null => datatype::DataType::Null,
        _ => datatype::DataType::Undefined,
    };

    Some(WhereClause {
        column,
        operator,
        value: datatype,
    })
}

#[cfg(test)]
mod tests {
    use crate::command::sqlcommands::SqlCommand;
    use crate::command::sqloperator::Operator;
    use crate::command::update::parse;
    use crate::database::datatype;
    use sqlparser::ast::Statement;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    fn parse_select(statement: &str) -> SqlCommand {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, statement).unwrap();

        match ast.into_iter().next().unwrap() {
            Statement::Update(update) => parse(update),
            _ => panic!("expected query"),
        }
    }

    #[test]
    fn test_update() {
        let command = parse_select(
            "UPDATE Customers SET ContactName = 'Alfred Schmidt', City= 'Frankfurt' WHERE CustomerID = 1;",
        );

        match command {
            SqlCommand::Update {
                command,
                table,
                sets,
                where_clause,
            } => {
                assert_eq!(command, "UPDATE");
                assert_eq!(table, "Customers");
                assert_eq!(sets.len(), 2);
                assert_eq!(sets[0].column, "ContactName");
                assert_eq!(sets[0].value, "Alfred Schmidt");
                assert_eq!(sets[1].column, "City");
                assert_eq!(sets[1].value, "Frankfurt");

                assert_eq!(where_clause.column, "CustomerID");
                assert_eq!(where_clause.operator, Operator::EQUAL);
                assert_eq!(where_clause.value, datatype::DataType::BigInt { x: 1 });
            }
            _ => panic!("expected UPDATE"),
        }
    }

    #[test]
    fn test_update_second() {
        let command = parse_select(
            "UPDATE Customers SET ContactName = 'Alfred Schmidt' WHERE CustomerID = 1;",
        );

        match command {
            SqlCommand::Update {
                command,
                table,
                sets,
                where_clause,
            } => {
                assert_eq!(command, "UPDATE");
                assert_eq!(table, "Customers");
                assert_eq!(sets.len(), 1);
                assert_eq!(sets[0].column, "ContactName");
                assert_eq!(sets[0].value, "Alfred Schmidt");

                assert_eq!(where_clause.column, "CustomerID");
                assert_eq!(where_clause.operator, Operator::EQUAL);
                assert_eq!(where_clause.value, datatype::DataType::BigInt { x: 1 });
            }
            _ => panic!("expected UPDATE"),
        }
    }

    #[test]
    fn test_update_without_where() {
        let command = parse_select("UPDATE Customers SET ContactName = 'Alfred Schmidt'");

        assert_eq!(command, SqlCommand::Undefined);
    }
}
