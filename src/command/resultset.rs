use crate::database::datatype::DataType;
use crate::database::table::Row;

#[derive(Debug)]
pub struct ResultSet{
    pub header: Vec<DataType>,
    pub rows: Vec<Row>,
    pub duration: String
}