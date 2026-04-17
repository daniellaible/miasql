// These are the datatypes we will support in the beginning
pub enum DataType {
    BigInt{x: i64},
    Int {x: i32},
    SmallInt {x: i16},
    TinyInt {x : i8},
    Decimal {x: f32},
    Float {x: f64},
    VarChar {x: String, y: u8},
    Bool {x: bool},
    Date {x: i64},
    Time {x: i64},
    DateTime{x: i64}
}