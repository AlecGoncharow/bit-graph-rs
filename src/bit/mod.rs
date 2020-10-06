const WORD_BYTES: usize = std::mem::size_of::<usize>();
const WORD_BITS: usize = WORD_BYTES * 8;
const DEFAULT_CAPACITY: usize = 16;

use crate::{EdgeMeta, Graph};

pub struct BitGraph {
    count: usize,

    nodes: Vec<u64>,
    ///
    /// Adjacency Matrix where to rows represent out from nodes and columns represent to nodes
    /// Encoded as a 1D array of Bits. 1 represents existance of edge, 0 no edge.
    edges: Vec<usize>,

    edges_transpose: Vec<usize>,
}

impl BitGraph {
    pub fn new() -> BitGraph {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(size: usize) -> BitGraph {
        BitGraph {
            count: 0,

            nodes: Vec::with_capacity(size),
            edges: vec![0; (size * size) / WORD_BITS + 1],
            edges_transpose: vec![0; (size * size) / WORD_BITS + 1],
        }
    }

    fn set_edge_of_both<F>(&mut self, from: usize, to: usize, fun: F) -> bool
    where
        F: Fn(usize, usize) -> usize,
    {
        // get proper word
        let row = (self.nodes.capacity() * from) / WORD_BITS;
        let mut column = to / WORD_BITS;
        let mut offset = to % WORD_BITS + ((self.nodes.capacity() * from) % WORD_BITS);

        if offset >= WORD_BITS {
            column += 1;
            offset -= WORD_BITS;
        }

        let word = self.edges[row + column];

        let new_word = fun(word, offset);

        self.edges[row + column] = new_word;

        self.set_edge_of_tranpose(to, from, fun);

        get_bit(word, offset)
    }

    fn set_edge_of_tranpose<F>(&mut self, from: usize, to: usize, fun: F)
    where
        F: FnOnce(usize, usize) -> usize,
    {
        // get proper word
        let row = (self.nodes.capacity() * from) / WORD_BITS;
        let mut column = to / WORD_BITS;
        let mut offset = to % WORD_BITS + ((self.nodes.capacity() * from) % WORD_BITS);

        if offset >= WORD_BITS {
            column += 1;
            offset -= WORD_BITS;
        }

        let word = self.edges_transpose[row + column];

        let new_word = fun(word, offset);

        self.edges_transpose[row + column] = new_word;
    }
}

/// makes a mask for a single bit of a given offset
pub fn single_bit_mask(offset: usize) -> usize {
    1 << offset
}

#[inline(always)]
fn get_bit(n: usize, k: usize) -> bool {
    if (n >> k) & 1 == 0 {
        false
    } else {
        true
    }
}

#[inline(always)]
pub fn set_bit(n: usize, k: usize) -> usize {
    n | (1 << k)
}

#[inline(always)]
pub fn unset_bit(n: usize, k: usize) -> usize {
    n & !(1 << k)
}

#[inline(always)]
pub fn toggle_bit(n: usize, k: usize) -> usize {
    n ^ (1 << k)
}

impl Graph<u64, bool> for BitGraph {
    fn add_edge(&mut self, from: usize, to: usize) -> bool {
        self.set_edge_of_both(from, to, set_bit)
    }

    fn remove_edge(&mut self, from: usize, to: usize) -> bool {
        self.set_edge_of_both(from, to, unset_bit)
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        let row = (self.nodes.capacity() * from) / WORD_BITS;
        let column = to / WORD_BITS;
        let offset = to % WORD_BITS + ((self.nodes.capacity() * from) % WORD_BITS);

        let word = self.edges[row + column];

        get_bit(word, offset)
    }

