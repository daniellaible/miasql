mod bptree;
#[path="table/table.rs"]
mod table;

mod command {
    pub mod sqloperator;
    pub mod whereclause;
    pub mod select;
    pub mod insert;
    pub mod functions;
}

fn main() {
    println!("Hello, world!");
}
