#[derive(Clone, Debug, PartialEq)]
pub enum Constraint {
    NOT_NULL,
    UNIQUE,
    PRIMARY_KEY,
    FOREIGN_KEY,
    CHECK,
    DEFAULT,
    UNDEFINED
}