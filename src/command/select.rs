use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::command::sqloperator::Operator;
use crate::command::whereclause::WhereClause;
use crate::database::datatype;
use sqlparser::ast::{BinaryOperator, Expr, Function as SqlFunction, FunctionArg, FunctionArgExpr, FunctionArgumentList, FunctionArguments, Ident, ObjectName, ObjectNamePart, Query, Select, SelectItem, TableFactor, TableWithJoins, Value, ValueWithSpan};
use sqlparser::tokenizer::Token;


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
    println!("with: {:?}", query.with);
    let body = *query.body;
    let select = body.as_select();

    let select_stmt = match select {
        Some(x) => x,
        _ => {
            println!("Unable to parse Select command");
            return SqlCommand::UNDEFINED;
        }
    };
    println!("{:?}", select_stmt);

    let ident = retrieve_identifier(&select_stmt);
    println!(" word_value: {:?}", ident);

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

    let tablename = match tablename_opt {
        Some(x) => x,
        None => {
            println!("Unable to parse table name");
            return SqlCommand::UNDEFINED;
        }
    };

    let Some(expr) = &select_stmt.selection else {
        return SqlCommand::UNDEFINED;
    };
    let where_opt = retrieve_where_clause(expr);
    let where_clause = match where_opt {
        Some(x) => x,
        None => {
            println!("Unable to parse where clause");
            return SqlCommand::UNDEFINED;
        }
    };

    let foo = &select_stmt.projection;
    let bar = foo.iter();
    for val in bar {
        let parsed = parse_projection(val);
        println!("{:?}", parsed);

/*        if let Some(func) = parsed {
            println!("function = {}, column = {}", func.name, func.column);
        }*/
    }




    println!("tablename: {:?}", tablename);
    println!("where: {:?}", where_clause);

    SqlCommand::SELECT {
        command: String::from(ident),
        table: String::from(tablename),
        columns: Vec::new(),
        values: Vec::new(),
        where_clause: where_clause,
    }
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
            Value::SingleQuotedString(ident) => datatype::DataType::Char {
                x: String::from(ident),
                y: ident.len(),
            },
            Value::DoubleQuotedString(ident) => datatype::DataType::Char {
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

#[cfg(test)]
mod tests {
    use crate::command::select::parse;
    use sqlparser::ast::Statement;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use crate::command::sqlcommands::SqlCommand;

    #[test]
    fn basic_select_test() {
        let command: &str =
            "Select distinct avg(amount), sum(name), lastname from employee where id='foo'";
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, command).unwrap();

        match ast[0].clone() {
            Statement::Query(query) => {
                println!("{:?}", query);
                let command = parse(query);
                println!("{:?} ",command);
            }
            _ => {
                assert!(false);
            }
        }
    }
}

/*   #[test]
    fn simple_select_with_where_clause() {
        let dbs: Vec<Database> = vec!();
        let statement = "SELECT name, country FROM population WHERE id=1";
        let _select: SqlCommand = Select::parse(String::from(statement), dbs);
    }

    #[test]
    fn simple_select_with_where_clause_lowercase() {
        let dbs: Vec<Database> = vec!();
        let select = "select name, country from population where id=1";
        let select: SqlCommand = Select::parse(String::from(select), dbs);

        match select {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                values: _,
                where_clause,
            } => {
                assert_eq!(command, "SELECT");
                assert_eq!(table, "POPULATION");
                assert_eq!(columns, vec!["NAME", "COUNTRY"]);

                let clause = where_clause;
                assert_eq!(clause.get_operator(), Operator::EQUAL);
                assert_eq!(clause.get_column(), "ID");
                assert_eq!(clause.get_value(), DataType::BigInt { x: 1 });
            }
            _ => (),
        }
    }

    #[test]
    fn select_with_the_stars(){
        let dbs: Vec<Database> = vec!();
        let select = "select * from users where id = 1";
        let select: SqlCommand = Select::parse(String::from(select), dbs);
        println!("{:#?}", select);

        match select {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                values: _,
                where_clause,
            } => {
                assert_eq!(command, "SELECT");
                assert_eq!(table, "USERS");
                assert_eq!(columns, vec!["*"]);

                let clause = where_clause;
                assert_eq!(clause.get_operator(), Operator::EQUAL);
                assert_eq!(clause.get_column(), "ID");
                assert_eq!(clause.get_value(), DataType::BigInt { x: 1 });

            }
            _ => (),
        }
    }

    #[test]
    fn simple_select_with_where_clause_less_than() {
        let dbs: Vec<Database> = vec!();
        let select = "select name, country from population where id<100";
        let select: SqlCommand = Select::parse(String::from(select), dbs);

        match select {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                values: _,
                where_clause,
            } => {
                assert_eq!(command, "SELECT");
                assert_eq!(table, "POPULATION");
                assert_eq!(columns, vec!["NAME", "COUNTRY"]);

                let clause = where_clause;
                assert_eq!(clause.get_operator(), Operator::LESSER);
                assert_eq!(clause.get_column(), "ID");
                assert_eq!(clause.get_value(), DataType::BigInt { x: 100 });
            }
            _ => (),
        }
    }

    #[test]
    fn simple_select_with_where_clause_greater_than() {
        let dbs: Vec<Database> = vec!();
        let select = "select name, country from population where id>100";
        let select: SqlCommand = Select::parse(String::from(select), dbs);

        match select {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                values: _,
                where_clause,
            } => {
                assert_eq!(command, "SELECT");
                assert_eq!(table, "POPULATION");
                assert_eq!(columns, vec!["NAME", "COUNTRY"]);

                let clause = where_clause;
                assert_eq!(clause.get_operator(), Operator::GREATER);
                assert_eq!(clause.get_column(), "ID");
                assert_eq!(clause.get_value(), DataType::BigInt { x: 100 });
            }
            _ => (),
        }
    }

    #[test]
    fn simple_select_without_where_clause() {
        let dbs: Vec<Database> = vec!();
        let select = "SELECT name, country FROM population";
        let select: SqlCommand = Select::parse(String::from(select), dbs);

        match select {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                values: _,
                where_clause,
            } => {
                assert_eq!(command, "SELECT");
                assert_eq!(table, "POPULATION");
                assert_eq!(columns, vec!["NAME", "COUNTRY"]);

                let clause = where_clause;
                assert_eq!(clause.get_operator(), Operator::UNDEFINED);
                assert_eq!(clause.get_column(), "");
                assert_eq!(clause.get_value(), DataType::Undefined);
            }
            _ => (),
        }
    }
}*/
