use crate::database::datatype::DataType;
use crate::database::table::Row;

#[derive(Debug)]
pub struct ResultSet{
    pub header: Vec<String>,
    pub rows: Vec<Row>,
    pub duration: String
}

impl ResultSet{
    pub fn create_with_rows( header: Vec<String>, rows: Vec<Row>) -> Self {
        let resultset = ResultSet{
            header,
            rows,
            duration: "0.0".to_string(),
        };
        resultset
    }

    pub fn create_with_header(header:Vec<String>) -> Self{
        let resultset = ResultSet{
            header,
            rows: Vec::new(),
            duration:"0.0".to_string()
        };
        resultset
    }

    pub fn create() -> Self{
        let resultset = ResultSet{
            header: Vec::new(),
            rows: Vec::new(),
            duration:"0.0".to_string()
        };
        resultset
    }
}