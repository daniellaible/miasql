use std::fmt;

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

impl fmt::Display for Constraint{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{:?}", self)
    }
}