
use crate::database::table::Table;
use crate::file::mtdreader::MtdFile;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Error, Read};
use std::path::Path;
use crate::database::datatype::DataType;

pub fn load_moi_file(mtd: &MtdFile) -> Result<Table, Error> {
    let mut table = Table::default();

    let column_defs = &mtd.column_type_definitions;

    let mut max_id: u64 = 0;
    let moi_file = &mtd.moi_files[0];

    let mut input = BufReader::new(
        File::open(moi_file) .expect("Failed to open file")
    );
    let mut i64_buffer = [0u8; std::mem::size_of::<i64>()];
    let mut f64_buffer = [0u8; std::mem::size_of::<f64>()];
    let mut i32_buffer = [0u8; std::mem::size_of::<i32>()];
    let mut f32_buffer = [0u8; std::mem::size_of::<f32>()];
    let mut i16_buffer = [0u8; std::mem::size_of::<i16>()];
    let mut i8_buffer = [0u8; std::mem::size_of::<i8>()];
    let mut u8_buffer = [0u8; std::mem::size_of::<u8>()];


    let res = input.read_exact(&mut i64_buffer);
    match res {
        Err(error) if error.kind() == ErrorKind::UnexpectedEof => "Why did we stop?",
        _ => { "doof" }
    };
    let max_id = i64::from_le_bytes(i64_buffer);
    println!("{:?}", max_id);


    let res = input.read_exact(&mut i8_buffer);
    match res {
        Err(error) if error.kind() == ErrorKind::UnexpectedEof => "There should be a new line character?",
        _ => { "doof" }
    };
    let newLine = i8::from_le_bytes(i8_buffer);
    println!("There is a new line {:?}", newLine);


    for columnRunner in 0.. mtd.number_of_columns {
        let mut column: Vec<DataType> = Vec::new();
        let res = input.read_exact(&mut i64_buffer);
        match res {
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => "There should be an id?",
            _ => { "doof" }
        };
        let id = i64::from_le_bytes(i64_buffer);
        column.push(DataType::BigInt(id));

        for i in 1..column_defs.len() {
            let column_datatype = &column_defs[i];

            let x = match column_datatype {
                DataType::BigInt(..) => {
                    let res = input.read_exact(&mut i64_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => "We expected a BigInt but got nothing",
                        _ => {
                            "Unable to parse BigInt"
                        }
                    };
                    let bigint = i64::from_le_bytes(i64_buffer);
                    column.push(DataType::BigInt(bigint));
                }

                DataType::Int(..) => {
                    let res = input.read_exact(&mut i32_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => "We expected a Int but got nothing",
                        _ => {
                            "Unable to parse Int"
                        }
                    };
                    let int = i32::from_le_bytes(i32_buffer);
                    column.push(DataType::Int(int));
                }

                DataType::SmallInt(..) => {
                    let res = input.read_exact(&mut i16_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => "We expected a SmallInt but got nothing",
                        _ => {
                            "Unable to parse SmallInt"
                        }
                    };
                    let small = i16::from_le_bytes(i16_buffer);
                    column.push(DataType::SmallInt(small));
                }

                DataType::TinyInt(..) => {
                    let res = input.read_exact(&mut i8_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => "We expected a TinyInt but got nothing",
                        _ => {
                            "Unable to parse SmallInt"
                        }
                    };
                    let tiny = i8::from_le_bytes(i8_buffer);
                    column.push(DataType::TinyInt(tiny));
                }

                DataType::Decimal(..) => {
                    let res = input.read_exact(&mut f32_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => "We expected a Decimal but got nothing",
                        _ => {
                            "Unable to parse Decimal"
                        }
                    };
                    let decimal = f32::from_le_bytes(f32_buffer);
                    column.push(DataType::Decimal(decimal));
                }

                DataType::Float(..) => {
                    let res = input.read_exact(&mut f64_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => "We expected a Float but got nothing",
                        _ => {
                            "Unable to parse Float"
                        }
                    };
                    let float = f64::from_le_bytes(f64_buffer);
                    column.push(DataType::Float(float));
                }

                DataType::Bool(..) => {
                    let res = input.read_exact(&mut i8_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => "We expected a Boolean but got nothing",
                        _ => {
                            "Unable to parse Boolean"
                        }
                    };
                    let boolean = i8::from_le_bytes(i8_buffer);
                    let mut result:bool;
                    result = false;
                    if boolean > 0 {
                        result = true;
                    }
                    column.push(DataType::Bool(result));
                }

                DataType::VarChar(..) => {
                    let res = input.read_exact(&mut u8_buffer);
                    match res {
                        Err(error) if error.kind() == ErrorKind::UnexpectedEof => "We expected a Boolean but got nothing",
                        _ => {
                            "Unable to parse Boolean"
                        }
                    };
                    let length = u8::from_le_bytes(u8_buffer);

                    let mut varchar = String::new();
                    for  j in 0 .. length {
                        let res = input.read_exact(&mut u8_buffer);
                        match res {
                            Err(error) if error.kind() == ErrorKind::UnexpectedEof => "Trying to read a Char",
                            _ => {
                                "Unable to parse VarChar"
                            }
                        };
                        let character:char = u8::from_le_bytes(u8_buffer) as char;
                        varchar.push(character);
                    }
                    column.push(DataType::VarChar(length, varchar));

                }

                _ => {
                    //return Err(Error::new(ErrorKind::InvalidData, "Unable to parse data from moi file"));
                }
            };
        }
        let newLine = i8::from_le_bytes(i8_buffer);
        println!("column: {:#?}", column);
    }

    return Ok(Table::default());
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
            use crate::file::moireader::load_moi_file;
            use crate::file::mtdreader::read_mtd_file;
            use std::fs::File;
            use std::io::{BufWriter, Write};

            #[test]
            fn test_read_mtd_file() {
                let foo = read_mtd_file("C:\\MiaSql\\system\\database.mtd");
                load_moi_file(&foo);
                println!("{:?}", foo);
            }

            #[test]
            fn test_writing_system_database_file() {
                let file_result = File::create("C:\\MiaSql\\system\\database.moi");

                match file_result {
                    Ok(file_item) => {
                        let mut writer = BufWriter::new(file_item);
                        let counter: u64 = 1;
                        let varchar_length: u8 = 6;

                        writer.write_all(&counter.to_le_bytes());
                        writer.write_all(b"\n");
                        writer.write_all(&counter.to_le_bytes());
                        writer.write_all(&varchar_length.to_le_bytes());
                        writer.write_all(b"system");
                        writer.write_all(b"\n");
                        writer.flush();
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
                let foo = read_mtd_file("C:\\MiaSql\\system\\database.mtd");
                let table = load_moi_file(&foo);
            }

            #[test]
            fn test_write_file_all_datatypes_multi_row() {
                let file_result = File::create("C:\\MiaSql\\system\\test_datatypes.moi");

                match file_result {
                    Ok(file_item) => {
                        let mut writer = BufWriter::new(file_item);
                        let max: u64 = 4;

                        writer.write(&max.to_le_bytes());
                        writer.write(b"\n").expect("TODO: panic message");

                        //line 1
                        let id: i64 = 1;
                        let big_number: i64 = 555555;
                        let number: i32 = 512;
                        let small: i16 = 277;
                        let tiny: i8 = 42;
                        let decimal: f32 = f32::EPSILON;
                        let float: f64 = f64::MIN_POSITIVE;
                        let text: String = String::from("This is a text");
                        let boolean: bool = true;
                        writer
                            .write_all(&id.to_le_bytes())
                            .expect("TODO: panic message");
                        writer
                            .write_all(&big_number.to_le_bytes())
                            .expect("TODO: panic message");
                        writer
                            .write_all(&number.to_le_bytes())
                            .expect("TODO: panic message");
                        writer
                            .write_all(&small.to_le_bytes())
                            .expect("TODO: panic message");
                        writer
                            .write_all(&tiny.to_le_bytes())
                            .expect("TODO: panic message");
                        writer
                            .write_all(&tiny.to_le_bytes())
                            .expect("TODO: panic message");
                        writer
                            .write_all(&big_number.to_le_bytes())
                            .expect("TODO: panic message");
                        writer
                            .write_all(&number.to_le_bytes())
                            .expect("TODO: panic message");

                        writer
                            .write_all(&small.to_le_bytes())
                            .expect("TODO: panic message");
                        writer
                            .write_all(&tiny.to_le_bytes())
                            .expect("TODO: panic message");
                        writer
                            .write_all(&decimal.to_le_bytes())
                            .expect("TODO: panic message");
                        writer
                            .write_all(&float.to_le_bytes())
                            .expect("TODO: panic message");

                        let foo:String = String::from(&text);
                        let number_of_chars = foo.chars().count() as u8;
                        writer
                            .write_all(&number_of_chars.to_le_bytes())
                            .expect("TODO: panic message");

                        writer
                            .write_all((&text).as_ref())
                            .expect("TODO: panic message");
                        if boolean == true {
                            let t: u8 = 1;
                            writer.write_all(&t.to_le_bytes());
                        } else {
                            let t: u8 = 0;
                            writer.write_all(&t.to_le_bytes());
                        }
                        writer.write_all(b"\n");
                        writer.flush();

                        //line 2
                        let second_id: i64 = 2;
                        let big_number: i64 = 555555;
                        let number: i32 = 327777;
                        let small: i16 = 2888;
                        let tiny: i8 = 124;
                        let decimal: f32 = 3.1415;
                        let float: f64 = 3.14;
                        let text: String = String::from("Another text");
                        let boolean: bool = false;
                        writer.write_all(&second_id.to_le_bytes());
                        writer.write_all(&big_number.to_le_bytes()).expect("TODO: panic message");
                        writer.write_all(&number.to_le_bytes());
                        writer.write_all(&small.to_le_bytes());
                        writer.write_all(&tiny.to_le_bytes());
                        writer.write_all(&decimal.to_le_bytes());
                        writer.write_all(&float.to_le_bytes());
                        writer.write_all(&text.len().to_le_bytes());
                        writer.write_all((&text).as_ref());
                        if boolean == true {
                            let t: u8 = 1;
                            writer.write_all(&t.to_le_bytes());
                        } else {
                            let t: u8 = 0;
                            writer.write_all(&t.to_le_bytes());
                        }

                        writer.write_all(b"\n");
                        writer.flush();
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }

                let foo = read_mtd_file("C:\\MiaSql\\system\\test_datatypes.mtd");
                let table = load_moi_file(&foo);
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
                    let mut number_of_spaces = len - varchar_length;

                    let mut result: String = varchar.clone();
                    for i in 0..number_of_spaces {
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