    fn outgoing_edges_of(&self, node_index: usize) -> Vec<usize> {
        let mut index = (self.nodes.capacity() * node_index) / WORD_BITS;
        let mut offset = (self.nodes.capacity() * node_index) % WORD_BITS;

        let mut word = self.edges[index];
        let mut out = Vec::new();
        let mut i = 0;
        loop {
            let shifted = word >> offset;
            let trailing_zeroes: usize = shifted.trailing_zeros() as usize;

            if i + trailing_zeroes >= self.count {
                break;
            }

            if (offset + trailing_zeroes) >= WORD_BITS - 1 {
                word = self.edges[index + 1];
                index += 1;
                offset = 0;
                i += trailing_zeroes;
            } else {
                out.push(i + trailing_zeroes);
                offset += trailing_zeroes + 1;
                i += trailing_zeroes + 1;
            }

            if offset == WORD_BITS - 1 {
                word = self.edges[index + 1];
                index += 1;
                offset = 0;
            }

            if i >= self.count - 1 {
                break;
            }
        }

        out
    }

    fn incoming_edges_of(&self, node_index: usize) -> Vec<usize> {
        let mut index = (self.nodes.capacity() * node_index) / WORD_BITS;
        let mut offset = (self.nodes.capacity() * node_index) % WORD_BITS;

        let mut word = self.edges_transpose[index];
        let mut out = Vec::new();
        let mut i = 0;
        loop {
            let shifted = word >> offset;
            let trailing_zeroes: usize = shifted.trailing_zeros() as usize;

            if i + trailing_zeroes >= self.count {
                break;
            }

            if (offset + trailing_zeroes) >= WORD_BITS - 1 {
                word = self.edges[index + 1];
                index += 1;
                offset = 0;
                i += trailing_zeroes;
            } else {
                out.push(i + trailing_zeroes);
                offset += trailing_zeroes + 1;
                i += trailing_zeroes + 1;
            }

            if offset == WORD_BITS - 1 {
                word = self.edges[index + 1];
                index += 1;
                offset = 0;
            }

            if i >= self.count - 1 {
                break;
            }
        }

        out
    }

    fn push_node(&mut self, value: u64) -> usize {
        self.count += 1;
        self.nodes.push(value);
        self.nodes.len() - 1
    }

    fn set_node(&mut self, _node_index: usize, _value: u64) {
        todo!()
    }

    fn get_node(&self, _node_index: usize) -> &u64 {
        todo!()
    }

    fn remove_node(&mut self, _node_index: usize) -> u64 {
        todo!()
    }

    fn get_edge(&self, from: usize, to: usize) -> Option<EdgeMeta<bool>> {
        let row = (self.nodes.capacity() * from) / WORD_BITS;
        let column = to / WORD_BITS;
        let offset = to % WORD_BITS + ((self.nodes.capacity() * from) % WORD_BITS);

        let word = self.edges[row + column];

        if get_bit(word, offset) {
            Some(EdgeMeta {
                source: from,
                destination: to,
                weight: true,
            })
        } else {
            None
        }
    }

    #[inline]
    fn node_count(&self) -> usize {
        self.count
    }

