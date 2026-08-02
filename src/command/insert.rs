use std::collections::VecDeque;
use anyhow::{anyhow, Error};
use crate::command::sqlcommands::SqlCommand;
use sqlparser::ast::{Expr, Insert, SetExpr, TableObject, Value};
use crate::command::createdatabase::update_database_moi;
use crate::database::datatype::DataType;
use crate::database::table::Row;
use crate::file;
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionContext;

pub fn parse(insert: Insert) -> SqlCommand {
    let table = match parse_table(&insert.table) {
        Some(table) => table,
        None => return SqlCommand::Undefined,
    };

    let columns = parse_columns(&insert.columns);

    let values = match parse_values(&insert) {
        Some(values) => values,
        None => return SqlCommand::Undefined,
    };

    SqlCommand::Insert {
        command: String::from("INSERT"),
        table,
        columns,
        values,
    }
}

pub fn insert_into(transaction: &TransactionContext, table: &String, columns: &Vec<String>, values: &Vec<Vec<String>>) -> anyhow::Result<TransactionContext, Error>{
    let ledger_clone_file = transaction.clone();
    let result_append_to_ledger = file::ledgerhandler::append_to_file(
        &ledger_clone_file.user,
        &ledger_clone_file.command,
        &ledger_clone_file.db_name,
    );
    match result_append_to_ledger{
        Ok(_) => {
            let result_moi_update: anyhow::Result<TransactionContext> =  update_database_moi(transaction.clone(), transaction.db_name.to_string());
            match result_moi_update{
                Ok(_) => {
                    //DbMem update
                    if let Some(table_arc) = DbMem::find_table(transaction.table_names[0].as_str(), table.as_str() ){
                        let table_guard = table_arc.lock().unwrap();
                        let column_names_mem = &table_guard.column_names;
                        let column_types_mem = &table_guard.column_types;
                        let btree = &table_guard.tree;

                        let mut column_indexs:Vec<usize> = Vec::new();
                        for i in 0 .. column_names_mem.len() {
                            if column_names_mem[i] == columns[i]{
                                column_indexs.push(i);
                            }
                        }

                        let mut types:Vec<DataType> = Vec::new();
                        for i in 0 .. column_indexs.len(){
                            types.push(column_types_mem[i].clone());
                        }

                        let mut typed_data:VecDeque<DataType> = VecDeque::new();
                        for i in 0 .. values.len() {
                            for j in 0 .. values[i].len() {
                                match types[j]{
                                    DataType::BigInt(_) => {
                                        typed_data.push_back(DataType::BigInt(values[i][j].parse::<i64>().unwrap()));
                                    }
                                    DataType::Int(_) => {
                                        typed_data.push_back(DataType::Int(values[i][j].parse::<i32>().unwrap()));
                                    }
                                    DataType::SmallInt(_) => {
                                        typed_data.push_back( DataType::SmallInt(values[i][j].parse::<i16>().unwrap()));
                                    }
                                    DataType::TinyInt(_) => {
                                        typed_data.push_back(DataType::TinyInt(values[i][j].parse::<i8>().unwrap()));
                                    }
                                    DataType::Decimal(_) => {
                                        typed_data.push_back(DataType::Decimal(values[i][j].parse::<f32>().unwrap()));
                                    }
                                    DataType::Float(_) => {
                                        typed_data.push_back(DataType::Float(values[i][j].parse::<f64>().unwrap()));
                                    }
                                    DataType::VarChar(_, _) => {
                                        let size = values[i][j].len() as u8;
                                        typed_data.push_back(DataType::VarChar(size, values[i][j].clone()));
                                    }
                                    DataType::Bool(_) => {
                                        let b_value = values[i][j].to_lowercase();
                                        if b_value == "true" {
                                            typed_data.push_back(DataType::Bool(true));
                                        }else{
                                            typed_data.push_back(DataType::Bool(false));
                                        }
                                    }
                                    DataType::Date(_) => {
                                        typed_data.push_back(DataType::Date(values[i][j].parse::<u64>().unwrap()));
                                    }
                                    DataType::Time(_) => {
                                        typed_data.push_back(DataType::Date(values[i][j].parse::<u64>().unwrap()))
                                    }
                                    DataType::DateTime(_) => {
                                        typed_data.push_back(DataType::Date(values[i][j].parse::<u64>().unwrap()))
                                    }
                                    DataType::Null => {
                                        typed_data.push_back(DataType::Null);
                                    }
                                    DataType::Undefined => {
                                        typed_data.push_back(DataType::Undefined);
                                    }
                                };
                            }
                        }
                        let mut data_for_row:Vec<DataType> = Vec::new();
                        for i in 0 .. column_names_mem.len(){
                            if !column_indexs.contains(&i){
                                data_for_row.push(DataType::Null);
                            }else{
                                data_for_row.push(typed_data.pop_front().unwrap())
                            }
                        }
                        let row:Row = Row{
                            data: data_for_row
                        };
                        DbMem::insert_row(&transaction.db_name, table, row);
                        Ok(transaction.clone())
                    }else{
                        Ok(transaction.clone()) 
                    }
                }
                Err(_) => {Err(anyhow!("Unable to update moi file for insert command"))}
            }
        }
        Err(_) => {Err(anyhow!("Unable to update ledger for insert command"))}
    }
    
}

