use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct VertexId(u32);

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Edge(VertexId, VertexId);

pub struct IllegalEdgeError;

pub struct Dag<T> {
    id_counter: u32,
    vertices: HashMap<VertexId, T>,
    edges: Vec<Edge>,

    transitive_closure: Vec<Edge>,
    topological_order: Vec<VertexId>,
}

impl<T> Dag<T> {
    pub fn new() -> Self {
        Dag {
            id_counter: 0,
            vertices: HashMap::new(),
            edges: Vec::new(),

            transitive_closure: Vec::new(),
            topological_order: Vec::new(),
        }
    }

    pub fn vertices(&self) -> Vertices<T> {
        Vertices {
            inner: self.vertices.iter(),
        }
    }

    pub fn edges(&self) -> Edges {
        Edges {
            inner: self.edges.iter(),
        }
    }

    pub fn contains_vertex(&self, v: &VertexId) -> bool {
        self.vertices.contains_key(v)
    }

    pub fn contains_edge(&self, from: &VertexId, to: &VertexId) -> bool {
        self.edges.contains(&Edge(*from, *to))
    }

    pub fn is_reachable(&self, from: &VertexId, to: &VertexId) -> bool {
        self.transitive_closure.contains(&Edge(*from, *to))
    }

    pub fn add_vertex(&mut self, body: T) -> VertexId {
        let id = VertexId(self.id_counter);
        self.id_counter += 1;
        self.vertices.insert(id, body);

        // a new vertex can go anywhere in the topo order
        self.topological_order.push(id);

        return id;
    }

    pub fn add_edge(&mut self, from: &VertexId, to: &VertexId) -> Result<(), IllegalEdgeError> {
        if self.contains_edge(&from, &to) {
            return Ok(());
        } else if self.is_reachable(&to, &from) {
            // edge would create cycle
            return Err(IllegalEdgeError);
        } else if self.contains_vertex(from) && self.contains_vertex(to) {
            self.edges.push(Edge(*from, *to));
            self.recompute_closure();
            return Ok(());
        } else {
            return Err(IllegalEdgeError);
        }
    }

    fn recompute_closure(&mut self) {
        self.topological_order = self.topological_order();
        self.transitive_closure = self.transitive_closure();
    }

    fn transitive_closure(&self) -> Vec<Edge> {
        // initialize closure as adjacency map
        let mut closure = HashMap::new();
        for v in self.topological_order.iter() {
            closure.insert(*v, HashSet::new());
        }

        // iterate backwards and accumulate closures from children to parents
        for v in self.topological_order.iter().rev() {
            closure.get_mut(v).unwrap().insert(*v);
            let v_reachable = closure[v].clone();

            for Edge(from, to) in self.edges.iter() {
                if *v == *to {
                    closure.get_mut(&from).unwrap().extend(v_reachable.clone());
                }
            }
        }

        // convert from adjacency map to edge list
        closure
            .iter()
            .flat_map(|(v, ws)| ws.iter().map(|w| Edge(*v, *w)))
            .collect()
    }

    fn topological_order(&self) -> Vec<VertexId> {
        // count incoming edges to each vertex
        let mut incoming = HashMap::new();
        for vid in self.vertices.keys() {
            incoming.insert(vid.to_owned(), 0);
        }
        for Edge(_from, to) in self.edges.iter() {
            *incoming.get_mut(to).unwrap() += 1;
        }

        // maintain set of all vertices with no incoming edges (source vertices)
        let mut sources = HashSet::new();
        for (vid, count) in incoming.iter() {
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
            for Edge(from, to) in self.edges.iter() {
                if *from == v {
                    *incoming.get_mut(to).unwrap() -= 1;

                    if incoming[to] == 0 {
                        sources.insert(to.to_owned());
                    }
                }
            }
        }

        return topo;
    }
}

pub struct Vertices<'a, T> {
    inner: std::collections::hash_map::Iter<'a, VertexId, T>,
}

impl<'a, T> Iterator for Vertices<'a, T> {
    type Item = (&'a VertexId, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct Edges<'a> {
    inner: std::slice::Iter<'a, Edge>,
}

impl<'a> Iterator for Edges<'a> {
    type Item = &'a Edge;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
