mod datatypes;

//this class represents a table of the database

#[derive(Debug)]
#[derive(Clone)]
pub struct Table {
    tablename: String,
    bptree: bptree::BPlusTree,
    uuid: Uuid,
}

impl Table {

    pub fn new(tablename: String, tree: BPlusTree, uuid: Uuid) -> Table {
        Table(tablename, tree, uuid)
    }

    pub fn new(tablename: String,  uuid: Uuid) -> Table {
        Table(tablename, BPlusTree::new(), uuid)
    }

    pub fn get_table_name(&self) -> String {
        return self.tablename.clone();
    }

    pub fn get_bptree(&self) -> &bptree::BPlusTree {
        return &self.bptree;
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

}