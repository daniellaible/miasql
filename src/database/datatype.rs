//! These are the datatypes we will support in the beginning

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    BigInt (i64),
    Int (i32),
    SmallInt (i16),
    TinyInt (i8),
    Decimal (f32),
    Float (f64),
    VarChar (u8, String),
    Bool (bool),
    Date (i64),
    Time (i64),
    DateTime (i64),
    Null,
    Undefined,
}



/*pub fn to_datatype(value: &str) -> DataType {
    match value {
        "BIGINT" => DataType::BigInt { },
        "INT" => DataType::Int { x: 0 },
        "SMALLINT" => DataType::SmallInt { x: 0 },
        "TINYINT" => DataType::TinyInt { x: 0 },
        "DECIMAL" => DataType::Decimal { x: 0.0 },
        "FLOAT" => DataType::Float { x: 0.0 },
        "VARCHAR" => DataType::VarChar { x: String::default(), y: 0 },
        "BOOL" => DataType::Bool {x: false},
        "DATE" => DataType::Date { x: 0 },
        "TIME" => DataType::Time { x: 0 },
        "DATETIME" => DataType::DateTime { x: 0 },
        "NULL" => DataType::Null,
        _ => DataType::Undefined,
    }
}
*/

