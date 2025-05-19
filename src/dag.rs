use std::collections::{HashMap, HashSet};

pub struct Dag<T> {
    edges: HashMap<T, HashSet<T>>,
    transitive_closure: HashMap<T, HashSet<T>>,
}

impl<T> Dag<T> {
    pub fn new() -> Self {
        Dag {
            edges: HashMap::new(),
            transitive_closure: HashMap::new(),
        }
    }
}
