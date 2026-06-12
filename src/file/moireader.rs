use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::database::datatype::DataType;
use crate::database::table::Table;
use crate::file::mtdreader::MtdFile;

pub fn load_moi_file(mtd: &MtdFile) -> Table {
    let mut table = Table::default();
    let column_defs = &mtd.column_type_definitions;
    let mut column:Vec<DataType> = vec![];
    let mut max_id: u64 = 0;
    let moi_file = &mtd.moi_files[0];
    if let Ok(lines) = read_lines(moi_file) {
        let mut counter = 0;

        for line in lines.map_while(Result::ok) {
            let lab = line.as_bytes();

            if(counter == 0) {
                max_id =  u64::from_le_bytes(lab[0..8].try_into().unwrap());
            }else{
                //First datatype in the array is always a BigInt id
                let id =  u64::from_le_bytes(lab[0..8].try_into().unwrap());

                let mut byte_count:usize = 8;
                for i in (1 .. column_defs.len())  {
                    //Here we are doing all other columns
                    let column_datatype = &column_defs[i];

                    let x = match column_datatype{
                        DataType::BigInt(..) => {
                            let value = i64::from_le_bytes(lab[byte_count..byte_count + 8].try_into().unwrap());
                            byte_count += 8;
                            column.push(DataType::BigInt(value));
                        },
                        DataType::Int(..) => {
                            let value = i32::from_le_bytes(lab[byte_count..byte_count+4].try_into().unwrap());
                            byte_count += 4;
                            column.push(DataType::Int(value));
                        }
                        DataType::SmallInt(..) => {
                            let value = i16::from_le_bytes(lab[byte_count..byte_count+2].try_into().unwrap());
                            byte_count += 2;
                            column.push(DataType::SmallInt(value));
                        }
                        DataType::TinyInt(..) => {
                            let value = i8::from_le_bytes(lab[byte_count..byte_count+1].try_into().unwrap());
                            byte_count += 1;
                            column.push(DataType::TinyInt(value));
                        }
                        DataType::Decimal(..) => {
                            let value = f32::from_le_bytes(lab[byte_count..byte_count+4].try_into().unwrap());
                            byte_count += 4;
                            column.push(DataType::Decimal(value));
                        }
                        DataType::Float(..) => {
                            let value = f64::from_le_bytes(lab[byte_count..byte_count+8].try_into().unwrap());
                            byte_count += 8;
                            column.push(DataType::Float(value));
                        }
                        DataType::VarChar(..) => {
                            let length = u8::from_le_bytes(lab[byte_count..byte_count+1].try_into().unwrap());
                            let value = u8::from_le_bytes(lab[byte_count..byte_count+length as usize].try_into().unwrap());
                            byte_count += length as usize;
                            column.push(DataType::VarChar(length, value.to_string()));
                        }
                        DataType::Bool(..) => {
                            let value = u8::from_le_bytes(lab[byte_count..byte_count+1].try_into().unwrap());
                            byte_count += 1;
                            column.push(DataType::Bool(value != 0));
                        }
                        //Null seems to be stores as a byte value of 00000000
                        DataType::Null => {
                            let value = u8::from_le_bytes(lab[byte_count..byte_count+1].try_into().unwrap());
                            byte_count += 1;
                            column.push(DataType::Null);
                        }
                        _ => panic!("unsupported datatype"),
                    };
                }
            }
            counter += 1;
            table.insert_row(column);
           
        }
    }



    Table::default()
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
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use crate::file::moireader::load_moi_file;
    use crate::file::mtdreader::read_mtd_file;
    /*    fn main() -> std::io::Result<()> {
                let file = File::create("data.bin")?;
                let mut writer = BufWriter::new(file);

                let entries: Vec<Vec<u8>> = vec![
                    vec![0, 1, 2, 3],
                    vec![4, 5, 6, 7],
                    vec![255, 10, 253], // may contain newline byte
                ];

                for entry in entries {
                    let len = entry.len() as u32;
                    writer.write_all(&len.to_le_bytes())?; // 4-byte length
                    writer.write_all(&entry)?;             // raw binary data
                }

                writer.flush()?;
                Ok(())
            }*/

    #[test]
    fn test_read_mtd_file() {
        let foo = read_mtd_file("C:\\MiaSql\\system\\database.mtd");
        load_moi_file(&foo);
        println!("{:?}", foo);
    }

    #[test]
    fn test_writing_file() {
        let file_result = File::create("C:\\MiaSql\\system\\database.moi");

         match file_result {
            Ok(file_item) => {
                let mut writer = BufWriter::new(file_item);
                let counter: u64 = 1;
                let varchar_length:u8 = 6;

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
        load_moi_file(&foo);
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

    fn fill_varchar(varchar: String, len:u16) -> String {
        if len == 0 || varchar.len() == 0{
            return String::new();
        }

        let varchar_length: u16 = varchar.len() as u16;
        if varchar_length  < len {
            let mut number_of_spaces = len - varchar_length;

            let mut result: String = varchar.clone();
            for i in 0..number_of_spaces  {
                result = result + " ";
            }
            return result;
        }else if varchar_length > len {

            let mut result = String::new();
            let length_as_usize = len as usize;

            for i in 0..=length_as_usize {
                if i == 0{
                    result = result + &varchar[..i];
                }else{
                    result = result + &varchar[i-1..i];
                }
            }
            return result;

        }
        varchar
    }

}