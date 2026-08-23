use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::database::datatype::DataType;
use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::table::Row;

// This holds two vectors with the ids of the rows
// The pos vector stores the ids of true values and the neg vector does the same with false values
#[derive(Debug, Clone, Default)]
pub struct BooleanStructure {
    pos: Vec<RowId>,
    neg: Vec<RowId>,
}

impl MemoryStructure for BooleanStructure {


    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Bool(true) => self.pos.push(id),
            IndexValue::Bool(false) => self.neg.push(id),
            _ => {
            }
        }
    }

    fn retrieve_by_other(&self, key: &IndexValue) -> Vec<RowId> {
        match key {
            IndexValue::Bool(true) => self.pos.clone(),
            IndexValue::Bool(false) => self.neg.clone(),
            _ => vec![],
        }
    }

    //this here needs the boolean value to be a number
    // 1 = true else false
    fn retrieve_by_i64(&self, id: RowId) -> Vec<IndexValue> {
        let mut out = Vec::new();
        if self.pos.contains(&id) {
            out.push(IndexValue::Bool(true));
        }
        if self.neg.contains(&id) {
            out.push(IndexValue::Bool(false));
        }
        out
    }

    fn delete(&mut self, id: RowId) {
        if let Some(i) = self.pos.iter().position(|v| *v == id) {
            self.pos.remove(i);
        }
        if let Some(i) = self.neg.iter().position(|v| *v == id) {
            self.neg.remove(i);
        }
    }

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }
}


#[cfg(test)]
mod tests {
    use crate::database::boolstructure::BooleanStructure;
    use crate::database::memstruct::MemoryStructure;

/*    #[test]
    fn basic_insert_retrieve() {
        let mut boolean_structure = BooleanStructure {
            pos: vec![],
            neg: vec![],
        };
        boolean_structure.insert(true, 1);
        boolean_structure.insert(true, 2);
        boolean_structure.insert(true, 3);
        boolean_structure.insert(false, 4);
        boolean_structure.insert(false, 5);
        boolean_structure.insert(false, 6);
        boolean_structure.insert(false, 7);

        let all_trues = boolean_structure.retrieve_keys(1);
        let all_false = boolean_structure.retrieve_keys(0);
        assert_eq!(all_trues.len(), 3);
        assert_eq!(all_false.len(), 4);

        boolean_structure.delete(4);
        let all_false = boolean_structure.retrieve_keys(0);
        assert_eq!(all_false.len(), 3);
    }*/
}
