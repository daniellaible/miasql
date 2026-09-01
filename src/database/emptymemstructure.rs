use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};

#[derive(Debug, Clone, Default)]
pub struct EmptyMemStructure {

}

impl MemoryStructure for EmptyMemStructure {
    fn insert(&mut self, value: IndexValue, id: RowId) {
        panic!("This function should not be called in this context");
    }

    fn retrieve_by_other(&self, key: &IndexValue) -> Vec<RowId> {
        panic!("This function should not be called in this context");
    }

    fn retrieve_by_u64(&self, id: RowId) -> Vec<IndexValue> {
        panic!("This function should not be called in this context");
    }

    fn delete(&mut self, id: RowId) {
        panic!("This function should not be called in this context");
    }

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    fn kind(&self) -> &'static str { "empty" }
}