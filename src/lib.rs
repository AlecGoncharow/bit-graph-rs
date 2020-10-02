pub mod baseline;
pub mod bit;
pub mod hash;

pub use baseline::AdjGraph;
pub use bit::BitGraph;

pub struct EdgeMeta {
    pub source: usize,
    pub destination: usize,

    pub weight: usize,
}

impl From<(usize, usize, usize)> for EdgeMeta {
    fn from(t: (usize, usize, usize)) -> Self {
        Self {
            source: t.0,
            destination: t.1,
            weight: t.2,
        }
    }
}

impl EdgeMeta {
    #[inline]
    pub fn key_pair(&self) -> (usize, usize) {
        (self.source, self.destination)
    }
}

pub trait Graph<T> {
    /// add a directed edge from `from` and to `to`, represent indicies in some
    /// collection of nodes,left up to the implementation to decide. Weight set to 1
    fn add_edge(&mut self, from: usize, to: usize) -> bool;

    /// remove a directed edge from `from` and to `to`, represent indicies in some
    /// collection of nodes,left up to the implementation to decide.
    fn remove_edge(&mut self, from: usize, to: usize) -> bool;

    /// checks for edge between from `from` to `to` if so returns `true`, else `false`
    fn has_edge(&self, from: usize, to: usize) -> bool;

    /// returns edge between from `from` to `to` if exists, else None
    fn get_edge(&self, from: usize, to: usize) -> Option<&EdgeMeta>;

    /// returns `Vec` of indicies coming out from a given node
    fn outgoing_edges_of(&self, node_index: usize) -> Vec<usize>;

    /// returns `Vec` of indicies coming in to a given node
    fn incoming_edges_of(&self, node_index: usize) -> Vec<usize>;

    /// appends node to graph's node storage
    fn push_node(&mut self, value: T);

    /// sets node at `node_index`
    fn set_node(&mut self, node_index: usize, value: T);

    /// returns given node's value
    fn get_node(&self, node_index: usize) -> &T;

    /// removes node from graph's node storage, removes dependant edges from graph.
    fn remove_node(&mut self, node_index: usize) -> T;
}
