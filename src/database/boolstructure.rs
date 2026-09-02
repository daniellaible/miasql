//use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::table::Row;

// This holds two vectors with the ids of the rows
// The pos vector stores the ids of true values and the neg vector does the same with false values
#[derive(Debug, Clone, Default)]
pub struct BooleanStructure {
    pos: Vec<RowId>,
    neg: Vec<RowId>,
}

/// This implementation of [MemoryStructure] inserts the ID of the inserted row in one of two vectors.
/// The goal is to have two vectors that contain all the ids of a certain table. If the value is
/// true the [RowId] is saved in the vector called pos else in the vector false. 
/// Therefore, you can have a quick reference to all ids of rows containing a certain column with 
/// a boolean value
impl MemoryStructure for BooleanStructure {
    
    /// This function inserts a [RowId] in one of two vectors depending on the value.
    /// Please make sure the value is of type IndexValue::Bool any other index vaue will be 
    /// disregarded. 
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Bool(true) => self.pos.push(id),
            IndexValue::Bool(false) => self.neg.push(id),
            _ => {
            }
        }
    }

    /// This function returns a list of [RowId]s depending on the provided key.
    /// The key needs to be of IndexValue::Bool otherwise an empyt vector is returned.
    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        match key {
            IndexValue::Bool(true) => self.pos.clone(),
            IndexValue::Bool(false) => self.neg.clone(),
            _ => vec![],
        }
    }

    /// Do not use this [MemoryStructure] function in this context. It would be possible to scan
    /// both vectors (pos/neg) and return a row of the database, however it is painfully slow.
    /// If you need to access a column value by id, it is faster to access the hashmap that contains
    /// the row instead of scanning both vectors completely. Therefore, to prevent misuse of this function,
    /// the engine panics.
    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
        panic!("This function should not be used in this context");
    }

    /// This function deletes an entry by the [RowId]
    fn delete(&mut self, id: RowId) {
        if let Some(i) = self.pos.iter().position(|v| *v == id) {
            self.pos.remove(i);
        }
        if let Some(i) = self.neg.iter().position(|v| *v == id) {
            self.neg.remove(i);
        }
    }

    /// Returns the type of this [MemoryStructure] implementation. 
    /// In this case 'bool' 
    fn kind(&self) -> &'static str { "bool" }

    /// This is needed to implement the clone trait
    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }
}


#[cfg(test)]
mod tests {
    use crate::database::boolstructure::BooleanStructure;
    use crate::database::memstruct::{IndexValue, MemoryStructure};

    #[test]
    fn basic_insert_retrieve_test() {
        let mut boolean_structure = BooleanStructure {
            pos: vec![],
            neg: vec![],
        };
        insert_helper(&mut boolean_structure);

        let all_trues = boolean_structure.retrieve_range(&IndexValue::Bool(true));
        let all_false = boolean_structure.retrieve_range(&IndexValue::Bool(false));
        assert_eq!(all_trues.len(), 3);
        assert_eq!(all_false.len(), 4);

        boolean_structure.delete(4);
        let all_false = boolean_structure.retrieve_range(&IndexValue::Bool(false));
        assert_eq!(all_false.len(), 3);
    }

    #[test]
    fn kind_test(){
        let mut boolean_structure = BooleanStructure {
            pos: vec![],
            neg: vec![],
        };
        insert_helper(&mut boolean_structure);
        assert_eq!(boolean_structure.kind(), "bool");
    }

    #[test]
    fn delete_test(){
        let mut boolean_structure = BooleanStructure {
            pos: vec![],
            neg: vec![],
        };
        insert_helper(&mut boolean_structure);
        boolean_structure.delete(1);
        boolean_structure.delete(2);
        let all_trues = boolean_structure.retrieve_range(&IndexValue::Bool(true));
        assert_eq!(all_trues.len(), 1);
    }

    #[test]
    fn delete_non_exist_test() {
        let mut boolean_structure = BooleanStructure {
            pos: vec![],
            neg: vec![],
        };
        insert_helper(&mut boolean_structure);
        boolean_structure.delete(10);
        let all_trues = boolean_structure.retrieve_range(&IndexValue::Bool(true));
        let all_false = boolean_structure.retrieve_range(&IndexValue::Bool(false));
        assert_eq!(all_trues.len(), 3);
        assert_eq!(all_false.len(), 4);
    }


    fn insert_helper(bool_struct: &mut BooleanStructure){
        bool_struct.insert(IndexValue::Bool(true), 1);
        bool_struct.insert(IndexValue::Bool(true), 2);
        bool_struct.insert(IndexValue::Bool(true), 3);
        bool_struct.insert(IndexValue::Bool(false), 4);
        bool_struct.insert(IndexValue::Bool(false), 5);
        bool_struct.insert(IndexValue::Bool(false), 6);
        bool_struct.insert(IndexValue::Bool(false), 7);
    }
}
