use crate::database::bptree::BPlusTree;
use crate::database::datatype::DataType;
use crate::database::table::{Row, Table};
use crate::file::mtdhandler::MtdFile;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom, Write};
use anyhow::anyhow;
use log::error;


pub fn load_moi_file(mtd: &MtdFile) -> Result<Table, Error> {
    let mut table = Table::default();
    let mut tree: BPlusTree<i64, Vec<DataType>, 3> = BPlusTree::default();
    table.db_name = mtd.dbname.clone();
    table.table_name = mtd.tablename.clone();
    table.column_names = mtd.column_names.clone();
    table.column_types = mtd.column_type_definitions.clone();
    table.display_order = mtd.display_order.clone();
    table.constraint = mtd.column_constraints.clone();

    let column_defs = &mtd.column_type_definitions;
    let moi_file = &mtd.moi_files[0];

    let mut i64_buffer = [0u8; std::mem::size_of::<i64>()];
    let mut u64_buffer = [0u8; std::mem::size_of::<u64>()];
    let mut f64_buffer = [0u8; std::mem::size_of::<f64>()];
    let mut i32_buffer = [0u8; std::mem::size_of::<i32>()];
    let mut f32_buffer = [0u8; std::mem::size_of::<f32>()];
    let mut i16_buffer = [0u8; std::mem::size_of::<i16>()];
    let mut i8_buffer = [0u8; std::mem::size_of::<i8>()];
    let mut u8_buffer = [0u8; std::mem::size_of::<u8>()];

    let mut input = BufReader::new(File::open(moi_file).expect("Failed to open file"));

    let res = input.read_exact(&mut i64_buffer);
    match res {
        Ok(_) => {
            table.max_id = i64::from_le_bytes(i64_buffer);
        },
        Err(_) => {error!("Unable to write the id to a moi file")}
    }


    input.read_exact(&mut u8_buffer);
    u8::from_le_bytes(u8_buffer);

    input.read_exact(&mut i64_buffer);
    let number_of_lines = i64::from_le_bytes(i64_buffer);

    input.read_exact(&mut u8_buffer);
    u8::from_le_bytes(u8_buffer);

    for _ in 0..number_of_lines {
        let mut row: Vec<DataType> = Vec::new();
        for column_counter in 0..column_defs.len() {
            let column_datatype = &column_defs[column_counter];

            match column_datatype {
                DataType::BigInt(..) => {
                    let res = input.read_exact(&mut i64_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a BigInt but got nothing"
                        }
                        _ => "Unable to parse BigInt",
                    };
                    let bigint = i64::from_le_bytes(i64_buffer);
                    row.push(DataType::BigInt(bigint));
                }
                DataType::Int(..) => {
                    let res = input.read_exact(&mut i32_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a Int but got nothing"
                        }
                        _ => "Unable to parse Int",
                    };
                    let int = i32::from_le_bytes(i32_buffer);
                    row.push(DataType::Int(int));
                }
                DataType::SmallInt(..) => {
                    let res = input.read_exact(&mut i16_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a SmallInt but got nothing"
                        }
                        _ => "Unable to parse SmallInt",
                    };
                    let small = i16::from_le_bytes(i16_buffer);
                    row.push(DataType::SmallInt(small));
                }
                DataType::TinyInt(..) => {
                    let res = input.read_exact(&mut i8_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a TinyInt but got nothing"
                        }
                        _ => "Unable to parse SmallInt",
                    };
                    let tiny = i8::from_le_bytes(i8_buffer);
                    row.push(DataType::TinyInt(tiny));
                }
                DataType::Decimal(..) => {
                    let res = input.read_exact(&mut f32_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a Decimal but got nothing"
                        }
                        _ => "Unable to parse Decimal",
                    };
                    let decimal = f32::from_le_bytes(f32_buffer);
                    row.push(DataType::Decimal(decimal));
                }
                DataType::Float(..) => {
                    let res = input.read_exact(&mut f64_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a Float but got nothing"
                        }
                        _ => "Unable to parse Float",
                    };
                    let float = f64::from_le_bytes(f64_buffer);
                    row.push(DataType::Float(float));
                }
                DataType::VarChar(..) => {
                    let res = input.read_exact(&mut u8_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a Boolean but got nothing"
                        }
                        _ => "Unable to parse Boolean",
                    };
                    let length = u8::from_le_bytes(u8_buffer);

                    let mut varchar = String::new();
                    for _ in 0..length {
                        let res = input.read_exact(&mut u8_buffer);
                        match res {
                            Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                                "Trying to read a Char"
                            }
                            _ => "Unable to parse VarChar",
                        };
                        let character: char = u8::from_le_bytes(u8_buffer) as char;
                        varchar.push(character);
                    }
                    row.push(DataType::VarChar(length, varchar));
                }
                DataType::Bool(..) => {
                    let res = input.read_exact(&mut i8_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a Boolean but got nothing"
                        }
                        _ => "Unable to parse Boolean",
                    };
                    let boolean = i8::from_le_bytes(i8_buffer);
                    let mut result: bool;
                    result = false;
                    if boolean > 0 {
                        result = true;
                    }
                    row.push(DataType::Bool(result));
                }
                DataType::Date(..) => {
                    let res = input.read_exact(&mut u64_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a BigInt but got nothing"
                        }
                        _ => "Unable to parse BigInt",
                    };
                    let date = u64::from_le_bytes(u64_buffer);
                    row.push(DataType::Date(date));
                }
                DataType::Time(..) => {
                    let res = input.read_exact(&mut u64_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a BigInt but got nothing"
                        }
                        _ => "Unable to parse BigInt",
                    };
                    let time = u64::from_le_bytes(u64_buffer);
                    row.push(DataType::Time(time));
                }
                DataType::DateTime(..) => {
                    let res = input.read_exact(&mut u64_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                            "We expected a BigInt but got nothing"
                        }
                        _ => "Unable to parse BigInt",
                    };
                    let datetime = u64::from_le_bytes(u64_buffer);
                    row.push(DataType::Time(datetime));
                }
                _ => {
                    //return Err(Error::new(ErrorKind::InvalidData, "Unable to parse data from moi file"));
                }
            }
        }
        input.read_exact(&mut u8_buffer);
        u8::from_le_bytes(u8_buffer);

        let row_id = match row[0] {
            DataType::BigInt(t) => { t }
            _ => { -1 }
        };

        tree.insert(row_id, row);
    }
    table.tree = tree;
    Ok(table)
}

