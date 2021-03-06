pub mod baseline;
pub mod bit;
pub mod hash;
pub mod search;

pub use baseline::AdjGraph;
pub use bit::BitGraph;

#[derive(Clone, Copy)]
pub struct EdgeMeta<W> {
    pub source: usize,
    pub destination: usize,

    pub weight: W,
}

impl From<(usize, usize, usize)> for EdgeMeta<usize> {
    fn from(t: (usize, usize, usize)) -> Self {
        Self {
            source: t.0,
            destination: t.1,
            weight: t.2,
        }
    }
}

impl EdgeMeta<usize> {
    #[inline]
    pub fn key_pair(&self) -> (usize, usize) {
        (self.source, self.destination)
    }
}

pub trait Graph<T, W> {
    /// add a directed edge from `from` and to `to`, represent indicies in some
    /// collection of nodes,left up to the implementation to decide. Weight set to 1
    fn add_edge(&mut self, from: usize, to: usize) -> bool;

    fn set_edge(&mut self, from_to: (usize, usize), weight: W) -> bool;

    /// remove a directed edge from `from` and to `to`, represent indicies in some
    /// collection of nodes,left up to the implementation to decide.
    fn remove_edge(&mut self, from: usize, to: usize) -> bool;

    /// checks for edge between from `from` to `to` if so returns `true`, else `false`
    fn has_edge(&self, from: usize, to: usize) -> bool;

    /// returns edge between from `from` to `to` if exists, else None
    fn get_edge(&self, from: usize, to: usize) -> Option<EdgeMeta<W>>;

    /// returns `Vec` of indicies coming out from a given node
    fn outgoing_edges_of(&self, node_index: usize) -> Vec<usize>;

    /// returns `Vec` of indicies coming in to a given node
    fn incoming_edges_of(&self, node_index: usize) -> Vec<usize>;

    fn all_edge_pairs(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for index in 0..self.node_count() {
            self.outgoing_edges_of(index)
                .iter()
                .for_each(|to| out.push((index, *to)))
        }
        out
    }

    /// appends node to graph's node storage
    fn push_node(&mut self, value: T) -> usize;

    /// sets node at `node_index`
    fn set_node(&mut self, node_index: usize, value: T);

    /// returns given node's value
    fn get_node(&self, node_index: usize) -> &T;

    /// removes node from graph's node storage, removes dependant edges from graph.
    fn remove_node(&mut self, node_index: usize) -> T;

    fn node_count(&self) -> usize;

    // :)
    fn set_count(&mut self, count: usize);
}
