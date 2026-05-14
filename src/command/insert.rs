use crate::command::command::Command;
use crate::command::sqlcommands::SqlCommand;
use crate::command::whereclause::WhereClause;
use crate::database::datatype::DataType;
use regex::Regex;
use crate::database::database::Database;

#[derive(Debug)]
pub struct Insert {
    table_name: String,
    columns: Vec<String>,
    values: Vec<Vec<DataType>>,
    where_clause: WhereClause,
}

impl Command for Insert {
    fn parse(stmt: String, dbs: Vec<Database>) -> SqlCommand {
        let table: String = get_table(&stmt);
        let columns: Vec<String> = get_columns(&stmt);
        let values: Vec<Vec<String>> = find_values(&stmt);
        let clause: WhereClause = WhereClause::parse(&stmt);

        SqlCommand::INSERT {
            command: String::from("INSERT"),
            table,
            columns,
            values,
            where_clause: clause,
        }
    }
}

fn find_values(stmt: &String) -> Vec<Vec<String>> {
    let values_re = Regex::new(r"(?i)VALUES\s*(.+)$").unwrap();
    let tuple_re = Regex::new(r"\([^)]*\)").unwrap();
    let field_re = Regex::new(r"'[^']*'|-?\d+(?:\.\d+)?|NULL").unwrap();

    let Some(cap) = values_re.captures(stmt) else {
        return Vec::new();
    };

    let values_part = cap.get(1).unwrap().as_str();

    tuple_re
        .find_iter(values_part)
        .map(|m| {
            let tuple = m.as_str();
            let inner = &tuple[1..tuple.len() - 1];

            field_re
                .find_iter(inner)
                .map(|f| {
                    let s = f.as_str();
                    if s.starts_with('\'') && s.ends_with('\'') {
                        s[1..s.len() - 1].to_string()
                    } else {
                        s.to_string()
                    }
                })
                .collect()
        })
        .collect()
}

fn get_columns(stmt: &String) -> Vec<String> {
    let table_n_columns: &str = retrieve_table_n_columns(&stmt);
    let re = Regex::new(r"\((.*)\)").unwrap();
    let mut temp_columns = Vec::new();
    if let Some(caps) = re.captures(table_n_columns) {
        let inside = caps.get(1).unwrap().as_str();
        temp_columns = inside.split(',').collect::<Vec<&str>>();
    }

    let mut columns: Vec<String> = Vec::new();
    for column in temp_columns.iter() {
        let c = column.trim();
        columns.push(c.to_string());
    }
    columns
}

fn get_table(stmt: &String) -> String {
    let table_n_columns: &str = retrieve_table_n_columns(&stmt);

    //This regex gets you all until the first opening brace
    let regex_table = Regex::new(r"^\s*([^(]*?)\s*(?:\(|$)").unwrap();
    let captures_table = regex_table.captures(table_n_columns).unwrap();
    let table_name = captures_table.get(1).unwrap().as_str();
    String::from(table_name)
}

fn retrieve_table_n_columns(stmt: &String) -> &str {
    let regex_table_n_columns =
        Regex::new(r"(?i)\binsert\b\s*into\b\s*([\s\S]*?)\s+\bvalues\b\s+").unwrap();
    let captures_table_n_columns = regex_table_n_columns.captures(stmt).unwrap();
    let table_n_columns = captures_table_n_columns.get(1).unwrap().as_str();
    table_n_columns
}

impl Insert {
    pub fn default() -> Self {
        Insert {
            table_name: String::default(),
            columns: vec![],
            values: vec![vec![]],
            where_clause: WhereClause::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::command::Command;
    use crate::command::insert::Insert;
    use crate::command::sqlcommands::SqlCommand;
    use crate::command::sqloperator::Operator;
    use crate::database::database::Database;
    use crate::database::datatype::DataType;

    #[test]
    fn simple_select_without_where_clause() {
        let dbs: Vec<Database> = vec!();
        let stmt = "INSERT INTO user (first_name, last_name, age) VALUES ('daniel', 'mayer', 35)";
        let cmd: SqlCommand = Insert::parse(stmt.to_string(), dbs);


        match cmd {
            SqlCommand::INSERT {
                command,
                table,
                columns,
                values: _,
                where_clause,
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "user");
                assert_eq!(columns, vec!["first_name", "last_name", "age"]);

                let clause = where_clause;
                assert_eq!(clause.get_operator(), Operator::UNDEFINED);
                assert_eq!(clause.get_column(), "");
                assert_eq!(clause.get_value(), DataType::Undefined);
            }
            _ => (),
        }
    }

    #[test]
    fn simple_select_with_where_clause() {
        let dbs: Vec<Database> = vec!();
        let select = "INSERT INTO user (first_name, last_name, age) VALUES ('Daniel', 'Mayer', '35') where id=1";
        let cmd: SqlCommand = Insert::parse(String::from(select), dbs);

        match cmd {
            SqlCommand::INSERT {
                command,
                table,
                columns,
                values,
                where_clause,
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "user");
                assert_eq!(columns, vec!["first_name", "last_name", "age"]);
                assert_eq!(values, vec![vec!["Daniel", "Mayer", "35"]]);
                println!("{:?}", values);

                let clause = where_clause;
                assert_eq!(clause.get_operator(), Operator::EQUAL);
                assert_eq!(clause.get_column(), "ID");
                assert_eq!(clause.get_value(), DataType::BigInt { x: 1 });
            }
            _ => (),
        }
    }

    #[test]
    fn select_with_multiple_value_tupels() {
        let dbs: Vec<Database> = vec!();
        let select = "INSERT INTO user (firstname, lastname, sex) VALUES ('max', 'maxwell', 1.75),('susie', 'sorglos', 'female'), ('hermann', 'etrusker', 'male')";
        let cmd: SqlCommand = Insert::parse(String::from(select), dbs);

        match cmd {
            SqlCommand::INSERT {
                command,
                table,
                columns,
                values: _,
                where_clause: _,
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "user");
                assert_eq!(columns, vec!["firstname", "lastname", "sex"]);
            }
            _ => (),
        }
    }

    // This should be possible as well
    // Copy all columns from one table to another table:
    //      INSERT INTO target_table
    //          SELECT * FROM source_table
    //          WHERE condition;
    //
    // Copy only some columns from one table to another table:
    //      INSERT INTO target_table (column1, column2, column3, ...)
    //          SELECT column1, column2, column3, ...
    //          FROM source_table
    //      WHERE condition;
}