pub fn get_max_id(path:& str) -> i64{
    let mut i64_buffer = [0u8; std::mem::size_of::<i64>()];
    let mut input = BufReader::new(File::open(path).expect("Failed to open file"));
    input.read_exact(&mut i64_buffer);
    i64::from_le_bytes(i64_buffer)
}

///This function increments the max id, after a new row has been added
pub fn add_row(path: &str, row: Row) -> anyhow::Result<()>{
    let mut i64_buffer = [0u8; std::mem::size_of::<i64>()];
    let mut u8_buffer = [0u8; std::mem::size_of::<u8>()];

    //read max id
    let mut input = BufReader::new(File::open(path).expect("Failed to open file"));
    input.read_exact(&mut i64_buffer);
    let _max_id = i64::from_le_bytes(i64_buffer);

    //read new line
    input.read_exact(&mut u8_buffer);
    u8::from_le_bytes(u8_buffer);

    //read number of rows stored in file
    input.read_exact(&mut i64_buffer);
    let number_of_lines = i64::from_le_bytes(i64_buffer);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();


    let pos_id = file.seek(SeekFrom::Start(0));
    match pos_id {
        Ok(_) => {
            let id:DataType = row.data[0].clone();
            match id {
                DataType::BigInt(number) => {
                    let res = file.write_all(&number.to_le_bytes());
                    match res{
                        Ok(_) => {},
                        Err(_) => {error!("Unable to write the id to a moi file")}
                    }
                }
                _ => {}
            }
        },
        Err(_) => todo!(),
    }

    let new_no_lines = number_of_lines +1;
    let pos_lines = file.seek(SeekFrom::Start(9));
    match pos_lines {
        Ok(_) => {
            let res = file.write_all(&new_no_lines.to_le_bytes());
            match res{
                Ok(_) => {},
                Err(_) => {error!("Unable to write a number of lines to a moi file")}
            }
        },
        Err(_) => error!("Unable to update number of lines")
    }

    for i in 0 .. row.data.len(){
        let cell = &row.data[i];

        match cell {
            DataType::BigInt( number) => {
                let pos = file.seek(SeekFrom::End(0));
                match pos {
                    Ok(_) => {
                        let res = file.write_all(&number.to_le_bytes());
                        match res{
                            Ok(_) => {},
                            Err(_) => {error!("Unable to write a BigInt to a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }

            }
            DataType::Int (number) => {
                let pos = file.seek(SeekFrom::End(0));
                match pos {
                    Ok(_)=> {
                        let res = file.write_all(&number.to_le_bytes());
                        match res{
                            Ok(_) => {},
                            Err(_) => {error!("Unable to write a Int to a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }

            },
            DataType::SmallInt (number) => {
                let pos = file.seek(SeekFrom::End(0));
                match pos {
                    Ok(_) => {
                        let res = file.write_all(&number.to_le_bytes());
                        match res{
                            Ok(_) => {},
                            Err(_) => {error!("Unable to write a SmallInt to a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }
            },
            DataType::TinyInt (number) => {
                let pos = file.seek(SeekFrom::End(0));
                match pos {
                    Ok(_) => {
                        let res = file.write_all(&number.to_le_bytes());
                        match res{
                            Ok(_) => {},
                            Err(_) => {error!("Unable to write a TinyInt to a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }
            },
            DataType::Decimal (number) => {
                let pos = file.seek(SeekFrom::End(0));
                match pos {
                    Ok(_) => {
                        let res = file.write_all(&number.to_le_bytes());
                        match res{
                            Ok(_) => {},
                            Err(_) => {error!("Unable to write a Decimal to a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }
            },
            DataType::Float (number) => {
                let pos = file.seek(SeekFrom::End(0));
                match pos{
                    Ok(_) => {
                        let res = file.write_all(&number.to_le_bytes());
                        match res{
                            Ok(_) => {},
                            Err(_) => {error!("Unable to write a Float to a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }
            },
            DataType::VarChar (_len, text) => {
                let number_of_chars = text.chars().count() as u8;
                let pos = file.seek(SeekFrom::End(0));
                match pos{
                    Ok(_) => {
                        file.write_all(&number_of_chars.to_le_bytes()).expect("unable to write to disc");
                        let pos = file.seek(SeekFrom::End(0));
                        match pos{
                            Ok(_) => {
                                file.write_all((&text).as_ref()).expect("unable to write to disc");
                            },
                            Err(_) => {error!("Unable to get the correct position in a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }
            },
            DataType::Bool (bool) => {
                let mut bool_value:u8 = 0;
                if *bool {
                    bool_value = 1;
                }
                let pos = file.seek(SeekFrom::End(0));
                match pos{
                    Ok(_) => {
                        let res = file.write_all(&bool_value.to_le_bytes());
                        match res{
                            Ok(_) => {},
                            Err(_) => {error!("Unable to write a Bool to a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }

            },
            DataType::Date (date) => {
                let pos = file.seek(SeekFrom::End(0));
                match pos {
                    Ok(_) => {
                        let res = file.write_all(&date.to_le_bytes());
                        match res{
                            Ok(_) => {},
                            Err(_) => {error!("Unable to write a date to a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }
            },
            DataType::Time (date) => {
                let pos = file.seek(SeekFrom::End(0));
                match pos {
                    Ok(_) => {
                        let res = file.write_all(&date.to_le_bytes());
                        match res{
                            Ok(_) => {},
                            Err(_) => {error!("Unable to write a Time to a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }
            },
            DataType::DateTime (date) => {
                let pos = file.seek(SeekFrom::End(0));
                match pos {
                    Ok(_) => {
                        let res = file.write_all(&date.to_le_bytes());
                        match res{
                            Ok(_) => {},
                            Err(_) => {error!("Unable to write a DateTime to a moi file")}
                        }
                    },
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }

            },
            DataType::Null => {
                let nul = "u{2400}";
                let pos = file.seek(SeekFrom::End(0));
                match pos {
                    Ok(_) => {
                         file.write_all((&nul).as_ref()).expect("unable to write NULL value");
                    }
                    Err(_) => {error!("Unable to get the correct position in a moi file")}
                }

            }
            _ => {}
        }
    }
    file.write(b"\n").expect("unable to write to disc");
    let flush_res = file.flush();
    match flush_res{
        Ok(_) => anyhow::Ok(()),
        Err(_) =>  Err(anyhow!("Unable to write moi file"))
    }
}

#[cfg(test)]
mod tests {
    use crate::file::moihandler::{add_row, load_moi_file};
    use crate::file::mtdhandler::read_mtd_file;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use log::error;

    #[test]
    fn test_read_max_id() {
        //add_row("C:\\MiaSql\\system\\database.moi");
    }

    #[test]
    fn create_user_table_moi() {
        let file_result = File::create("C:\\MiaSql\\system\\user.moi");

        match file_result {
            Ok(file_item) => {
                let mut writer = BufWriter::new(file_item);
                let max_id: u64 = 2;
                writer.write(&max_id.to_le_bytes()).expect("unable to write to disc");
                writer.write(b"\n").expect("unable to write to disc");

                let lines: u64 = 2;
                writer.write(&lines.to_le_bytes()).expect("unable to write to disc");
                writer.write(b"\n").expect("unable to write to disc");


                let id_1: i64 = 1 as i64;
                let id_2: i64 = 2 as i64;
                let username_1: String = String::from("admin");
                let username_2: String = String::from("daniel");
                let forename_1: String = String::from("");
                let forename_2: String = String::from("Daniel");
                let familyname_1: String = String::from("");
                let familyname_2: String = String::from("Laible");
                let type_1: String = String::from("admin");
                let type_2: String = String::from("user");

                writer.write(&id_1.to_le_bytes()).expect("unable to write to disc");

                let number_of_chars_username = username_1.chars().count() as u8;
                writer
                    .write_all(&number_of_chars_username.to_le_bytes())
                    .expect("unable to write to disc");
                writer.write_all((&username_1).as_ref()).expect("unable to write to disc");

                let number_of_chars_forename_1 = forename_1.chars().count() as u8;
                writer
                    .write_all(&number_of_chars_forename_1.to_le_bytes())
                    .expect("unable to write to disc");
                writer.write_all((&forename_1).as_ref()).expect("unable to write to disc");

                let number_of_chars_familyname_1 = familyname_1.chars().count() as u8;
                writer
                    .write_all(&number_of_chars_familyname_1.to_le_bytes())
                    .expect("unable to write to disc");
                writer.write_all((&familyname_1).as_ref()).expect("unable to write to disc");

                let number_of_chars_type_1 = type_1.chars().count() as u8;
                writer
                    .write_all(&number_of_chars_type_1.to_le_bytes())
                    .expect("unable to write to disc");
                writer.write_all((&type_1).as_ref()).expect("unable to write to disc");

                writer.write(b"\n").expect("unable to write to disc");

                writer.write(&id_2.to_le_bytes()).expect("unable to write to disc");

                let number_of_chars_username_2 = username_2.chars().count() as u8;
                writer
                    .write_all(&number_of_chars_username_2.to_le_bytes())
                    .expect("unable to write to disc");
                writer.write_all((&username_2).as_ref()).expect("unable to write to disc");

                let number_of_chars_forename_2 = forename_2.chars().count() as u8;
                writer
                    .write_all(&number_of_chars_forename_2.to_le_bytes())
                    .expect("unable to write to disc");
                writer.write_all((&forename_2).as_ref()).expect("unable to write to disc");

                let number_of_chars_familyname_2 = familyname_2.chars().count() as u8;
                writer
                    .write_all(&number_of_chars_familyname_2.to_le_bytes())
                    .expect("unable to write to disc");
                writer.write_all((&familyname_2).as_ref()).expect("unable to write to disc");

                let number_of_chars_type_2 = type_2.chars().count() as u8;
                writer
                    .write_all(&number_of_chars_type_2.to_le_bytes())
                    .expect("unable to write to disc");
                writer.write_all((&type_2).as_ref()).expect("unable to write to disc");

                writer.flush();
            }
            Err(error) => {
                println!("Something went terribly wrong reading user table: {}", error);
            }
        }
    }

        #[test]
        fn create_system_tables_moi() {
            let file_result = File::create("C:\\MiaSql\\system\\tables.moi");

            match file_result {
                Ok(file_item) => {
                    let mut writer = BufWriter::new(file_item);
                    let max_id: u64 = 3;
                    writer.write(&max_id.to_le_bytes()).expect("unable to write to disc");
                    writer.write(b"\n").expect("unable to write to disc");

                    let lines: u64 = 3;
                    writer.write(&lines.to_le_bytes()).expect("unable to write to disc");
                    writer.write(b"\n").expect("unable to write to disc");

                    let mut counter = 0;
                    while counter < lines {
                        let id: i64 = (counter + 1) as i64;
                        let dbname: String = String::from("system");
                        let mut tablename: String = String::new();
                        let mut path: String = String::new();
                        if id == 1 {
                            tablename = String::from("databases");
                            path = String::from("C:\\MiaSql\\system\\database.mtd");
                        } else if id == 2 {
                            tablename = String::from("tables");
                            path = String::from("C:\\MiaSql\\system\\tables.mtd");
                        } else if id == 3 {
                            tablename = String::from("user");
                            path = String::from("C:\\MiaSql\\system\\user.mtd");
                    } else {
                            println!("There's something strange in your neighbourhood ... who do you gonna call");
                        }

                        writer.write(&id.to_le_bytes()).expect("unable to write to disc");

                        let db_as_string: String = String::from(&dbname);
                        let number_of_chars_in_db = db_as_string.chars().count() as u8;
                        writer
                            .write_all(&number_of_chars_in_db.to_le_bytes())
                            .expect("unable to write to disc");
                        writer.write_all((&dbname).as_ref()).expect("unable to write to disc");

                        let table_as_string: String = String::from(&tablename);
                        let number_of_chars_in_table = table_as_string.chars().count() as u8;
                        writer
                            .write_all(&number_of_chars_in_table.to_le_bytes())
                            .expect("unable to write to disc");
                        writer.write_all((&tablename).as_ref()).expect("unable to write to disc");

                        let path_as_string: String = String::from(&path);
                        let number_of_chars_in_table = path_as_string.chars().count() as u8;
                        writer
                            .write_all(&number_of_chars_in_table.to_le_bytes())
                            .expect("unable to write to disc");
                        writer.write_all((&path).as_ref()).expect("unable to write to disc");

                        writer.write(b"\n").expect("unable to write to disc");

                        writer.flush();
                        counter += 1;
                    }
                }
                Err(error) => {
                    println!("Something went terribly wrong reading system tables table: {}", error);
                }
            }
        }

        #[test]
        fn create_system_database_moi() {
            let file_result = File::create("C:\\MiaSql\\system\\database.moi");

            match file_result {
                Ok(file_item) => {
                    let mut writer = BufWriter::new(file_item);
                    let max: u64 = 1;
                    writer.write(&max.to_le_bytes()).expect("unable to write to disc");
                    writer.write(b"\n").expect("unable to write to disc");

                    let lines: u64 = 1;
                    writer.write(&lines.to_le_bytes()).expect("unable to write to disc");
                    writer.write(b"\n").expect("unable to write to disc");

                    let mut counter = 0;
                    while counter < 1 {
                        let id: i64 = (counter + 1) as i64;
                        let text: String = String::from("system");

                        writer
                            .write_all(&id.to_le_bytes())
                            .expect("unable to write to disc");

                        let text_as_string: String = String::from(&text);
                        let number_of_chars = text_as_string.chars().count() as u8;
                        writer
                            .write_all(&number_of_chars.to_le_bytes())
                            .expect("unable to write to disc");

                        writer
                            .write_all((&text).as_ref())
                            .expect("unable to write to disc");

                        writer.write_all(b"\n");
                        counter += 1;
                    }
                    writer.flush();
                }
                Err(error) => {
                    println!("Something went terribly wrong reading system database tables: {}", error);
                }
            }
        }

        #[test]
        fn test_read_mtd_file() {
            let foo = read_mtd_file("C:\\MiaSql\\system\\database.mtd");
            println!("{:?}", foo);
            let bar = load_moi_file(&foo);
            match bar {
                Ok(_) => {println!("{}", bar.unwrap())},
                Err(e) => println!("test failed: {:?}", e)
            }
            println!("{:?}", foo);
        }

    #[test]
    fn test_read_moi_file() {
        let foo = read_mtd_file("C:\\MiaSql\\system\\database.mtd");
        let bar = load_moi_file(&foo);
        println!("{:?}", bar.unwrap())
    }


        #[test]
        fn test_fill_varchar() {
            let result = fill_varchar("test".to_string(), 5);
            assert_eq!(result, "test ");

            let result = fill_varchar("test".to_string(), 4);
            assert_eq!(result, "test");

            let result = fill_varchar("test".to_string(), 3);
            assert_eq!(result, "tes");
        }

        fn fill_varchar(varchar: String, len: u16) -> String {
            if len == 0 || varchar.len() == 0 {
                return String::new();
            }

            let varchar_length: u16 = varchar.len() as u16;
            if varchar_length < len {
                let number_of_spaces = len - varchar_length;

                let mut result: String = varchar.clone();
                for _ in 0..number_of_spaces {
                    result = result + " ";
                }
                return result;
            } else if varchar_length > len {
                let mut result = String::new();
                let length_as_usize = len as usize;

                for i in 0..=length_as_usize {
                    if i == 0 {
                        result = result + &varchar[..i];
                    } else {
                        result = result + &varchar[i - 1..i];
                    }
                }
                return result;
            }
            varchar
        }
    }


