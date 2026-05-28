use crate::command;
use crate::command::sqlcommands::SqlCommand;
use crate::database::database::Database;
use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::tokenizer::Token;
use std::any::Any;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub async fn handle_client(mut stream: TcpStream, mut dbs: &Vec<Database>) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }

        let mut input = str::from_utf8(&buf[..n]).unwrap();
        input = input.trim();

        let command: SqlCommand = tokenizer(input);

        /*let command: String = input.to_uppercase();

        if command == "QUIT" || command == "BYE" {
            return Ok(());
        } else if command == "SHUTDOWN" {
            return Ok(());
        } else if command == "HELP" {
        } else if command == "SHOW DATABASES" {
        } else if command == "SHOW TABLES" {
        } else {
            let mut sql: SqlCommand = SqlCommand::UNDEFINED;

            if command.starts_with("SELECT") {
                sql = Select::parse(String::from(command), dbs.clone());
            } else if command.starts_with("INSERT") {
                sql = Insert::parse(String::from(command), dbs.clone());
            } else if command.starts_with("UPDATE") {
                sql = Update::parse(String::from(command), dbs.clone());
            } else if command.starts_with("DELETE") {
                sql = Delete::parse(String::from(command), dbs.clone());
            } else if command.starts_with("CREATE") {
                println!("CREATE recognized");
                println!("Could be CREATE DATABASE or CREATE TABLE");
            } else if command.starts_with("ALTER") {
                println!("ALTER recognized");
                println!("ALTER command is a bitch");

            } else if command.starts_with("DROP") {
                let clone = command.clone().trim().to_uppercase().to_string();

                if clone.contains(" TABLE ") {
                    sql = DropTable::parse(String::from(command), dbs.clone());
                } else if clone.contains(" DATABASE ") {
                    sql = DropDatabase::parse(String::from(command), dbs.clone());
                } else {
                    println!("Unable to interpret the command");
                }

            } else if command.starts_with("TRUNCATE") {
                println!("TRUNCATE recognized");
            } else if command.starts_with("GRANT") {
                println!("GRANT recognized");
            } else if command.starts_with("REVOKE") {
                println!("REVOKE recognized");
            } else if command.starts_with("USE") {
                println!("USE recognized");
                println!("Needs to be the first command");
            }
        }

        stream.write_all(&buf[..n]).await?;*/
    }
}

fn tokenizer(stmt: &str) -> SqlCommand {
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, stmt).unwrap();
    println!("{:#?}", ast[0].clone());
    let mut command: SqlCommand = SqlCommand::UNDEFINED;

    match ast[0].clone() {
        Statement::AlterTable(alter) => {
            println!("table name: {}", alter.name);
            println!("if_exists: {}", alter.if_exists);
            println!("only: {}", alter.only);
            println!("operations: {:?}", alter.operations);
            println!("location: {:?}", alter.location);
            println!("on_cluster: {:?}", alter.on_cluster);
            println!("table_type: {:?}", alter.table_type);
            println!("end_token: {:?}", alter.end_token);
        }
        Statement::CreateTable(create) => {
            command = command::createtable::parse(create.clone());
        }
        Statement::CreateDatabase { .. }=> {
            command = command::createdatabase::parse(ast);
        }
        Statement::Drop { .. } => {
            command = command::drop::parse(ast);
        }
        Statement::Insert(insert) => {
            println!("table: {:?}", insert.table);
        }
        Statement::Query(query) => {
            command = command::select::parse(query.clone());

            println!("with: {:?}", query.with);
            let body = *query.body.clone();
            let select = body.as_select().unwrap();

            let word_value: Option<&str> = match &select.select_token.0.token {
                Token::Word(w) => Some(w.value.as_str()),
                _ => None,
            };
            println!(" word_value: {:?}", word_value.unwrap());

            println!("  body.optimizer_hints: {:?}", select.optimizer_hints);
            println!("  body.distinct: {:?}", select.distinct);
            println!("  body.select_modifiers: {:?}", select.select_modifiers);
            println!("  body.top: {:?}", select.top);
            println!(
                "  body.top_before_distinct: {:?}",
                select.top_before_distinct
            );
            println!("  body.projection: {:?}", select.projection);
            println!("  body.exclude: {:?}", select.exclude);
            println!("  body.into: {:?}", select.into);
            println!("  body.from (tablename): {:?}", select.from);
            println!("  body.lateral_views: {:?}", select.lateral_views);
            println!("  body.prewhere: {:?}", select.prewhere);
            println!("  body.selection (where part): {:?}", select.selection);
            println!("  body.connect_by: {:?}", select.connect_by);
            println!("  body.group_by: {:?}", select.group_by);
            println!("  body.cluster_by: {:?}", select.cluster_by);
            println!("  body.distribute_by: {:?}", select.distribute_by);
            println!("  body.sort_by: {:?}", select.sort_by);
            println!("  body.having: {:?}", select.having);
            println!("  body.named_window: {:?}", select.named_window);
            println!("  body.qualify: {:?}", select.qualify);
            println!(
                "  body.window_before_qualify: {:?}",
                select.window_before_qualify
            );
            println!("  body.value_table_mode: {:?}", select.value_table_mode);
            println!("  body.flavor: {:?}", select.flavor);

            println!("order_by: {:?}", query.order_by);
            println!("limit_clause: {:?}", query.limit_clause);
            println!("fetch: {:?}", query.fetch);
            println!("locks: {:?}", query.locks);
            println!("for_clause: {:?}", query.for_clause);
            println!("settings: {:?}", query.settings);
            println!("format_clause: {:?}", query.format_clause);
            println!("pipe_operators: {:?}", query.pipe_operators);
        }
        Statement::Update(update) => {
            println!("table: {:?}", update.table);
        }
        _ => println!("other statement"),
    }
    SqlCommand::UNDEFINED
}

#[cfg(test)]
mod tests {
    use crate::server::server::tokenizer;

    #[test]
    fn test_tokenizer_select() {
        let command: &str =
            "Select distinct avg(amount), name, lastname from employee where id='foo'";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_create_table() {
        let command: &str = "CREATE TABLE Persons ( PersonID BigInt PRIMARY KEY, LastName VarChar(255) NOT NULL, FirstName VarChar(255), Address VarChar(255), City VarChar(255));";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_create_database(){
        let command: &str = "CREATE DATABASE employee";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_drop_database(){
        let command: &str = "DROP DATABASE employee";
        tokenizer(command);
    }

    #[test]
    fn test_tokenizer_drop_table(){
        let command: &str = "DROP TABLE employee";
        tokenizer(command);
    }
}
