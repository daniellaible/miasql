//! These are the datatypes we will support in the beginning

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    BigInt { x: i64 },
    Int { x: i32 },
    SmallInt { x: i16 },
    TinyInt { x: i8 },
    Decimal { x: f32 },
    Float { x: f64 },
    VarChar { x: String, y: usize },
    Bool { x: bool },
    Date { x: i64 },
    Time { x: i64 },
    DateTime { x: i64 },
    Undefined,
}

impl DataType {
    pub fn as_i64(&self) -> Option<i64> {
        match self{
            DataType::BigInt { x } => Some(*x),
            _ => None
        }
    }
}

pub fn to_datatype(value: &str) -> DataType {
    match value {
        "BIGINT" => DataType::BigInt { x: 0 },
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
        _ => DataType::Undefined,
    }
}


