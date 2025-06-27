use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct DeviceId(u32);

pub type Edge = (DeviceId, DeviceId);

pub struct IllegalEdgeError;

pub struct Dag {
    id_counter: u32,
    edges: Vec<Edge>,
    topological_order: Vec<DeviceId>,
    transitive_closure: Vec<Edge>,
}

impl Dag {
    pub fn new() -> Self {
        Dag {
            id_counter: 0,
            edges: Vec::new(),
            transitive_closure: Vec::new(),
            topological_order: Vec::new(),
        }
    }

    pub fn vertices(&self) -> impl Iterator<Item = &DeviceId> {
        self.topological_order.iter()
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }

    pub fn parents(&self, child: DeviceId) -> impl Iterator<Item = &DeviceId> {
        self.edges.iter().filter_map(
            move |(from, to)| {
                if *to == child {
                    Some(from)
                } else {
                    None
                }
            },
        )
    }

    pub fn contains_vertex(&self, v: DeviceId) -> bool {
        self.topological_order.contains(&v)
    }

    pub fn contains_edge(&self, e: Edge) -> bool {
        self.edges.contains(&e)
    }

    pub fn is_reachable(&self, from: DeviceId, to: DeviceId) -> bool {
        self.transitive_closure.contains(&(from, to))
    }

    pub fn add_vertex(&mut self) -> DeviceId {
        let id = DeviceId(self.id_counter);
        self.id_counter += 1;

        // a new vertex can go anywhere in the topo order
        self.topological_order.push(id);

        id
    }

    pub fn add_edge(&mut self, e: Edge) -> Result<(), IllegalEdgeError> {
        let (from, to) = e;

        if self.contains_edge(e) {
            Ok(())
        } else if self.is_reachable(to, from) {
            // edge would create cycle
            Err(IllegalEdgeError)
        } else if self.contains_vertex(from) && self.contains_vertex(to) {
            self.edges.push(e);
            self.recompute_caches();
            Ok(())
        } else {
            // edge includes nonexistent vertices
            Err(IllegalEdgeError)
        }
    }

    pub fn remove_vertex(&mut self, v: DeviceId) {
        self.edges.retain(|(from, to)| *from != v && *to != v);
        self.topological_order.retain(|x| *x != v);

        self.recompute_caches();
    }

    pub fn remove_edge(&mut self, e: Edge) {
        self.edges.retain(|x| *x != e);

        self.recompute_caches();
    }

    /// The Dag keeps a cached copy of its own topological ordering and
    /// transitive closure because these structures are accessed way more often
    /// than the graph is modified.
    ///
    /// This function recomputes those cached structures and should be called
    /// whenever the graph topology changes.
    fn recompute_caches(&mut self) {
        self.recompute_topological_order();
        self.recompute_transitive_closure();
    }

    fn recompute_transitive_closure(&mut self) {
        // initialize closure as adjacency map
        let mut closure = HashMap::new();
        for v in self.topological_order.iter() {
            closure.insert(*v, HashSet::new());
        }

        // iterate backwards and accumulate closures from children to parents
        for v in self.topological_order.iter().rev() {
            closure.get_mut(v).unwrap().insert(*v);
            let v_reachable = closure[v].clone();

            for (from, to) in self.edges.iter() {
                if *v == *to {
                    closure.get_mut(&from).unwrap().extend(v_reachable.clone());
                }
            }
        }

        // convert from adjacency map to edge list
        self.transitive_closure = closure
            .iter()
            .flat_map(|(v, ws)| ws.iter().map(|w| (*v, *w)))
            .collect();
    }

    fn recompute_topological_order(&mut self) {
        // count incoming edges to each vertex
        let mut incoming = HashMap::new();
        for vid in self.topological_order.iter() {
            incoming.insert(vid.to_owned(), 0);
        }
        for (_from, to) in self.edges.iter() {
            *incoming.get_mut(to).unwrap() += 1;
        }

        // maintain set of all vertices with no incoming edges (source vertices)
        let mut sources = HashSet::new();
        for (vid, count) in incoming.iter() {
            if *count == 0 {
                sources.insert(vid.to_owned());
            }
        }

        let mut topo = Vec::with_capacity(self.topological_order.len());
        while !sources.is_empty() {
            // pop some vertex with no incoming edges
            let v = sources.iter().next().unwrap().clone();
            sources.remove(&v);

            // make it next in the topological ordering
            topo.push(v);

            // remove outgoing edges from that vertex, and add any new sources
            for (from, to) in self.edges.iter() {
                if *from == v {
                    *incoming.get_mut(to).unwrap() -= 1;

                    if incoming[to] == 0 {
                        sources.insert(to.to_owned());
                    }
                }
            }
        }

        self.topological_order = topo;
    }
}
