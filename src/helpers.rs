use std::collections::HashMap;

pub trait AppendableMap<
    K: std::cmp::Eq + std::hash::Hash,
    V
> {
    fn append(&mut self, other: HashMap<K, Vec<V>>) -> Result<(), ()>;
}

impl<
    K: std::cmp::Eq + std::hash::Hash,
    V
> AppendableMap<K, V> for HashMap<K, Vec<V>> {
    fn append(&mut self, other: HashMap<K, Vec<V>>) -> Result<(), ()> {
        for (key, mut value) in other {
            match self.get_mut(&key) {
                Some(old_val) => {
                    old_val.append(&mut value);
                }
                None => {
                    self.insert(key, value);
                }
            }
        }
        Ok(())
    }
}