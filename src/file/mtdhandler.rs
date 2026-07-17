use crate::command::constraint::Constraint;
use crate::command::createtable::ForeignKeyToken;
use crate::database::datatype::DataType;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug)]
pub struct MtdFile {
    pub version: f32,
    pub number_of_columns: i32,
    pub dbname: String,
    pub tablename: String,
    pub column_names: Vec<String>,
    pub column_type_definitions: Vec<DataType>,
    pub column_constraints: Vec<(u32, Constraint)>,
    pub display_order: Vec<(u32, u32)>,
    pub moi_files: Vec<String>,
}

impl MtdFile {
    pub fn default() -> Self {
        MtdFile {
            version: 1.0,
            number_of_columns: 0,
            dbname: String::new(),
            tablename: String::new(),
            column_names: vec![],
            column_type_definitions: vec![],
            column_constraints: vec![],
            display_order: vec![],
            moi_files: vec![],
        }
    }
}

pub fn read_mtd_file(path: &str) -> MtdFile {
    if let Ok(lines) = read_lines(path) {
        let mut mtd: MtdFile = MtdFile::default();
        for line in lines.map_while(Result::ok) {
            if line.starts_with("version") {
                let mut splits = line.split("=");
                _ = splits.next();
                mtd.version = splits.next().unwrap().trim().parse::<f32>().unwrap();
            }

            if line.starts_with("numberOfColumns") {
                let mut splits = line.split("=");
                _ = splits.next().unwrap();
                mtd.number_of_columns = splits.next().unwrap().trim().parse::<i32>().unwrap();
            }

            if line.starts_with("dbname") {
                let mut splits = line.split("=");
                _ = splits.next();
                mtd.dbname = splits.next().unwrap().to_string();
            }

            if line.starts_with("tablename") {
                let mut splits = line.split("=");
                _ = splits.next();
                mtd.tablename = splits.next().unwrap().to_string();
            }

            if line.starts_with("columnNames") {
                let mut splits = line.split("=");
                _ = splits.next();
                let mut names = splits.next().unwrap().to_string();
                names = names.replace("[", "");
                names = names.replace("]", "");
                let names_split = names.split(";");
                let mut columns: Vec<String> = Vec::new();

                for name in names_split {
                    columns.push(name.to_string());
                }
                mtd.column_names = columns;
            }

            if line.starts_with("columnTypeDefinition") {
                let mut splits = line.split("=");
                _ = splits.next();
                let mut type_defs = splits.next().unwrap().to_string();
                type_defs = type_defs.replace("[", "");
                type_defs = type_defs.replace("]", "");
                type_defs = type_defs.replace("(", "");
                type_defs = type_defs.replace(")", "");
                let types_defs_split = type_defs.split(";");

                let mut columns: Vec<DataType> = Vec::new();
                for defs in types_defs_split {
                    match defs {
                        "VarChar" => columns.push(DataType::VarChar(0, String::new())),
                        "BigInt" => columns.push(DataType::BigInt(0)),
                        "Int" => columns.push(DataType::Int(0)),
                        "SmallInt" => columns.push(DataType::SmallInt(0)),
                        "TinyInt" => columns.push(DataType::TinyInt(0)),
                        "Decimal" => columns.push(DataType::Decimal(0.0)),
                        "Float" => columns.push(DataType::Float(0.0)),
                        "Bool" => columns.push(DataType::Bool(false)),
                        "Date" => columns.push(DataType::Date(0)),
                        "Time" => columns.push(DataType::Time(0)),
                        "DateTime" => columns.push(DataType::DateTime(0)),
                        "Null" => columns.push(DataType::Null),
                        _ => {
                            if !defs.starts_with("VarChar") {
                                println!("Unknown type: {}", defs)
                            }
                        }
                    }
                }
                mtd.column_type_definitions = columns;
            }

            if line.starts_with("columnConstraints") {
                let mut splits = line.split("=");
                _ = splits.next();
                let mut column_constraints = splits.next().unwrap().to_string();
                column_constraints = column_constraints.replace("[", "");
                column_constraints = column_constraints.replace("]", "");
                let column_constraints_split = column_constraints.split(";");
                let mut column_constraints: Vec<(u32, Constraint)> = Vec::new();
                for constraint in column_constraints_split {
                    let mut clean_constraint = constraint.replace("(", "");
                    clean_constraint = clean_constraint.replace(")", "");
                    let mut tuple_splits = clean_constraint.split(",");
                    let left = tuple_splits.next();
                    let right = tuple_splits.next().unwrap();
                    let column_number = left.unwrap().trim().parse::<u32>().unwrap();

                    let constraint = match right {
                        "NotNull" => Constraint::NotNull,
                        "Unique" => Constraint::Unique,
                        "PrimaryKey" => Constraint::PrimaryKey,
                        "ForeignKey" => Constraint::ForeignKey,
                        "Check" => Constraint::Check,
                        "Default" => Constraint::Default,
                        _ => Constraint::Undefined,
                    };
                    column_constraints.push((column_number, constraint));
                }
                mtd.column_constraints = column_constraints;
            }

            if line.starts_with("moiFiles") {
                let mut splits = line.split("=");
                _ = splits.next();
                let mut moi_files = splits.next().unwrap().to_string();
                moi_files = moi_files.replace("[", "");
                moi_files = moi_files.replace("]", "");
                let moi_files_split = moi_files.split(";");
                for moi_file in moi_files_split {
                    mtd.moi_files.push(moi_file.to_string());
                }
            }
        }

        mtd
    } else {
        MtdFile::default()
    }
}

