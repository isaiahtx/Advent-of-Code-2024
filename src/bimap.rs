use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Default)]
pub struct BiMap<K> {
    forward: HashMap<K, usize>,
    backward: Vec<K>,
}

impl<K: Hash + Eq + Clone> BiMap<K> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            forward: HashMap::new(),
            backward: Vec::new(),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.forward.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn get_value(&self, id: usize) -> Option<&K> {
        if id > self.len() {
            None
        } else {
            Some(&self.backward[id])
        }
    }

    #[must_use]
    pub fn get_index<Q>(&self, value: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.forward.get(value).copied()
    }

    pub fn insert(&mut self, value: K) -> usize {
        if let Some(&id) = self.forward.get(&value) {
            return id;
        }
        let length = self.backward.len();

        self.forward.insert(value.clone(), length);
        self.backward.push(value);

        length
    }

    #[must_use]
    pub fn contains_value<Q>(&self, value: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.forward.contains_key(value)
    }

    #[must_use]
    pub fn contains_id(&self, id: usize) -> bool {
        id < self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bimap() {
        let mut bmap: BiMap<&str> = BiMap::default();
        bmap.insert("hello there");
        bmap.insert("b");
        bmap.insert("c");
        assert_eq!(bmap.get_index("b"), Some(1));
        assert_eq!(bmap.get_value(2), Some(&"c"));
    }
}
