use uuid::Uuid;


///this class represents a table of the database
mod datatype;
#[path="../bptree.rs"]
mod bptree;

#[derive(Debug)]
#[derive(Clone)]
pub struct Table {
    tablename: String,
    bptree: bptree::BPlusTree<K, V>,
    uuid: Uuid,
}

impl Table {

    pub fn new(tablename: String, bptree: BPlusTree, uuid: Uuid) -> Table {
        Table{tablename, bptree, uuid}
    }

    pub fn get_table_name(&self) -> String {
        return self.tablename.clone();
    }

    pub fn get_bptree(&self) -> &bptree::BPlusTree<K, V> {
        return &self.bptree;
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }
}

#[cfg(test)]
mod tests {
    use super::Table;
    use uuid::Uuid;
    use crate::bptree::BPlusTree;

    #[test]
    fn create_new_table() {
        let mut bpTree = BPlusTree::default();
        let table: Table =  Table::new(String::from("test"), bpTree, Uuid::new_v4());
        let name: String =  table.get_table_name();
        assert_eq!(name, "test");
    }
}