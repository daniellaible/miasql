use std::collections::HashMap;
use crate::database::memstruct::MemoryStructure;
use crate::database::table::Row;

#[derive(Debug)]
pub struct HashmapStructure {
    data: HashMap<i64, Row>,
}

impl MemoryStructure<Row, i64> for HashmapStructure {
    fn create(&self) -> Self {
        Self{ data: HashMap::new()}
    }

    fn insert(&mut self, value:Row, id:i64) {
       self.data.insert(id, value);
    }

    fn retrieve_values(&self, id:i64) -> Row {
        if self.data.contains_key(&id){
            match self.data.get(&id){
                None => {
                    let row: Row = Row { data: Vec::new() };
                    row
                }
                Some(row) => {
                    row.clone()
                }
            }
        }else{
            let row: Row = Row { data: Vec::new() };
            row
        }
    }

    fn retrieve_keys(&self, value: i64) -> Vec<i64> {
        let iter_keys = self.data.keys();
        let mut result = Vec::with_capacity(self.data.len());
        for i in iter_keys {
            result.push(i.clone());
        }
        result
    }

    fn delete(&mut self, id:i64) {
        self.data.remove(&id);
    }
}