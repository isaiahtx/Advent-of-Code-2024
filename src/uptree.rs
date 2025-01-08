#![allow(dead_code)]
use crate::bimap::{BiMap, InsertResult::Contained, InsertResult::DidNotContain};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Up Tree data structure with weighted nodes
#[derive(Default, Debug, Clone)]
pub struct UpTree<T, W = ()> {
    dict: BiMap<T>,
    up: Vec<(Option<usize>, W)>,
    num_nodes: usize,
    num_roots: usize,
}

impl<T, W> UpTree<T, W>
where
    T: Hash + Eq + Clone,
    W: Clone,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            dict: BiMap::new(),
            up: Vec::new(),
            num_nodes: 0,
            num_roots: 0,
        }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            dict: BiMap::with_capacity(capacity),
            up: Vec::with_capacity(capacity),
            num_nodes: 0,
            num_roots: 0,
        }
    }

    #[must_use]
    pub fn contains(&self, node: &T) -> bool {
        self.dict.contains_value(node)
    }

    fn id_is_root(&self, id: usize) -> Option<bool> {
        self.up.get(id).map(|(p, _)| Some(p.is_none()))?
    }

    pub fn is_root(&self, node: &T) -> Option<bool> {
        self.dict
            .get_id(node)
            .map(|id| Some(self.up[id].0.is_none()))?
    }

    pub fn insert_root(&mut self, root: T) -> bool
    where
        W: Default,
    {
        self.insert_root_weighted((root, W::default()))
    }

    pub fn insert_root_weighted(&mut self, root: (T, W)) -> bool {
        match self.dict.insert(root.0) {
            Contained(id) => {
                if self.up[id].0.is_some() {
                    self.up[id].0 = None;
                    self.num_roots += 1;
                }
                false
            }
            DidNotContain(_) => {
                self.up.push((None, root.1));
                self.num_nodes += 1;
                self.num_roots += 1;
                true
            }
        }
    }

    /// # Errors
    /// Returns error if imputs are equal.
    pub fn insert(&mut self, child: T, parent: T) -> Result<(), String>
    where
        W: Default,
    {
        if child == parent {
            Err("Inputs cannot be equal".to_string())
        } else {
            self.insert_weighted((child, W::default()), (parent, W::default()))
        }
    }

    /// # Errors
    /// Returns error if inputs have equal identifiers
    pub fn insert_weighted(&mut self, child: (T, W), parent: (T, W)) -> Result<(), String> {
        if child.0 == parent.0 {
            Err("Inputs cannot be equal".to_string())
        } else {
            let child_id = self.dict.insert(child.0);
            let parent_id = self.dict.insert(parent.0);

            match child_id {
                Contained(cid) => {
                    // println!("Warning: Re-assigning parent of existing node!");
                    if self.up[cid].0.is_none() {
                        self.num_roots -= 1;
                    }
                    self.up[cid] = (Some(parent_id.unwrap()), child.1);
                }
                DidNotContain(_) => {
                    self.up.push((Some(parent_id.unwrap()), child.1));
                    self.num_nodes += 1;
                }
            }

            if let DidNotContain(_) = parent_id {
                self.up.push((None, parent.1));
                self.num_roots += 1;
                self.num_nodes += 1;
            }

            Ok(())
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.num_nodes
    }

    #[must_use]
    pub const fn num_roots(&self) -> usize {
        self.num_roots
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.num_nodes == 0
    }

    /// # Panics
    ///
    /// Will never
    #[must_use]
    fn up_to_original(self) -> HashSet<(T, Option<T>, W)>
    where
        W: Hash + Eq,
    {
        let output = self
            .up
            .into_iter()
            .enumerate()
            .map(|(i, (j, w))| {
                (
                    self.dict.get_value(i).unwrap().clone(),
                    j.map(|id| self.dict.get_value(id).unwrap().clone()),
                    w,
                )
            })
            .collect();

        output
    }

    pub fn find(&mut self, node: &T) -> Option<T> {
        self.dict
            .get_id(node)
            .map(|id| self.find_by_id(id))?
            .map(|id| self.dict.get_value(id))?
            .cloned()
    }

    fn find_by_id(&mut self, id: usize) -> Option<usize> {
        if id >= self.num_nodes {
            None
        } else if let (Some(parent_id), _) = self.up[id] {
            let output = self
                .find_by_id(parent_id)
                .expect("found child pointing to non-existent parent!");
            self.up[id].0 = Some(output);
            Some(output)
        } else {
            Some(id)
        }
    }

    pub fn find_no_collapse(&self, node: &T) -> Option<T> {
        self.dict
            .get_id(node)
            .map(|id| self.find_by_id_no_collapse(id))?
            .map(|id| self.dict.get_value(id))?
            .cloned()
    }

    fn find_by_id_no_collapse(&self, id: usize) -> Option<usize> {
        if id >= self.num_nodes {
            None
        } else if let (Some(parent_id), _) = self.up[id] {
            Some(
                self.find_by_id_no_collapse(parent_id)
                    .expect("found child pointing to non-existent parent!"),
            )
        } else {
            Some(id)
        }
    }

    fn union_by_id(&mut self, x: usize, y: usize) -> bool {
        if x >= self.num_nodes || y >= self.num_nodes {
            false
        } else {
            let root_x = self.find_by_id(x).expect("invalid up tree!");
            let root_y = self.find_by_id(y).expect("invalid up tree!");
            if root_x != root_y {
                assert!(self.up[root_x].0.is_none());
                self.up[root_x].0 = Some(root_y);
                self.num_roots -= 1;
            }
            true
        }
    }

    pub fn union(&mut self, x: &T, y: &T) -> bool {
        if let Some(id_x) = self.dict.get_id(x) {
            if let Some(id_y) = self.dict.get_id(y) {
                return self.union_by_id(id_x, id_y);
            }
        }
        false
    }

    pub fn get_sizes(&mut self) -> Vec<(T, usize)> {
        let mut sets: HashMap<usize, Vec<usize>> = HashMap::new();
        for id in 0..self.num_nodes {
            if let Some(parent_id) = self.find_by_id(id) {
                sets.entry(parent_id).or_default().push(id);
            }
        }

        sets.values()
            .map(|v| (self.dict.get_value(v[0]).unwrap().clone(), v.len()))
            .collect()
    }

    pub fn flatten(&mut self) -> Vec<Vec<(&T, &W)>> {
        let mut sets: HashMap<usize, Vec<usize>> = HashMap::new();
        for id in 0..self.num_nodes {
            if let Some(parent_id) = self.find_by_id(id) {
                sets.entry(parent_id).or_default().push(id);
            }
        }

        sets.values()
            .map(|v| {
                v.iter()
                    .map(|&id| {
                        let value = self.dict.get_value(id).unwrap();
                        let weight = &self.up[id].1;
                        (value, weight)
                    })
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_insert() {
        let mut ut: UpTree<char> = UpTree::with_capacity(9);
        let _ = ut.insert('a', 'c');
        let _ = ut.insert('b', 'c');
        let _ = ut.insert('c', 'f');
        let _ = ut.insert('d', 'f');
        let _ = ut.insert('e', 'f');
        let _ = ut.insert('g', 'i');
        let _ = ut.insert('h', 'i');

        let _ = ut.insert('c', 'h');
        ut.insert_root('h');

        let expected: HashSet<(char, Option<char>, ())> = vec![
            ('a', Some('c'), ()),
            ('b', Some('c'), ()),
            ('c', Some('h'), ()),
            ('d', Some('f'), ()),
            ('e', Some('f'), ()),
            ('f', None, ()),
            ('g', Some('i'), ()),
            ('h', None, ()),
            ('i', None, ()),
        ]
        .into_iter()
        .collect();

        assert!(ut.is_root(&'i').unwrap());
        assert!(ut.is_root(&'X').is_none());
        assert_eq!(expected, ut.up_to_original());
    }

    #[test]
    fn test_find_1() {
        let mut ut: UpTree<usize> = UpTree::new();

        let _ = ut.insert(0, 1);
        let _ = ut.insert(1, 2);
        let _ = ut.insert(2, 3);
        let _ = ut.insert(3, 4);

        assert_eq!(ut.find(&0), Some(4));

        for i in 0..4 {
            assert_eq!(ut.up[i].0, Some(4));
        }
    }

    #[test]
    fn test_find_2() {
        let mut ut: UpTree<char> = UpTree::with_capacity(9);
        let _ = ut.insert('a', 'c');
        let _ = ut.insert('b', 'c');
        let _ = ut.insert('c', 'f');
        let _ = ut.insert('d', 'f');
        let _ = ut.insert('e', 'f');
        let _ = ut.insert('g', 'i');
        let _ = ut.insert('h', 'i');

        let _ = ut.insert('c', 'h');
        ut.insert_root('h');

        assert_eq!(ut.find(&'a'), Some('h'));
        assert_eq!(ut.find(&'b'), Some('h'));
        assert_eq!(ut.find(&'c'), Some('h'));
        assert_eq!(ut.find(&'d'), Some('f'));
        assert_eq!(ut.find(&'e'), Some('f'));
        assert_eq!(ut.find(&'f'), Some('f'));
        assert_eq!(ut.find(&'g'), Some('i'));
        assert_eq!(ut.find(&'h'), Some('h'));
        assert_eq!(ut.find(&'i'), Some('i'));

        let expected: HashSet<(char, Option<char>, ())> = HashSet::from([
            ('a', Some('h'), ()),
            ('b', Some('h'), ()),
            ('c', Some('h'), ()),
            ('d', Some('f'), ()),
            ('e', Some('f'), ()),
            ('f', None, ()),
            ('g', Some('i'), ()),
            ('h', None, ()),
            ('i', None, ()),
        ]);
        assert_eq!(expected, ut.up_to_original());
    }

    #[test]
    fn test_insert_no_collapse() {
        let mut ut: UpTree<char> = UpTree::with_capacity(9);
        let _ = ut.insert('a', 'c');
        let _ = ut.insert('b', 'c');
        let _ = ut.insert('c', 'f');
        let _ = ut.insert('d', 'f');
        let _ = ut.insert('e', 'f');
        let _ = ut.insert('g', 'i');
        let _ = ut.insert('h', 'i');

        let _ = ut.insert('c', 'h');
        ut.insert_root('h');

        assert_eq!(ut.find_no_collapse(&'a'), Some('h'));
        assert_eq!(ut.find_no_collapse(&'b'), Some('h'));
        assert_eq!(ut.find_no_collapse(&'c'), Some('h'));
        assert_eq!(ut.find_no_collapse(&'d'), Some('f'));
        assert_eq!(ut.find_no_collapse(&'e'), Some('f'));
        assert_eq!(ut.find_no_collapse(&'f'), Some('f'));
        assert_eq!(ut.find_no_collapse(&'g'), Some('i'));
        assert_eq!(ut.find_no_collapse(&'h'), Some('h'));
        assert_eq!(ut.find_no_collapse(&'i'), Some('i'));

        let expected: HashSet<(char, Option<char>, ())> = HashSet::from([
            ('a', Some('c'), ()),
            ('b', Some('c'), ()),
            ('c', Some('h'), ()),
            ('d', Some('f'), ()),
            ('e', Some('f'), ()),
            ('f', None, ()),
            ('g', Some('i'), ()),
            ('h', None, ()),
            ('i', None, ()),
        ]);

        assert_eq!(ut.len(), 9);
        assert!(ut.is_root(&'i').unwrap());
        assert!(ut.is_root(&'X').is_none());
        assert_eq!(expected, ut.up_to_original());
    }

    #[test]
    fn test_union_1() {
        let mut ut: UpTree<char> = UpTree::with_capacity(9);
        let _ = ut.insert('a', 'c');
        let _ = ut.insert('b', 'c');
        let _ = ut.insert('c', 'f');
        let _ = ut.insert('d', 'f');
        let _ = ut.insert('e', 'f');
        let _ = ut.insert('g', 'i');
        let _ = ut.insert('h', 'i');
        let _ = ut.insert('j', 'a');

        let _ = ut.insert('c', 'h');
        ut.insert_root('h');

        assert!(ut.union(&'f', &'i'));
        assert!(ut.union(&'j', &'e'));
        assert!(!ut.union(&'X', &'a'));
        assert!(!ut.union(&'a', &'Y'));

        let expected: HashSet<(char, Option<char>, ())> = HashSet::from([
            ('i', None, ()),
            ('f', Some('i'), ()),
            ('e', Some('i'), ()),
            ('g', Some('i'), ()),
            ('h', Some('i'), ()),
            ('d', Some('f'), ()),
            ('j', Some('h'), ()),
            ('a', Some('h'), ()),
            ('c', Some('h'), ()),
            ('b', Some('c'), ()),
        ]);

        assert_eq!(expected, ut.up_to_original());
    }

    #[test]
    fn test_num_roots_1() {
        for _ in 0..100 {
            let mut ut: UpTree<usize> = UpTree::new();

            let mut rng = rand::thread_rng();
            for (a, b) in (0..500).map(move |_| {
                let a = rng.gen_range(0..=100);
                let b = rng.gen_range(0..=100);
                (a as usize, b as usize)
            }) {
                let _ = ut.insert(a, b);
            }

            let mut num_roots = 0;

            for (x, ()) in ut.up {
                if x.is_none() {
                    num_roots += 1;
                }
            }

            assert_eq!(num_roots, ut.num_roots);
        }
    }
}
