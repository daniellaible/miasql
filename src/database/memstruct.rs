
pub trait MemoryStructure<V, K>{
    fn create(&self) -> Self;
    fn insert(&mut self, value:V, id:K);
    fn retrieve_values(&self, value:K) -> V;
    fn retrieve_keys(&self, value:K) -> Vec<K>;
    fn delete (&mut self, id:K);
}






