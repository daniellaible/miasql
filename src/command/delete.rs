use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::command::whereclause::WhereClause;
use crate::database::database::Database;
use regex::Regex;

#[derive(Debug)]
pub struct Delete {
}


impl Command for Delete {
    fn parse(stmt: String, dbs: Vec<Database>) -> SqlCommand {
        let table = get_table(&stmt);
        let clause: WhereClause = WhereClause::parse(&stmt);
        SqlCommand::DELETE {
            command:String::from("DELETE"),
            table,
            where_clause: clause,
        }
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
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::command::Command;
    use crate::command::delete::Delete;
    use crate::database::database::Database;

    #[test]
    fn simple_delete_with_where() {
        let dbs: Vec<Database> = vec!();
        let stmt = "DELETE FROM Customers WHERE id=1;";
        let command = Delete::parse(stmt.to_string(), dbs);
    }
}