    fn set_edge(&mut self, from_to: (usize, usize), weight: bool) -> bool {
        if weight {
            self.add_edge(from_to.0, from_to.1)
        } else {
            self.remove_edge(from_to.0, from_to.1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut graph = BitGraph::new();

        for i in 1..16 {
            graph.push_node(i);
        }

        graph.add_edge(0, 1);

        graph.add_edge(2, 0);

        assert!(graph.has_edge(2, 0));
        assert!(!graph.has_edge(4, 3));
    }

    #[test]
    fn outgoing_edges_test() {
        let mut graph = BitGraph::new();

        for i in 1..16 {
            graph.push_node(i);
        }

        graph.add_edge(0, 1);

        graph.add_edge(2, 0);

        assert!(graph.outgoing_edges_of(0).len() == 1);
        assert!(graph.outgoing_edges_of(4).len() == 0);

        graph.add_edge(10, 2);
        graph.add_edge(10, 3);
        graph.add_edge(10, 4);
        graph.add_edge(10, 5);
        graph.add_edge(10, 6);
        graph.add_edge(10, 7);
        graph.add_edge(10, 8);
        graph.add_edge(10, 9);

        assert!(graph.outgoing_edges_of(10).len() == 8);
        assert_eq!(graph.outgoing_edges_of(10), vec![2, 3, 4, 5, 6, 7, 8, 9]);

        graph.add_edge(10, 5);
        graph.add_edge(10, 5);
        graph.add_edge(10, 5);
        assert!(graph.outgoing_edges_of(10).len() == 8);
        assert_eq!(graph.outgoing_edges_of(10), vec![2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn strange_outgoing_edges_test() {
        let mut graph = BitGraph::with_capacity(521);

        for i in 1..510 {
            graph.push_node(i);
        }

        graph.add_edge(500, 402);

        graph.add_edge(2, 0);

        assert!(graph.outgoing_edges_of(500).len() == 1);
        assert!(graph.outgoing_edges_of(4).len() == 0);

        graph.add_edge(10, 2);
        graph.add_edge(10, 3);
        graph.add_edge(10, 4);
        graph.add_edge(10, 5);
        graph.add_edge(10, 6);
        graph.add_edge(10, 7);
        graph.add_edge(10, 8);
        graph.add_edge(10, 9);

        assert!(graph.outgoing_edges_of(10).len() == 8);
        assert_eq!(graph.outgoing_edges_of(10), vec![2, 3, 4, 5, 6, 7, 8, 9]);

        graph.add_edge(10, 5);
        graph.add_edge(10, 5);
        graph.add_edge(10, 5);
        assert!(graph.outgoing_edges_of(10).len() == 8);
        assert_eq!(graph.outgoing_edges_of(10), vec![2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn large_outgoing_edges_test() {
        let mut graph = BitGraph::with_capacity(100_000);

        for i in 0..100_000 {
            graph.push_node(i);
        }

        graph.add_edge(500, 402);

        graph.add_edge(2, 0);

        assert!(graph.outgoing_edges_of(500).len() == 1);
        assert!(graph.outgoing_edges_of(4).len() == 0);

        graph.add_edge(10, 2);
        graph.add_edge(10, 3);
        graph.add_edge(10, 4);
        graph.add_edge(10, 5);
        graph.add_edge(10, 6);
        graph.add_edge(10, 7);
        graph.add_edge(10, 8);
        graph.add_edge(10, 9);

        assert!(graph.outgoing_edges_of(10).len() == 8);
        assert_eq!(graph.outgoing_edges_of(10), vec![2, 3, 4, 5, 6, 7, 8, 9]);

        graph.add_edge(10, 5);
        graph.add_edge(10, 5);
        graph.add_edge(10, 5);
        assert!(graph.outgoing_edges_of(10).len() == 8);
        assert_eq!(graph.outgoing_edges_of(10), vec![2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn large_incoming_edges_test() {
        let mut graph = BitGraph::with_capacity(100_000);

        for i in 0..100_000 {
            graph.push_node(i);
        }

        graph.add_edge(0, 1);

        graph.add_edge(2, 0);

        assert!(graph.incoming_edges_of(1).len() == 1);
        assert!(graph.incoming_edges_of(4).len() == 0);

        graph.add_edge(2, 1);
        graph.add_edge(3, 1);
        graph.add_edge(4, 1);
        graph.add_edge(5, 1);
        graph.add_edge(7, 1);

        assert!(graph.incoming_edges_of(1).len() == 6);
        assert_eq!(graph.incoming_edges_of(1), vec![0, 2, 3, 4, 5, 7]);
    }

    #[test]
    fn incoming_edges_test() {
        let mut graph = BitGraph::new();

        for i in 1..16 {
            graph.push_node(i);
        }

        graph.add_edge(0, 1);

        graph.add_edge(2, 0);

        assert!(graph.incoming_edges_of(1).len() == 1);
        assert!(graph.incoming_edges_of(4).len() == 0);

        graph.add_edge(2, 1);
        graph.add_edge(3, 1);
        graph.add_edge(4, 1);
        graph.add_edge(5, 1);
        graph.add_edge(7, 1);

        assert!(graph.incoming_edges_of(1).len() == 6);
        assert_eq!(graph.incoming_edges_of(1), vec![0, 2, 3, 4, 5, 7]);
    }
}
