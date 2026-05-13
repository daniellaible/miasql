use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::command::whereclause::WhereClause;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub struct Select {
    table_name: String,
    columns: Vec<String>,
    where_clause: WhereClause,
}

impl Command for Select {
    fn parse(mut stmt: String) -> SqlCommand {
        stmt = stmt.to_uppercase();
        let columns = get_columns(&stmt);
        let table = get_table_name(&stmt)
            .unwrap_or_else(|| "doof")
            .parse()
            .unwrap();
        if check_for_where(&stmt) {
            let clause = WhereClause::parse(&stmt);
            let command = SqlCommand::SELECT {
                command: String::from("SELECT"),
                table,
                columns,
                values: Vec::new(),
                where_clause: clause,
            };

            println!("{:#?}", command);
            command
        } else {
            let command = SqlCommand::SELECT {
                command: String::from("SELECT"),
                table,
                columns,
                values: Vec::new(),
                where_clause: WhereClause::default(),
            };
            command
        }
    }
}

fn get_columns(stmt: &String) -> Vec<String> {
    let regex = Regex::new(r"(?i)\bselect\b\s*([\s\S]*?)\s+\bfrom\b\s+").unwrap();
    let captures = regex.captures(stmt).unwrap();
    let columns_as_string = captures.get(1).unwrap().as_str();
    let single_column: Vec<&str> = columns_as_string.split(",").collect();
    let mut parts: Vec<String> = Vec::new();
    for mut single_column in single_column {
        single_column = single_column.trim();
        parts.push(single_column.to_string());
    }
    parts
}

fn get_table_name(stmt: &str) -> Option<&str> {
    if stmt.contains("WHERE") {
        let re = Regex::new(r"(?i)\bfrom\b\s+([^\s;]+)(?:\s+\bwhere\b\s+[\s\S]*)?$").unwrap();
        re.captures(stmt).and_then(|c| c.get(1).map(|m| m.as_str()))

    }else{
        let splits: Vec<_> =  stmt.split(" FROM ").collect();
        let table_name: Option<&str> = Some(splits[1]);
        table_name
    }

}

fn check_for_where(stmt: &String) -> bool {
    let reg = Regex::new(r"(?i)\swhere\s").unwrap();
    if reg.is_match(stmt) {
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
    use crate::command::sqloperator::Operator;
    use crate::database::datatype::DataType;

    #[test]
    fn simple_select_with_where_clause() {
        let statement = "SELECT name, country FROM population WHERE id=1";
        let select: SqlCommand = Select::parse(String::from(statement));
    }

    #[test]
    fn simple_select_with_where_clause_lowercase() {
        let select = "select name, country from population where id=1";
        let select: SqlCommand = Select::parse(String::from(select));

        match select {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                values,
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
        let select = "select * from users where id = 1";
        let select: SqlCommand = Select::parse(String::from(select));
        println!("{:#?}", select);

        match select {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                values,
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
        let select = "select name, country from population where id<100";
        let select: SqlCommand = Select::parse(String::from(select));

        match select {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                values,
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
        let select = "select name, country from population where id>100";
        let select: SqlCommand = Select::parse(String::from(select));

        match select {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                values,
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
        let select = "SELECT name, country FROM population";
        let select: SqlCommand = Select::parse(String::from(select));

        match select {
            SqlCommand::SELECT {
                command,
                table,
                columns,
                values,
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
}
