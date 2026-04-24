#[derive(Clone, Debug)]
#[derive(PartialEq)]
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