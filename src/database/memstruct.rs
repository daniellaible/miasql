use std::fmt::Debug;
use crate::database::table::Row;

pub type RowId = i64;

#[derive(Debug, Clone, PartialEq)]
pub enum IndexValue {
    Bool(bool),
    Text(String),
    Row(Row)
}

pub trait MemoryStructure: Debug + Send + Sync {
    fn insert(&mut self, value: IndexValue, id: RowId);
    fn retrieve_by_other(&self, key: &IndexValue) -> Vec<RowId>;
    fn retrieve_by_i64(&self, id: RowId) -> Vec<IndexValue>;
    fn delete(&mut self, id: RowId);

    fn clone_box(&self) -> Box<dyn MemoryStructure>;
}

impl Clone for Box<dyn MemoryStructure> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}








