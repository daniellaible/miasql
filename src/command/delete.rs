use regex::Regex;
use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::command::update::Update;
use crate::command::whereclause::WhereClause;
use crate::database::database::Database;

#[derive(Debug)]
pub struct Delete {
    table_name: String,
    where_clause: WhereClause,
}


impl Command for Delete {
    fn parse(stmt: String, dbs: Vec<Database>) -> SqlCommand {
        let table = get_table(&stmt);
        println!("table: {:?}", table);
        let clause: WhereClause = WhereClause::parse(&stmt);
        println!("clause: {:?}", clause);
        SqlCommand::UNDEFINED
    }
}

fn get_table(stmt: &String) -> String {
    let regex = Regex::new(r"(?i)FROM\s+(.*?)\s+WHERE").unwrap();
    let captures = regex.captures(&stmt).unwrap();
    let table = captures.get(1).unwrap().as_str();
    table.to_string()
}

impl Delete {
    pub fn default() -> Self {
        Delete {
            table_name: String::default(),
            where_clause: WhereClause::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::command::Command;
    use crate::command::delete::Delete;
    use crate::command::sqlcommands::SqlCommand;
    use crate::command::sqloperator::Operator;
    use crate::database::database::Database;
    use crate::database::datatype::DataType;

    #[test]
    fn simple_delete_with_where() {
        let dbs: Vec<Database> = vec!();
        let stmt = "DELETE FROM Customers WHERE id=1;";
        let command = Delete::parse(stmt.to_string(), dbs);
    }
}