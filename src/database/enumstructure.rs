use log::error;
use crate::database::mapstructure::HashmapStructure;
use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::table::Row;

/// This struct stores the [RowId]s of enum values. The data for each enum value is stored in a tupel.
/// The first element of the tupel is an [IndexValue] that stores the value of the enum as text.
/// The second value of the tupel stores a vector with the Ids of the rows that use this specifc enum
/// value.
#[derive(Debug, Clone, Default)]
pub struct EnumStructure {
    pub values: Vec<(IndexValue, Vec<RowId>)>
}

/// This implementaion of the [MemoryStructure] trait is used for the [DataType::Enum]
impl MemoryStructure for EnumStructure {

    /// This function is used to add a reference to a table row
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Text(enum_value) => {
                for i in 0 ..self.values.len(){
                    let tupel = &mut self.values[i];
                    let indexvalue = &tupel.0;
                    match indexvalue {
                        IndexValue::Text(enum_as_text) => {
                            if *enum_as_text == enum_value{
                                &tupel.1.push(id);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    /// This call gives you all the [RowId]s of the [Row]s that have this special enum value.
    /// Form here on you can access the [Row]s directly using the main [HashmapStructure].
    /// Make sure that the key is of type [IndexValue::Text]
    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
       let index_value =  key.clone();
        match index_value{
            IndexValue::Text(enum_val) => {
                for i in 0 .. self.values.len(){
                    let tupel = &self.values[i];
                    let saved_index_value = tupel.0.clone();
                    match saved_index_value {
                        IndexValue::Text(value) => {
                            let enum_val_upper = enum_val.to_uppercase();
                            if  enum_val_upper == value{
                                return tupel.1.clone();
                            }
                        },
                        _ => error!("How the heck did we end up here where the saved value is not of type IndexValue::Text")
                    }
                }
            }
            _ => error!("The key must be of type IndexValue::Text")
        }
        vec![]
    }

    /// Retrieving by index does not make any sense in this case. It would be possible,
    /// however, incredibly slow therefore, this function call panics.
    /// If you want to retrieve the enums data use the main [HashmapStructure] to retrieve
    /// the complete [Row] and access the element within the [Row] directly.
    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
        panic!("EnumStructure uses retrieve_range function to access the data")
    }

    fn delete(&mut self, id: RowId) {

    }

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    fn kind(&self) -> &'static str { "enum" }
}

#[cfg(test)]
mod tests {
    use crate::database::enumstructure::EnumStructure;
    use crate::database::memstruct::{IndexValue, MemoryStructure};

    #[test]
    fn insert_test(){
        let high_tupel = (IndexValue::Text("HIGH".to_string()), Vec::new());
        let medium_tupel = (IndexValue::Text("MEDIUM".to_string()), Vec::new());
        let low_tupel = (IndexValue::Text("LOW".to_string()), Vec::new());

        let mut tupel_vec = Vec::new();
        tupel_vec.push(high_tupel);
        tupel_vec.push(medium_tupel);
        tupel_vec.push(low_tupel);
        let mut enum_structure = EnumStructure{
            values: tupel_vec
        };

        enum_structure.insert(IndexValue::Text("HIGH".to_string()), 1);
        enum_structure.insert(IndexValue::Text("HIGH".to_string()), 2);
        enum_structure.insert(IndexValue::Text("HIGH".to_string()), 3);
        enum_structure.insert(IndexValue::Text("MEDIUM".to_string()), 4);
        enum_structure.insert(IndexValue::Text("MEDIUM".to_string()), 5);
        enum_structure.insert(IndexValue::Text("LOW".to_string()), 6);
        enum_structure.insert(IndexValue::Text("LOW".to_string()), 7);

        let high_result = &enum_structure.values[0];
        let high_index_value = &high_result.0;
        match high_index_value{
            IndexValue::Text(val) => {
                assert_eq!(val.to_string(), "HIGH".to_string())
            }
            _ => assert!(false)
        }
        let high_ids = &high_result.1;
        assert_eq!(high_ids[0], 1);
        assert_eq!(high_ids[1], 2);
        assert_eq!(high_ids[2], 3);

        let medium_result = &enum_structure.values[1];
        let medium_index_value = &medium_result.0;
        match medium_index_value{
            IndexValue::Text(val) => {
                assert_eq!(val.to_string(), "MEDIUM".to_string())
            }
            _ => assert!(false)
        }
        let medium_ids = &medium_result.1;
        assert_eq!(medium_ids[0], 4);
        assert_eq!(medium_ids[1], 5);

        let low_result = &enum_structure.values[2];
        let low_index_value = &low_result.0;
        match low_index_value{
            IndexValue::Text(val) => {
                assert_eq!(val.to_string(), "LOW".to_string())
            }
            _ => assert!(false)
        }
        let low_ids = &low_result.1;
        assert_eq!(low_ids[0], 6);
        assert_eq!(low_ids[1], 7);

        let low_result = &enum_structure.values[2];
        println!("{:?}", low_result);
    }

    #[test]
    fn receive_test(){
        let enum_struct = insert();
        let result_high = enum_struct.retrieve_range(&IndexValue::Text("HIGH".to_string()));
        let result_medium = enum_struct.retrieve_range(&IndexValue::Text("MEDIUM".to_string()));
        let result_low = enum_struct.retrieve_range(&IndexValue::Text("LOW".to_string()));
        assert_eq!(result_high.len(), 3);
        assert_eq!(result_medium.len(), 2);
        assert_eq!(result_low.len(), 2);
    }

    fn insert() -> EnumStructure{
        let high_tupel = (IndexValue::Text("HIGH".to_string()), Vec::new());
        let medium_tupel = (IndexValue::Text("MEDIUM".to_string()), Vec::new());
        let low_tupel = (IndexValue::Text("LOW".to_string()), Vec::new());

        let mut tupel_vec = Vec::new();
        tupel_vec.push(high_tupel);
        tupel_vec.push(medium_tupel);
        tupel_vec.push(low_tupel);
        let mut enum_structure = EnumStructure{
            values: tupel_vec
        };

        enum_structure.insert(IndexValue::Text("HIGH".to_string()), 1);
        enum_structure.insert(IndexValue::Text("HIGH".to_string()), 2);
        enum_structure.insert(IndexValue::Text("HIGH".to_string()), 3);
        enum_structure.insert(IndexValue::Text("MEDIUM".to_string()), 4);
        enum_structure.insert(IndexValue::Text("MEDIUM".to_string()), 5);
        enum_structure.insert(IndexValue::Text("LOW".to_string()), 6);
        enum_structure.insert(IndexValue::Text("LOW".to_string()), 7);
        enum_structure
    }
}