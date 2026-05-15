
#[derive(Clone, Debug, PartialEq)]
pub enum Constraint {
    NOT_NULL {x: bool},
    UNIQUE {x: bool},
    PRIMARY_KEY {x: bool},
    FOREIGN_KEY {x: bool},
    CHECK {x: bool},
    DEFAULT {x: bool},
}