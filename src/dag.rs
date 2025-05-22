use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct VertexId(u32);

pub struct IllegalEdgeError;

pub struct Dag<T> {
    id_counter: u32,
    // maybe replace the hashmaps with some kind of vec arena if performance is a problem
    vertices: HashMap<VertexId, T>,
    edges: HashMap<VertexId, HashSet<VertexId>>,
    transitive_closure: HashMap<VertexId, HashSet<VertexId>>,
}

impl<T> Dag<T> {
    pub fn new() -> Self {
        Dag {
            id_counter: 0,
            vertices: HashMap::new(),
            edges: HashMap::new(),
            transitive_closure: HashMap::new(),
        }
    }

    pub fn contains_vertex(&self, v: &VertexId) -> bool {
        self.vertices.contains_key(v)
    }

    pub fn contains_edge(&self, from: &VertexId, to: &VertexId) -> bool {
        match self.edges.get(from) {
            Some(children) => children.contains(to),
            None => false,
        }
    }

    pub fn contains_path(&self, from: &VertexId, to: &VertexId) -> bool {
        match self.transitive_closure.get(from) {
            Some(children) => children.contains(to),
            None => false,
        }
    }

    pub fn add_vertex(&mut self, body: T) -> VertexId {
        let id = VertexId(self.id_counter);
        self.id_counter += 1;
        self.vertices.insert(id, body);
        self.edges.insert(id, HashSet::new());
        self.transitive_closure.insert(id, HashSet::new());

        return id;
    }

    pub fn add_edge(&mut self, from: &VertexId, to: &VertexId) -> Result<(), IllegalEdgeError> {
        if self.contains_edge(&from, &to) {
            return Ok(());
        } else if self.contains_path(&to, &from) {
            return Err(IllegalEdgeError);
        } else {
            match self.edges.get_mut(&from) {
                Some(children) => {
                    children.insert(*to);
                    self.recompute_closure();
                    return Ok(());
                }
                None => {
                    return Err(IllegalEdgeError);
                }
            }
        }
    }

    fn recompute_closure(&mut self) {
        todo!()
    }
}
