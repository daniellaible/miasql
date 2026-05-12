use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::command::whereclause::WhereClause;
use regex::Regex;

#[derive(Debug)]
pub struct Select {
    table_name: String,
    columns: Vec<String>,
    where_clause: WhereClause,
}

impl Command for Select{

    fn parse(stmt: String) -> SqlCommand{
        let columns = get_columns(&stmt);
        let table = get_table_name(&stmt).unwrap_or_else(|| "doof").parse().unwrap();
        if check_for_where(&stmt) {
            let clause  = WhereClause::parse(&stmt);
            let command = SqlCommand::SELECT {command: String::from("SELECT"), table, columns, values: Vec::new(), where_clause: clause};
            println!("{:?}",command);
            command
        }else{
            let command = SqlCommand::SELECT {command: String::from("SELECT"), table, columns, values: Vec::new(), where_clause: WhereClause::default()};
            println!("{:?}",command);
            command
        }
    }
}



fn get_columns(stmt: &String) -> Vec<String> {
    let regex = Regex::new(r"(?i)\bselect\b\s*([\s\S]*?)\s+\bfrom\b\s+").unwrap();
    let captures = regex.captures(stmt).unwrap();
    let columns_as_string = captures.get(1).unwrap().as_str();
    let single_column:Vec<&str> = columns_as_string.split(",").collect();
    let mut parts: Vec<String> = Vec::new();
    for single_column in single_column {
        parts.push(single_column.to_string());
    }
    parts
}

fn get_table_name(stmt: &str) -> Option<&str> {
    let re = Regex::new(r"(?i)\bfrom\b\s+([^\s;]+)(?:\s+\bwhere\b\s+[\s\S]*)?$").unwrap();
    re.captures(stmt).and_then(|c| c.get(1).map(|m| m.as_str()))
}

fn check_for_where(stmt: &String) -> bool {
    let reg = Regex::new(r"(?i)\swhere\s").unwrap();
    if reg.is_match(stmt){
        return true;
    }
    false
}


impl Select {
    pub fn default() -> Self {
        Select {
            table_name: String::default(),
            columns: vec![],
            where_clause: WhereClause::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::command::Command;
    use crate::command::select::Select;
    use crate::command::sqlcommands::SqlCommand;

    #[test]
    fn simple_select_with_where_clause() {
        let statement = "SELECT name, country FROM population WHERE id=1";
        let select: SqlCommand = Select::parse(String::from(statement));
    }

    #[test]
    fn simple_select_with_where_clause_lowercase() {
        let select = "select name, country from population where id=1";
        let select: SqlCommand = Select::parse(String::from(select));
/*        let clause: WhereClause = select.where_clause;
        assert_eq!(select.table_name, "population");
        assert_eq!(select.columns.len(), 2);
        assert_eq!(clause.get_operator(), Operator::EQUAL);*/
    }

    #[test]
    fn simple_select_with_where_clause_less_than() {
        let select = "select name, country from population where id<100";
        let select: SqlCommand = Select::parse(String::from(select));
/*        let clause: WhereClause = select.where_clause;
        assert_eq!(select.table_name, "population");
        assert_eq!(select.columns.len(), 2);
        assert_eq!(clause.get_operator(), Operator::LESSER);*/
    }

    #[test]
    fn simple_select_with_where_clause_greater_than() {
        let select = "select name, country from population where id>100";
        let select: SqlCommand = Select::parse(String::from(select));
/*        let clause: WhereClause = select.where_clause;
        assert_eq!(select.table_name, "population");
        assert_eq!(select.columns.len(), 2);
        assert_eq!(clause.get_operator(), Operator::GREATER);*/
    }

    #[test]
    fn simple_select_without_where_clause() {
        let select = "SELECT name, country FROM population";
        let select: SqlCommand = Select::parse(String::from(select));
/*        let clause: WhereClause = select.where_clause;
        assert_eq!(select.table_name, "population");
        assert_eq!(select.columns.len(), 2);
        assert_eq!(clause.get_operator(), Operator::UNDEFINED);*/
    }
}