pub fn new_mtd_file(
    db_name: &String,
    table: &String,
    columns: &Vec<(String, DataType, Vec<Constraint>)>,
    foreign_keys: &Vec<ForeignKeyToken>,
    uuid: Uuid,
) -> anyhow::Result<()> {
    let path = "C:\\MiaSql\\tables\\".to_owned() + uuid.to_string().as_str() + ".mtd";
    let file_result = File::create(path.clone());

    match file_result {
        Ok(file_item) => {
            let mut writer = BufWriter::new(file_item);
            let version_line = "version=1.0\n";
            writer
                .write_all((&version_line).as_ref())
                .expect("unable to write version_line to disc");

            let num_of_columns =
                "numberOfColumns=".to_owned() + columns.len().to_string().as_str() + "\n";
            writer
                .write_all((&num_of_columns).as_ref())
                .expect("unable to write num_of_columns to disc");

            let dbname = "dbname=".to_owned() + db_name + "\n";
            writer
                .write_all((&dbname).as_ref())
                .expect("unable to write dbname to disc");

            let tablename = "tablename=".to_owned() + table + "\n";
            writer
                .write_all((&tablename).as_ref())
                .expect("unable to write tablename to disc");

            let column_names = "columnNames=[".to_owned();
            writer
                .write_all((&column_names).as_ref())
                .expect("unable to write the start of columnnames to disc");

            for i in 0..columns.len() {
                let column = &columns[i];
                let column_name = &column.0;

                if i == columns.len() - 1 {
                    let single_column_name = column_name.to_owned() + "]\n";
                    writer
                        .write_all((&single_column_name).as_ref())
                        .expect("unable to write the start of single_column_name to disc");
                } else {
                    let single_column_name = column_name.to_owned() + ";";
                    writer
                        .write_all((&single_column_name).as_ref())
                        .expect("unable to write the start of single_column_name to disc");
                }
            }

            let column_names = "columnTypeDefinition=[".to_owned();
            writer
                .write_all((&column_names).as_ref())
                .expect("unable to write the start of columnnames to disc");
            for i in 0..columns.len() {
                let column = &columns[i];
                let column_datatype = &column.1;
                let dt_as_str = match column_datatype {
                    DataType::BigInt(_) => "BigInt",
                    DataType::Int(_) => "Int",
                    DataType::SmallInt(_) => "SmallInt",
                    DataType::TinyInt(_) => "TinyInt",
                    DataType::Decimal(_) => "Decimal",
                    DataType::Float(_) => "Float",
                    DataType::VarChar(_, _) => "VarChar",
                    DataType::Bool(_) => "Bool",
                    DataType::Date(_) => "Date",
                    DataType::Time(_) => "Time",
                    DataType::DateTime(_) => "DateTime",
                    DataType::Null => "Null",
                    DataType::Undefined => "Undefined",
                };
                if i == columns.len() - 1 {
                    let dt_string = dt_as_str.to_owned() + "]\n";
                    writer
                        .write_all((&dt_string).as_ref())
                        .expect("unable to write the start of columnnames to disc");
                } else {
                    let dt_string = dt_as_str.to_owned() + ";";
                    writer
                        .write_all((&dt_string).as_ref())
                        .expect("unable to write the start of columnnames to disc");
                }
            }

            let column_constraints = "columnConstraints=[".to_owned();
            writer
                .write_all((&column_constraints).as_ref())
                .expect("unable to write the start of column_constraints to disc");
            for i in 0..columns.len() {
                let column = &columns[i];
                let constraints = &column.2;
                for j in 0 .. constraints.len(){
                    if i == columns.len() -1  && j == constraints.len()-1 {
                        let constraint = constraints[j].clone();
                        let output = "(".to_owned() + i.to_string().as_str() + "," + constraint.to_string().as_str() + ")]\n";
                        writer
                            .write_all((&output).as_ref())
                            .expect("unable to write the start of output to disc");
                    }else {
                        let constraint = constraints[j].clone();
                        let output = "(".to_owned() + i.to_string().as_str() + "," + constraint.to_string().as_str() + ");";
                        writer
                            .write_all((&output).as_ref())
                            .expect("unable to write the start of output to disc");
                    }
                }
            }

            let moi_path = "moiFiles=[".to_owned() + path.as_str() + "]";
            writer
                .write_all((&moi_path).as_ref())
                .expect("unable to write the start of moi to disc");
        }
        Err(_) => {}
    };

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {}
