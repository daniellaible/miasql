use crate::database::tabel::Row;

#[derive(Debug)]
pub struct ResultSet{
    pub header: Vec<String>,
    pub rows: Vec<Row>,
    pub duration: u128
}

impl ResultSet{
    pub fn create_with_rows( header: Vec<String>, rows: Vec<Row>) -> Self {
        let resultset = ResultSet{
            header,
            rows,
            duration: 0,
        };
        resultset
    }

    pub fn create_with_header(header:Vec<String>) -> Self{
        let resultset = ResultSet{
            header,
            rows: Vec::new(),
            duration:0
        };
        resultset
    }

    pub fn create() -> Self{
        let resultset = ResultSet{
            header: Vec::new(),
            rows: Vec::new(),
            duration: 0
        };
        resultset
    }
}