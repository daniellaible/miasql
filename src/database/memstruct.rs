// here are all the memory structures defined except the bptree.
// BpTrees are just too much code so they got an extra file

pub trait memory_structure{
    fn insert(&self);
    fn retrieve(&self);
    fn delete (&self);
}






