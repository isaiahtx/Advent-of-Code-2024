use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

/// Provides a dictionary to convert between a generic type K and usize
#[derive(Debug, Clone, Default)]
pub struct BiMap<K> {
    forward: HashMap<K, usize>,
    backward: Vec<K>,
}

// When inserting a key in a BiMap, will output the resulting id as
// Contained(id) if the BiMap already contained the key, otherwise we will
// output DidNotContain(id).
#[derive(Clone, Copy)]
pub enum InsertResult {
    Contained(usize),
    DidNotContain(usize),
}

impl InsertResult {
    #[must_use]
    pub const fn unwrap(self) -> usize {
        match self {
            Self::Contained(key) | Self::DidNotContain(key) => key,
        }
    }
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
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            forward: HashMap::with_capacity(capacity),
            backward: Vec::with_capacity(capacity),
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
    pub fn get_id<Q>(&self, value: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.forward.get(value).copied()
    }

    pub fn insert(&mut self, value: K) -> InsertResult {
        if let Some(&id) = self.forward.get(&value) {
            InsertResult::Contained(id)
        } else {
            let length = self.backward.len();

            self.forward.insert(value.clone(), length);
            self.backward.push(value);

            InsertResult::DidNotContain(length)
        }
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
        assert_eq!(bmap.get_id("b"), Some(1));
        assert_eq!(bmap.get_value(2), Some(&"c"));
    }
}
