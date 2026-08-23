use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::database::datatype::DataType;
use crate::database::memstruct::MemoryStructure;

// This holds two vectors with the ids of the rows
// The pos vector stores the ids of true values and the neg vector does the same with false values
#[derive(Debug)]
pub struct BooleanStructure {
    pos: Vec<i64>,
    neg: Vec<i64>,
}

impl MemoryStructure<bool, i64> for BooleanStructure {
    fn create(&self) -> Self {
        Self {pos: Vec::new(), neg: Vec::new() }
    }

    fn insert(&mut self, value: bool, id: i64) {
        if value {
            self.pos.push(id);
        }else{
            self.neg.push(id);
        }
    }

    fn retrieve_values(&self, value: i64) -> bool {
       panic!("This function of the trait does not make any sense in this context- so don't use it");
    }

    //this here needs the boolean value to be a number
    // 1 = true else false
    fn retrieve_keys(&self, value: i64) -> Vec<i64>{
        if value == 1 {
            self.pos.to_vec()
        }else{
            self.neg.to_vec()
        }
    }

    fn delete(&mut self, id: i64) {
        rayon::scope(|s| {
            s.spawn(|_|
                if let Some(index) = self.pos.iter().position(|value| value == &id) {
                self.pos.swap_remove(index);
            });
            s.spawn(|s| {
                if let Some(index) = self.neg.iter().position(|value| value == &id) {
                    self.neg.swap_remove(index);
                }
            })
        } );
    }
}


#[cfg(test)]
mod tests {
    use crate::database::boolstructure::BooleanStructure;
    use crate::database::memstruct::MemoryStructure;

    #[test]
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
    }
}
