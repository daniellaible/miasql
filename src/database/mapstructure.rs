use std::collections::HashMap;
use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::table::Row;

/// This struct saves a [Row] in a hashmap. The key to the row is the [RowId] - just a fancy name
/// for an u64.
#[derive(Debug, Clone, Default)]
pub struct HashmapStructure {
    pub data: HashMap<RowId, Row>,
}

/// This implementation of [MemoryStructure] uses a hashmap to access data quickly if e.g. the
/// id of a row is known. This kind of [MemoryStructure] should be used to store the complete
/// row, do not use [BPlusTree] to store a row. Use [BPlusTree] to index numbers so ranges
/// can be found quickly.
impl MemoryStructure for HashmapStructure {

    /// This function inserts a row into a Hashmap by the [RowId]. Make sure the underlying [IndexValue]
    /// is of type IndexValue::Row - any other type is ignored.
    /// [RowId] is of type u64, so no negative values are possible.
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Row(row) => {
                self.data.insert(id, row);
            }
            _ => { }
        }
    }

    /// If you call this function, the program panics!
    /// This implementation of [MemoryStructure] provides a specific selection of data, it does not
    /// provide the selection of a range of data - therefore, this functionality should not be used.
    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        panic!("This should not be called in this context");
    }

    /// Use this method to retrieve row data from this[MemoryStructure].
    /// As a result, it returns the [Row] of the given tabel
    fn retrieve_by_index(&self, id: RowId) -> Option<Row>
    {
        let row_option = self.data.get(&id);
        match  row_option {
            Some(row) => Some(row.clone()),
            None => None,
        }
    }

    /// Use this method to delete a [Row] from the tabel
    fn delete(&mut self, id: RowId)
    {
        self.data.remove(&id);
    }

    /// This is needed to implement the clone trait 
    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }
    
    /// Returns the type of this [MemoryStructure] implementation. 
    /// In this case 'hash' 
    fn kind(&self) -> &'static str { "hash" }
}

#[cfg(test)]
mod tests {
    use crate::database::datatype::DataType::{BigInt, Int};
    use crate::database::mapstructure::HashmapStructure;
    use crate::database::memstruct::{IndexValue, MemoryStructure};
    use crate::database::table::Row;


    #[test]
    fn basic_insert_retrieve_test() {
        let mut map_structure = HashmapStructure::default();
        insert_helper(&mut map_structure);
        assert_eq!(map_structure.data.len(), 7);

        let row = map_structure.retrieve_by_index(6).unwrap();

        assert_eq!(row.data[0], BigInt(6));
        assert_eq!(row.data[1], Int(48));
        assert_eq!(row.data[2], Int(49));
    }

    #[test]
    fn basic_insert_delete_test() {
        let mut map_structure = HashmapStructure::default();
        insert_helper(&mut map_structure);
        map_structure.delete(1);
        assert_eq!(map_structure.retrieve_by_index(1), None);
    }

    fn insert_helper(map_struct: &mut HashmapStructure){
        let row1:Row = Row{
            data: vec![BigInt(0), Int(42), Int(43)]
        };
        map_struct.insert(IndexValue::Row(row1), 0);

        let row2:Row = Row{
            data: vec![BigInt(1), Int(443), Int(44)]
        };
        map_struct.insert(IndexValue::Row(row2), 1);

        let row3:Row = Row{
            data: vec![BigInt(2), Int(44), Int(45)]
        };
        map_struct.insert(IndexValue::Row(row3), 2);

        let row4:Row = Row{
            data: vec![BigInt(3), Int(45), Int(46)]
        };
        map_struct.insert(IndexValue::Row(row4), 3);

        let row5:Row = Row{
            data: vec![BigInt(4), Int(46), Int(47)]
        };
        map_struct.insert(IndexValue::Row(row5), 4);

        let row6:Row = Row{
            data: vec![BigInt(5), Int(47), Int(48)]
        };
        map_struct.insert(IndexValue::Row(row6), 5);

        let row7:Row = Row{
            data: vec![BigInt(6), Int(48), Int(49)]
        };
        map_struct.insert(IndexValue::Row(row7), 6);
    }


}