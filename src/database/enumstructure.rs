use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::tabel::Row;

#[derive(Debug, Clone, Default)]
pub struct EnumStructure {
    pub values: Vec<IndexValue>,
    pub ids: Vec<Vec<RowId>>
}

impl MemoryStructure for EnumStructure {
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Text(ident) => {
                for i in self.values.len().. {
                    if let IndexValue::Text(enum_value) = &self.values[i]{
                        let enum_lower = enum_value.to_lowercase().clone();
                        let ident = ident.to_lowercase();
                        if ident == enum_lower{
                            &self.ids[i].push(id);
                        }
                    }
                }
            }
            _ => {panic!("EnumStructure just accepts IndexValue::Text type")}
        }
    }

    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        match key {
            IndexValue::Text(ident) => {
                for i in self.values.len().. {
                    if let IndexValue::Text(enum_val) = &self.values[i]{
                        let enum_lower = enum_val.to_lowercase().clone();
                        let ident_lower = ident.to_lowercase().clone();
                        if enum_lower == ident_lower {
                            return self.ids[i].clone();
                        }
                    }
                }
                vec![]
            },
            _ => {panic!("EnumStructure accespts only IndexValue::Text type to retrieve")}
        }
    }

    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
        panic!("EnumStructure uses retrieve_by_other function to access the data")
    }

    fn delete(&mut self, id: RowId) {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    fn kind(&self) -> &'static str { "enum" }
}

#[cfg(test)]
mod tests {

}