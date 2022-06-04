use std::{collections::HashMap, hash::Hash};

/// A generic `HashMap`-based graph representation.
///
/// When the edge's type is `bool`, the edge data represents whether the edge connects a vertex
/// to its parent and allows traversing the graph in both directions (from parent to children
/// and viceversa)
#[derive(Debug)]
pub struct Graph<VId, V = (), E = ()> {
    pub vertices: HashMap<VId, V>,
    pub adjacency: HashMap<VId, Vec<(VId, E)>>,
}

impl<VId, V, E> Graph<VId, V, E>
where
    VId: Clone + Eq + Hash,
    V: Hash,
{
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::default(),
            adjacency: HashMap::default(),
        }
    }

    pub fn get_vertex(self: &Self, vid: VId) -> Option<&V> {
        self.vertices.get(&vid)
    }

    pub fn get_vertex_mut(self: &mut Self, vid: VId) -> Option<&mut V> {
        self.vertices.get_mut(&vid)
    }

    pub fn push_vertex(self: &mut Self, vid: VId, vertex: V) {
        self.vertices.insert(vid, vertex);
    }

    pub fn push_edge(self: &mut Self, from: VId, to: VId, edge_data: E) {
        let adjacent_to_from = self.adjacency.entry(from).or_default();
        adjacent_to_from.push((to, edge_data));
    }
}

impl<VId, V> Graph<VId, V, bool>
where
    VId: Clone + Eq + Hash,
    V: Hash,
{
    pub fn get_parent(self: &mut Self, vid: VId) -> Option<VId> {
        // Find the edge connecting to the parent node and return its ID
        let edges = self.adjacency.entry(vid).or_default();
        for (vertex_id, is_parent) in edges {
            if *is_parent {
                return Some(vertex_id.clone());
            }
        }

        // Return None if there isn't a parent node
        None
    }
}
