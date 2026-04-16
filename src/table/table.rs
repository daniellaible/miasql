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