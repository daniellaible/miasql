use std::arch::x86_64::_mm256_mask_cmp_epi16_mask;
use crate::command::sqlcommands::SqlCommand;
use crate::database::datatype::DataType;
use crate::database::tabel::{Row, Tabel};
use crate::file;
use crate::server::dbmem::DbMem;
use crate::server::queue::TransactionContext;
use anyhow::{anyhow, Error};
use sqlparser::ast::{Expr, Insert, SetExpr, TableObject, Value};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use log::error;
use crate::file::moihandler;
use crate::file::mtdhandler::read_mtd_file;

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

pub fn insert_into(
    transaction: &TransactionContext,
    table: &String,
    columns: &Vec<String>,
    values: &Vec<Vec<String>>,
) -> anyhow::Result<TransactionContext, Error> {
    let result_append_to_ledger = file::ledgerhandler::append_to_file(
        &transaction.user,
        &transaction.command,
        &transaction.db_name,
    );

    if result_append_to_ledger.is_err() {
        return Err(anyhow!("Unable to update moi file for insert command"));
    }

    let table_arc = match DbMem::find_table_in_mem(transaction.db_name.as_str(), table.as_str()) {
        Some(table_arc) => table_arc,
        None => {
            DbMem::load_table_from_system_tables(transaction.db_name.as_str(), table.as_str());
            DbMem::find_table_in_mem(transaction.db_name.as_str(), table.as_str())
                .ok_or_else(|| anyhow!("Unable to find table in memory"))?
        }
    };

    let mut mtd_path = {
        let table_guard = table_arc
            .lock()
            .map_err(|_| anyhow!("Table lock poisoned while reading mtd_path"))?;
        table_guard.mtd_path.clone()
    };

    if mtd_path.is_empty(){
        match DbMem::load_table_from_system_tables(transaction.db_name.as_str(), table.as_str()){
            Ok(path_to_mtd) => {
                mtd_path = path_to_mtd;
            }
            Err(_) => {
                return Err(anyhow!("Unable to load table into memory"))
            }
        }
    }

    // Lock only to snapshot metadata, then release immediately
    let (column_names_mem, column_types_mem) = {
        let table_guard = table_arc
            .lock()
            .map_err(|_| anyhow!("Table lock poisoned while reading metadata"))?;
        (table_guard.column_names.clone(), table_guard.column_types.clone())
    };

    // Validate columns and build index map
    let mut column_indexs: Vec<usize> = Vec::new();
    for (input_col_pos, input_col_name) in columns.iter().enumerate() {
        let Some(schema_idx) = column_names_mem.iter().position(|c| c == input_col_name) else {
            return Err(anyhow!("Unknown column '{}'", input_col_name));
        };
        // keep same ordering as incoming values
        if input_col_pos < columns.len() {
            column_indexs.push(schema_idx);
        }
    }

    let types: Vec<DataType> = column_indexs
        .iter()
        .map(|idx| column_types_mem[*idx].clone())
        .collect();

    let mut typed_data: VecDeque<DataType> = VecDeque::new();
    for row_vals in values {
        if row_vals.len() != types.len() {
            return Err(anyhow!(
                "Value count ({}) does not match column count ({})",
                row_vals.len(),
                types.len()
            ));
        }

        for (j, raw) in row_vals.iter().enumerate() {
            let dt = match &types[j] {
                DataType::BigInt(_) => DataType::BigInt(raw.parse::<i64>()?),
                DataType::Int(_) => DataType::Int(raw.parse::<i32>()?),
                DataType::SmallInt(_) => DataType::SmallInt(raw.parse::<i16>()?),
                DataType::TinyInt(_) => DataType::TinyInt(raw.parse::<i8>()?),
                DataType::Decimal(_) => DataType::Decimal(raw.parse::<f32>()?),
                DataType::Float(_) => DataType::Float(raw.parse::<f64>()?),
                DataType::VarChar(_, _) => DataType::VarChar(raw.len() as u8, raw.clone()),
                DataType::Bool(_) => DataType::Bool(raw.eq_ignore_ascii_case("true")),
                DataType::Date(_) => DataType::Date(raw.parse::<u64>()?),
                DataType::Time(_) => DataType::Time(raw.parse::<u64>()?),
                DataType::DateTime(_) => DataType::DateTime(raw.parse::<u64>()?),
                DataType::Null => DataType::Null,
                DataType::Undefined => DataType::Undefined,
                DataType::Enum(_, _) => todo!(),
            };
            typed_data.push_back(dt);
        }
    }

    let mut data_for_row: Vec<DataType> = Vec::new();
    for i in 0..column_names_mem.len() {
        if !column_indexs.contains(&i) {
            data_for_row.push(DataType::Null);
        } else {
            let Some(v) = typed_data.pop_front() else {
                return Err(anyhow!("Internal error while building row data"));
            };
            data_for_row.push(v);
        }
    }

    let mois = read_mtd_file(&mtd_path).moi_files;
    if let Some(path) = mois.last() {
        let row = Row { data: data_for_row.clone() };
        moihandler::add_row(path, row);
    } else {
        error!("No moi file found for table '{}'", table);
    }

    // This locks table internally, but we are NOT holding table_guard now
    DbMem::insert_row(&transaction.db_name, table, data_for_row.clone());
    Ok(transaction.clone())
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
