#[derive(Clone, Debug, PartialEq)]
pub enum Constraint {
    NotNull,
    Unique,
    PrimaryKey,
    ForeignKey,
    Check,
    Default,
    Undefined
}