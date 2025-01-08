use std::collections::HashMap;
use std::hash::Hash;

pub struct Memoizer<F, I, O>
where
    F: Fn(I) -> O,
{
    function: F,
    map: HashMap<I, O>,
}

impl<F, I, O> Memoizer<F, I, O>
where
    I: Hash + Eq + Clone,
    O: Clone,
    F: Fn(I) -> O,
{
    pub fn new(function: F) -> Self {
        Self {
            function,
            map: HashMap::new(),
        }
    }

    pub fn call(&mut self, arg: I) -> O {
        let f = &self.function;
        self.map
            .entry(arg.clone())
            .or_insert_with(|| (f)(arg))
            .clone()
    }
}
