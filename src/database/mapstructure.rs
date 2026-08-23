use std::collections::HashMap;
use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::table::Row;

#[derive(Debug, Clone)]
pub struct HashmapStructure {
    data: HashMap<RowId, Row>,
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

    fn retrieve_by_i64(&self, id: RowId) -> Vec<IndexValue>
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


    /*    fn retrieve_keys(&self, value: &i64) -> Vec<i64> {
            let iter_keys = self.data.keys();
            let mut result = Vec::with_capacity(self.data.len());
            for i in iter_keys {
                result.push(i.clone());
            }
            result
        }*/

   

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }
}