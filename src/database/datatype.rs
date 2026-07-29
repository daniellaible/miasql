//! These are the datatypes we will support in the beginning

use std::fmt;

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
    Date (u64),
    Time (u64),
    DateTime (u64),
    Null,
    Undefined,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            DataType::BigInt(x) => {
                write!(f, "{:?}", x)
            }
            DataType::Int(x) => {
                write!(f, "{:?}", x)
            }
            DataType::SmallInt(x) => {
                write!(f, "{:?}", x)
            }
            DataType::TinyInt(x) => {
                write!(f, "{:?}", x)
            }
            DataType::Decimal(x) => {
                write!(f, "{:?}", x)
            }
            DataType::Float(x) => {
                write!(f, "{:?}", x)
            }
            DataType::VarChar(.., s) => {
                write!(f, "{:?}", s)
            }
            DataType::Bool(b) => {write!(f, "{:?}", b)}
            DataType::Date(d) => {
                write!(f, "{:?}", d)
            }
            DataType::Time(d) => {
                write!(f, "{:?}", d)
            }
            DataType::DateTime(d) => {
                write!(f, "{:?}", d)
            }
            DataType::Null => {
                write!(f, "NULL" )
            }
            DataType::Undefined => {
                write!(f, "UNDEF" )
            }
        }
    }
}

