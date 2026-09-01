use std::collections::HashMap;
use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::table::Row;

#[derive(Debug, Clone)]
pub struct HashmapStructure {
    pub data: HashMap<RowId, Row>,
}

impl MemoryStructure for HashmapStructure {
       fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Row(row) => {
                self.data.insert(id, row);
            }
            _ => {
            }
        }
    }

    fn retrieve_by_other(&self, key: &IndexValue) -> Vec<RowId> {
        panic!("This should not be called in this context");
    }

    fn retrieve_by_u64(&self, id: RowId) -> Vec<IndexValue>
    {
        let row_option = self.data.get(&id);
        match  row_option {
            Some(row) => vec![IndexValue::Row(row.clone())],
            None => vec![],
        }
    }

    fn delete(&mut self, id: RowId) {
        todo!()
    }
    

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }


    fn kind(&self) -> &'static str { "hash" }
}