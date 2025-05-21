use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

struct IllegalEdgeError;

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

    pub fn contains_vertex(&self, v: &T) -> bool
    where
        T: Hash + Eq,
    {
        self.edges.contains_key(&v)
    }

    pub fn contains_edge(&self, from: &T, to: &T) -> bool
    where
        T: Hash + Eq,
    {
        match self.edges.get(from) {
            Some(children) => children.contains(to),
            None => false,
        }
    }

    pub fn contains_path(&self, from: &T, to: &T) -> bool
    where
        T: Hash + Eq,
    {
        match self.transitive_closure.get(from) {
            Some(children) => children.contains(to),
            None => false,
        }
    }

    pub fn insert_vertex(&mut self, v: T)
    where
        T: Hash + Eq + Copy,
    {
        if self.contains_vertex(&v) {
            return;
        }
        self.edges.insert(v, HashSet::new());
        self.transitive_closure.insert(v, HashSet::new());
    }

    pub fn insert_edge(&mut self, from: T, to: T) -> Result<(), IllegalEdgeError>
    where
        T: Hash + Eq + Copy,
    {
        if self.contains_edge(&from, &to) {
            return Ok(());
        } else if self.contains_path(&to, &from) {
            return Err(IllegalEdgeError);
        } else {
            self.insert_vertex(from);
            self.insert_vertex(to);
            let children = self.edges.get_mut(&from).unwrap();
            children.insert(to);
            self.recompute_closure();

            return Ok(());
        }
    }

    fn recompute_closure(&mut self) {
        todo!()
    }
}
