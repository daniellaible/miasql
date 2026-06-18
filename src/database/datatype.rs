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
    Date (u64),
    Time (u64),
    DateTime (u64),
    Null,
    Undefined,
}

