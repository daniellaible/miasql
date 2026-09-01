use crate::database::boolstructure::BooleanStructure;
use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};

#[derive(Debug, Clone, Default)]
pub struct EnumStructure {
    pub data: Vec<Vec<RowId>>
}

impl MemoryStructure for EnumStructure {
    fn insert(&mut self, value: IndexValue, id: RowId) {
        todo!()
    }

    fn retrieve_by_other(&self, key: &IndexValue) -> Vec<RowId> {
        todo!()
    }

    fn retrieve_by_u64(&self, id: RowId) -> Vec<IndexValue> {
        todo!()
    }

    fn delete(&mut self, id: RowId) {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        todo!()
    }

    fn kind(&self) -> &'static str {
        todo!()
    }
}