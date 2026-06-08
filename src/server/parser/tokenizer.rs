use crate::command;
use crate::command::sqlcommands::SqlCommand;
use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub fn tokeniz(input: &str) -> SqlCommand {
    let dialect = GenericDialect {};

    let parse_result = Parser::parse_sql(&dialect, input);

    let ast = match &parse_result {
        Ok(ast) => parse_result.unwrap(),
        Err(e) => return SqlCommand::Undefined,
    };

    if ast.is_empty() { return SqlCommand::Undefined }
    let mut command: SqlCommand = SqlCommand::Undefined;


    match ast[0].clone() {
        Statement::AlterTable(alter) => {
            command = command::alter::parse(alter.clone());
        }
        Statement::CreateTable(create) => {
            command = command::createtable::parse(create.clone());
        }
        Statement::Truncate(truncate) => {
            command = command::truncate::parse(truncate);
        }
        Statement::CreateDatabase { .. } => {
            command = command::createdatabase::parse(ast);
        }
        Statement::Drop { .. } => {
            command = command::drop::parse(ast);
        }
        Statement::Insert(insert) => {
            command = command::insert::parse(insert.clone());
        }
        Statement::Query(query) => {
            command = command::select::parse(query.clone());
        }
        Statement::Update(update) => {
            command = command::update::parse(update.clone());
        }
        Statement::Delete(delete) => {
            command = command::delete::parse(delete.clone());
        }
        _ => command = SqlCommand::Undefined,
    }
    command
}
