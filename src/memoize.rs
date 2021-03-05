use std::{collections::HashMap, hash::Hash};

pub struct Memoize<F, A, B>
where
    F: FnMut(A) -> B,
{
    cache: HashMap<A, B>,
    f: F,
}

impl<F, A, B> Memoize<F, A, B>
where
    F: FnMut(A) -> B,
    A: Eq + Hash + Copy,
    B: Copy,
{
    pub fn new(f: F) -> Self {
        Memoize {
            cache: HashMap::new(),
            f,
        }
    }

    pub fn call(&mut self, x: A) -> B {
        if !self.cache.contains_key(&x) {
            self.cache.insert(x, (self.f)(x));
        }
        self.cache[&x]
    }
}
