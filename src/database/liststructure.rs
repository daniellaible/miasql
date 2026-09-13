use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::table::Row;
use rayon::prelude::*;
use std::cmp::Ordering;

/// This structure is used to store decimals of typ t in memory.
/// Type T must be of type f32 or f64.
/// The data vector is sorted after every insert. This way a lookup can be done with binary search.
#[derive(Debug, Clone, Default)]
pub struct ListStructure<T> {
    data: Vec<(T, Vec<RowId>)>,
}

/// This type of [MemoryStructure] is dedicated to float and decimal numbers.
/// The structure consists of a vector of tupels of type (f64, Vec<[RowId]>).
/// So a decimal value is stored in order along with a vector that saved the [RowId]s of the
/// corresponding [Row]
impl MemoryStructure for ListStructure<f64> {
    // Use this function to insert a f64 number to this data structure. NaN is filtered out.
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Float(f) => {
                if f.is_nan() {
                    return;
                }
                if !check_value_in_list_f64::<f64>(self.data.clone(), f) {
                    let mut rowIds = Vec::new();
                    rowIds.push(id);
                    self.data.push((f, rowIds));
                    self.data.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
                } else {
                    self.data = find_elem_and_add_f64(self.data.clone(), f, id);
                }
            }
            _ => {}
        }
    }

    /// This interfacte is mainly for the use of enums and boolean datatypes
    /// Don't use this interface with decimals
    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        panic!("This function should not be used in this context");
    }

    /// Do no use this interface with decimals. Select the [Row] directly
    /// in the [HashmapStructure]
    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
        panic!("This function should not be used in this context");
    }

    /// This deletes the reference to a row, if the containing vector is empty the
    /// whole tupel is deleted
    fn delete(&mut self, id: RowId, _value:Option<IndexValue>) {
        self.data.par_iter_mut().for_each(|tupel| {
            tupel.1.retain(|&x| x != id);
        });
        self.data.retain(|(_, ids)| !ids.is_empty());
    }

    /// This is needed to implement the clone trait
    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    /// Returns the type of this [MemoryStructure] implementation.
    /// In this case 'list'
    fn kind(&self) -> &'static str { "list" }
}

fn find_elem_and_add_f64(mut list: Vec<(f64, Vec<RowId>)>, number: f64, id: RowId)
                         -> Vec<(f64, Vec<RowId>)>
{
    let index = list
        .binary_search_by(|(value, _)| value.partial_cmp(&number).unwrap_or(Ordering::Equal))
        .expect("Element not found although it should be here");

    list[index].1.push(id);
    list
}

fn check_value_in_list_f64<T: 'static>(list: Vec<(f64, Vec<RowId>)>, number: f64) -> bool
{
    let is_in_list =
        list.binary_search_by(|(value, _)| value.partial_cmp(&number).unwrap_or(Ordering::Equal));
    match is_in_list {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// This type of [MemoryStructure] is dedicated to float and decimal numbers.
/// The structure consists of a vector of tupels of type (f32, Vec<[RowId]>).
/// So a decimal value is stored in order along with a vector that saved the [RowId]s of the
/// corresponding [Row]
impl MemoryStructure for ListStructure<f32> {
    // Use this function to insert a f32 number to this data structure. NaN is filtered out.
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Decimal(f) => {
                if f.is_nan() {
                    return;
                }
                if !check_value_in_list_f32(self.data.clone(), f) {
                    let mut rowIds = Vec::new();
                    rowIds.push(id);
                    self.data.push((f, rowIds));
                    self.data.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
                } else {
                    self.data = find_elem_and_add_f32(self.data.clone(), f, id);
                }
            }
            _ => {}
        }
    }

    /// This interfacte is mainly for the use of enums and boolean datatypes
    /// Don't use this interface with decimals
    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        panic!("This function should not be used in this context");
    }

    /// Do no use this interface with decimals. Select the [Row] directly
    /// in the [HashmapStructure]
    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
        panic!("This function should not be used in this context");
    }

    /// This deletes the reference to a row, if the containing vector is empty the
    /// whole tupel is deleted
    fn delete(&mut self, id: RowId, _value:Option<IndexValue>) {
        self.data.par_iter_mut().for_each(|tupel| {
            tupel.1.retain(|&x| x != id);
        });
        self.data.retain(|(_, ids)| !ids.is_empty());
    }

    /// This is needed to implement the clone trait
    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    /// Returns the type of this [MemoryStructure] implementation.
    /// In this case 'list'
    fn kind(&self) -> &'static str { "list" }
}

fn check_value_in_list_f32(list: Vec<(f32, Vec<RowId>)>, number: f32) -> bool {
    let is_in_list =
        list.binary_search_by(|(value, _)| value.partial_cmp(&number).unwrap_or(Ordering::Equal));
    match is_in_list {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn find_elem_and_add_f32(mut list: Vec<(f32, Vec<RowId>)>, number: f32, id: RowId)
                         -> Vec<(f32, Vec<RowId>)>
{
    let index = list
        .binary_search_by(|(value, _)| value.partial_cmp(&number).unwrap_or(Ordering::Equal))
        .expect("Element not found although it should be here");

    list[index].1.push(id);
    list
}

#[cfg(test)]
mod tests {
    use crate::database::liststructure::ListStructure;
    use crate::database::memstruct::{IndexValue, MemoryStructure};

    #[test]
    fn insert_new_id(){
        let mut list_structure:ListStructure<f64> = ListStructure {
            data: vec![]
        };
        let test_number1 = IndexValue::Float(3.14159);
        let test_number2 = IndexValue::Float(2.718);
        let test_number3 = IndexValue::Float(3.01);
        let test_number4 = IndexValue::Float(3.01);

       list_structure.insert(test_number1, 1);
       list_structure.insert(test_number2, 2);
       list_structure.insert(test_number3, 3);
       list_structure.insert(test_number4, 4);
        println!("{:?}", list_structure);
    }

    //TODO do more tests

}


