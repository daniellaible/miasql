use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::table::Row;
use std::cmp::Ordering;
use std::f64::NAN;

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

    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        todo!()
    }

    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
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

fn find_elem_and_add_f32(p0: Vec<(IndexValue, Vec<RowId>)>, p1: f32) {
    todo!()
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


fn check_value_in_list_f32<T>(list: Vec<(f32, Vec<RowId>)>, number: f32) -> bool
where
    f32: From<T>,
{
    let is_in_list =
        list.binary_search_by(|(value, _)| value.partial_cmp(&number).unwrap_or(Ordering::Equal));
    match is_in_list {
        Ok(_) => true,
        Err(_) => false,
    }
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

#[cfg(test)]
mod tests {
    use crate::database::liststructure::ListStructure;
    use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};

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

}


