use std::collections::HashMap;

pub struct Counter<T> {
    map: HashMap<T, usize>,
}

impl<T: std::hash::Hash + Eq> Counter<T> {
    pub fn from(array: Vec<T>) -> HashMap<T, usize> {
        let mut _map: HashMap<T, usize> = HashMap::new();
        for item in array {
            *_map.entry(item).or_insert(0) += 1;
        }
        _map
    }
}
