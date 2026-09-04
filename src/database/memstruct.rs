use std::fmt::Debug;
use crate::database::table::Row;

/// A RowId is just a fancy way of saying that the ids have to be 0 or greater and that
/// they are of 64-bit wide.
pub type RowId = u64;

/// When accessing different MemoryStructures like a tree [BPlusTree] or a hashmap [MapStructure]
/// to manipulate data in memory, different datatypes are often used, although the intention
/// of the trait is the same. This enum is used to keep the interface small.
#[derive(Debug, Clone, PartialEq)]
pub enum IndexValue {
    Bool(bool),
    Text(String),
    Row(Row),
    BigInt(i64),
    Int(i32),
    SmallInt(i16),
    TinyInt(i8)

}

/// This trait describes the interface of several different structs which are designed to
/// handle data in memory.
/// These structs are [BooleanStructure], [BPlusTree], [EmptyMemStructure], [EnumStructure] and [MapStructure]
pub trait MemoryStructure: Debug + Send + Sync {

    /// Use this interface to add a new value to a memory structure
    fn insert(&mut self, value: IndexValue, id: RowId);
    /// Some MemoryStructures return a range instead of a specific [Row]
    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId>;
    /// If you need a specifx [Row] and you know the [RowId]
    fn retrieve_by_index(&self, id: RowId) -> Option<Row>;
    /// You might want to delete an entry from the [MemoryStructure]
    fn delete(&mut self, id: RowId);
    /// This is used to implement the clone trait
    fn clone_box(&self) -> Box<dyn MemoryStructure>;
    /// If you need to know what kind of [MemoryStructure] you are dealing with
    fn kind(&self) -> &'static str;
}

impl Clone for Box<dyn MemoryStructure> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}