fn parse_table(table: &TableObject) -> Option<String> {
    match table {
        TableObject::TableName(name) => Some(
            name.0
                .iter()
                .map(|part| part.to_string())
                .collect::<Vec<_>>()
                .join("."),
        ),
        _ => None,
    }
}

fn parse_columns(columns: &Vec<sqlparser::ast::ObjectName>) -> Vec<String> {
    let mut result = vec![];

    for column in columns {
        let name = column
            .0
            .iter()
            .map(|part| part.to_string())
            .collect::<Vec<_>>()
            .join(".");
        result.push(name);
    }

    result
}

fn parse_values(insert: &Insert) -> Option<Vec<Vec<String>>> {
    let source = insert.source.as_ref()?;

    match source.body.as_ref() {
        SetExpr::Values(values) => {
            let mut result: Vec<Vec<String>> = vec![];

            for row in values.rows.iter() {
                let mut parsed_row: Vec<String> = vec![];

                for expr in row.iter() {
                    let value = parse_expr_value(expr)?;
                    parsed_row.push(value);
                }

                result.push(parsed_row);
            }

            Some(result)
        }
        _ => None,
    }
}

fn parse_expr_value(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Value(vws) => match &vws.value {
            Value::SingleQuotedString(s) => Some(s.clone()),
            Value::DoubleQuotedString(s) => Some(s.clone()),
            Value::Number(n, _) => Some(n.clone()),
            Value::Boolean(b) => Some(b.to_string()),
            Value::Null => Some(String::from("NULL")),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::command::insert::parse;
    use crate::command::sqlcommands::SqlCommand;
    use sqlparser::ast::Statement;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    fn parse_insert(statement: &str) -> SqlCommand {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, statement).unwrap();

        match ast.into_iter().next().unwrap() {
            Statement::Insert(insert) => parse(insert),
            _ => panic!("expected query"),
        }
    }

    #[test]
    fn test_insert_without_columns() {
        let command = crate::command::insert::tests::parse_insert(
            "INSERT INTO Customers VALUES ('Cardinal', 'Stavanger', 'Norway');",
        );

        match command {
            SqlCommand::Insert {
                command,
                table,
                columns,
                values,
                ..
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "Customers");
                assert_eq!(columns.len(), 0);
                assert_eq!(values[0][0], "Cardinal");
                assert_eq!(values[0][1], "Stavanger");
                assert_eq!(values[0][2], "Norway");
            }
            _ => panic!("expected INSERT"),
        }
    }


    #[test]
    fn test_insert_with_columns() {
        let command = crate::command::insert::tests::parse_insert(
            "INSERT INTO Customers (CustomerName, City, Country) VALUES ('Cardinal', 'Stavanger', 'Norway');",
        );

        match command {
            SqlCommand::Insert {
                command,
                table,
                columns,
                values,
                ..
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "Customers");
                assert_eq!(columns.len(), 3);
                assert_eq!(columns[0], "CustomerName");
                assert_eq!(columns[1], "City");
                assert_eq!(columns[2], "Country");
                assert_eq!(values[0][0], "Cardinal");
                assert_eq!(values[0][1], "Stavanger");
                assert_eq!(values[0][2], "Norway");
            }
            _ => panic!("expected INSERT"),
        }
    }

    #[test]
    fn test_insert_multirow() {
        let command = crate::command::insert::tests::parse_insert(
            "INSERT INTO Customers VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway'), ('Greasy Burger', 'Per Olsen', 'Gateveien 15', 'Sandnes', '4306', 'Norway'), ('Tasty Tee', 'Finn Egan', 'Streetroad 19B', 'Liverpool', 'L1 0AA', 'UK');",
        );

        match command {
            SqlCommand::Insert {
                command,
                table,
                columns,
                values,
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "Customers");
                assert_eq!(values[0][0], "Cardinal");
                assert_eq!(values[0][1], "Tom B. Erichsen");
                assert_eq!(values[0][2], "Skagen 21");
                assert_eq!(values[0][3], "Stavanger");
                assert_eq!(values[0][4], "4006");
                assert_eq!(values[0][5], "Norway");
                assert_eq!(values[1][0], "Greasy Burger");
                assert_eq!(values[1][1], "Per Olsen");
                assert_eq!(values[1][2], "Gateveien 15");
                assert_eq!(values[1][3], "Sandnes");
                assert_eq!(values[1][4], "4306");
                assert_eq!(values[1][5], "Norway");
                assert_eq!(values[2][0], "Tasty Tee");
                assert_eq!(values[2][1], "Finn Egan");
                assert_eq!(values[2][2], "Streetroad 19B");
                assert_eq!(values[2][3], "Liverpool");
                assert_eq!(values[2][4], "L1 0AA");
                assert_eq!(values[2][5], "UK");
            }
            _ => panic!("expected INSERT"),
        }
    }

    #[test]
    fn test_insert_multirow_with_columns() {
        let command = crate::command::insert::tests::parse_insert(
            "INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country) VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway'), ('Greasy Burger', 'Per Olsen', 'Gateveien 15', 'Sandnes', '4306', 'Norway'), ('Tasty Tee', 'Finn Egan', 'Streetroad 19B', 'Liverpool', 'L1 0AA', 'UK');",
        );

        match command {
            SqlCommand::Insert {
                command,
                table,
                columns,
                values,
            } => {
                assert_eq!(command, "INSERT");
                assert_eq!(table, "Customers");
                assert_eq!(columns[0], "CustomerName");
                assert_eq!(columns[1], "ContactName");
                assert_eq!(columns[2], "Address");
                assert_eq!(columns[3], "City");
                assert_eq!(columns[4], "PostalCode");
                assert_eq!(columns[5], "Country");
                assert_eq!(values[0][0], "Cardinal");
                assert_eq!(values[0][1], "Tom B. Erichsen");
                assert_eq!(values[0][2], "Skagen 21");
                assert_eq!(values[0][3], "Stavanger");
                assert_eq!(values[0][4], "4006");
                assert_eq!(values[0][5], "Norway");
                assert_eq!(values[1][0], "Greasy Burger");
                assert_eq!(values[1][1], "Per Olsen");
                assert_eq!(values[1][2], "Gateveien 15");
                assert_eq!(values[1][3], "Sandnes");
                assert_eq!(values[1][4], "4306");
                assert_eq!(values[1][5], "Norway");
                assert_eq!(values[2][0], "Tasty Tee");
                assert_eq!(values[2][1], "Finn Egan");
                assert_eq!(values[2][2], "Streetroad 19B");
                assert_eq!(values[2][3], "Liverpool");
                assert_eq!(values[2][4], "L1 0AA");
                assert_eq!(values[2][5], "UK");
            }
            _ => panic!("expected INSERT"),
        }
    }


}
