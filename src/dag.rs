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

    sources: Vec<VertexId>,
    transitive_closure: HashMap<VertexId, HashSet<VertexId>>,
}

impl<T> Dag<T> {
    pub fn new() -> Self {
        Dag {
            id_counter: 0,
            vertices: HashMap::new(),
            edges: HashMap::new(),

            sources: Vec::new(),

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

    fn topological_order(&self) -> Vec<VertexId> {
        // count incoming edges to each vertex
        let mut outgoing = HashMap::new();
        for vid in self.vertices.keys() {
            outgoing.insert(vid.to_owned(), 0);
        }
        for children in self.edges.values() {
            for vid in children {
                outgoing[vid] += 1;
            }
        }

        // maintain set of all vertices with no incoming edges (source vertices)
        let mut sources = HashSet::new();
        for (vid, count) in outgoing.iter() {
            if *count == 0 {
                sources.insert(vid.to_owned());
            }
        }

        let mut topo = Vec::with_capacity(self.vertices.len());
        while !sources.is_empty() {
            // pop some vertex with no incoming edges
            let v = sources.iter().next().unwrap().clone();
            sources.remove(&v);

            // make it next in the topological ordering
            topo.push(v);

            // remove outgoing edges from that vertex, and add any new sources
            for w in self.edges.get(&v).unwrap() {
                outgoing[w] -= 1;

                if outgoing[w] == 0 {
                    sources.insert(w.to_owned());
                }
            }
        }

        return topo;
    }
}
