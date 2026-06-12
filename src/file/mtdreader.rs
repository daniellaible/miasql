use crate::command::constraint::Constraint;
use crate::database::datatype::DataType;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;


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
                let mut names_split = names.split(";");
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
                let mut types_defs__split = type_defs.split(";");

                let mut columns: Vec<DataType> = Vec::new();
                for defs in types_defs__split {
                    if defs.starts_with("VarChar(") {
                        let mut splits = defs.split("(");
                        _ = splits.next();
                        let mut almost = splits.next().unwrap().to_string();
                        almost = almost.replace(")", "");
                        almost = almost.replace("]", "");
                        let length = almost.parse::<u16>().unwrap();
                        columns.push(DataType::VarChar(length as u8, String::default()));
                    }
                    match defs {
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
                let mut column_constraints_split = column_constraints.split(";");
                let mut column_constraints: Vec<(u32, Constraint)> = Vec::new();
                for constraint in column_constraints_split {
                    let mut clean_constraint = constraint.replace("(", "");
                    clean_constraint = clean_constraint.replace(")", "");
                    let mut tuple_splits = clean_constraint.split(",");
                    let left = tuple_splits.next();
                    let right = tuple_splits.next().unwrap();
                    let columnNumber = left.unwrap().trim().parse::<u32>().unwrap();

                    let constraint = match right {
                        "NotNull" => Constraint::NotNull,
                        "Unique" => Constraint::Unique,
                        "PrimaryKey" => Constraint::PrimaryKey,
                        "ForeignKey" => Constraint::ForeignKey,
                        "Check" => Constraint::Check,
                        "Default" => Constraint::Default,
                        _ => Constraint::Undefined,
                    };
                    column_constraints.push((columnNumber, constraint));
                }
                mtd.column_constraints = column_constraints;
            }

            if line.starts_with("displayOrder") {
                let mut splits = line.split("=");
                _ = splits.next();

                let mut display_order_arr = splits.next().unwrap().to_string();
                display_order_arr = display_order_arr.replace("[", "");
                display_order_arr = display_order_arr.replace("]", "");

                let mut single_order_split = display_order_arr.split(";");
                let mut order: Vec<(u32, u32)> = Vec::new();

                for mut order_tuple in single_order_split {
                    let mut order_tuple = order_tuple.replace("(", "").to_string();
                    order_tuple = order_tuple.replace(")", "").to_string();
                    let mut tupel_value = order_tuple.split(",");
                    let left = tupel_value.next().unwrap().trim().parse::<u32>().unwrap();
                    let right = tupel_value.next().unwrap().trim().parse::<u32>().unwrap();
                    order.push((left, right));
                }
                mtd.display_order = order;
            }

            if line.starts_with("moiFiles") {
                let mut splits = line.split("=");
                _ = splits.next();
                let mut moi_files = splits.next().unwrap().to_string();
                moi_files = moi_files.replace("[", "");
                moi_files = moi_files.replace("]", "");
                let mut moi_files_split = moi_files.split(";");
                for moi_file in moi_files_split {
                    mtd.moi_files.push(moi_file.to_string());
                }
            }
        }

        mtd
    }
    else {
        MtdFile::default()
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::import_system_tables;
}

