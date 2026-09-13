use rayon::iter::ParallelIterator;
use std::cmp::Ordering;
use rayon::iter::IntoParallelRefMutIterator;
use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::table::Row;

/// This structure is used to save varchar data in an ordered list.
/// The list contains tupels which are structured like this ([IndexValue::Text], Vec<[RowId]>)
#[derive(Debug, Clone, Default)]
pub struct VarCharStructure {
    data: Vec<(String, Vec<RowId>)>
}

/// This type of [MemoryStructure] is dedicated to [DataType::VarChar].
/// The structure consists of a vector of tupels of type ([IndexValue::Text], [Vec]<[RowId]>).
/// So a [DataType::VarChar] is stored in order along with a vector that saved the [RowId]s of the
/// corresponding [Row]. Internally, the given value is of type [IndexValue::Text] which in itself
/// is a wrapper for a String which is in this case a [DataType::VarChar].
/// TLDR value has to be of type [IndexValue::Text]
impl MemoryStructure for VarCharStructure {
    /// This function is used to insert a [IndexValue::Text] into this kind of [MemoryStructure]
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Text(text) => {
                if text.is_empty(){
                    return;
                }
                if !text_is_stored(&self.data, text.clone()) {
                    let mut row_vec = Vec::new();
                    row_vec.push(id);
                    self.data.push((text, row_vec));
                    self.data.sort_by(|x,y| x.0.partial_cmp(&y.0).unwrap())
                }else{
                    self.data = find_elem_and_add(self.data.clone(), text, id);
                }
            }
            _ => {}
        }
    }

    /// Rule #1: Don't use it <br>
    /// Rule #2: Don't think about using    it <br>
    /// goto Rule #1
    fn retrieve_range(&self, _key: &IndexValue) -> Vec<RowId> {
        panic!("We don't do this over here - wrong command for the wrong memorystructure")
    }

    /// Rule #1: Don't do it <br>
    /// Rule #2: Don't think about doing it <br>
    /// goto Rule #1
    fn retrieve_by_index(&self, _id: RowId) -> Option<Row> {
        panic!("Stupid thing we do not do - wrong command for the wrong memorystructure")
    }

    fn delete(&mut self, id: RowId, _value:Option<IndexValue>) {
        self.data.par_iter_mut().for_each(|tupel| {
            tupel.1.retain(|&x| x != id);
        });
        self.data.retain(|(_, ids)| !ids.is_empty());
    }

    /// Needed for the clone trait
    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    /// Returns the kind of [MemoryStructure] this is - here it is 'varchar'
    fn kind(&self) -> &'static str { "varchar" }
}

fn find_elem_and_add(mut data: Vec<(String, Vec<RowId>)>, text: String, id: RowId) -> Vec<(String, Vec<RowId>)> {
    let index = data
        .binary_search_by(|(value, _)| value.partial_cmp(&text).unwrap_or(Ordering::Equal))
        .expect("Element not found although it should be here");
    data[index].1.push(id);
    data
}

fn text_is_stored(data: &Vec<(String, Vec<RowId>)>, text: String) -> bool {
    let res = data.binary_search_by(|(value, _) | value.partial_cmp(&text).unwrap_or(Ordering::Equal));
    match res {
        Ok(_) => {true}
        Err(_) => {false}
    }
}

#[cfg(test)]
mod tests {


}