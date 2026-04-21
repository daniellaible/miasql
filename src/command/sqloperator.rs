#[derive(Clone, Debug)]
pub enum Operator {
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
    MODULO,
    EQUAL,
    GREATER,
    LESSER,
    GREATEROREQ,
    LESSEROREQ,
    NOTEQUAL,
    UNDEFINED
}