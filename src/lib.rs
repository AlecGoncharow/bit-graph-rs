const WORD_BYTES: usize = std::mem::size_of::<usize>();
const WORD_BITS: usize = WORD_BYTES * 8;
const DEFAULT_CAPACITY: usize = 16;

pub struct Graph {
    count: usize,

    nodes: Vec<u64>,
    ///
    /// Adjacency Matrix where to rows represent out from nodes and columns represent to nodes
    /// Encoded as a 1D array of Bits. 1 represents existance of edge, 0 no edge.
    edges: Vec<usize>,

    edges_transpose: Vec<usize>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            count: 0,

            nodes: Vec::with_capacity(DEFAULT_CAPACITY),
            edges: vec![0; (DEFAULT_CAPACITY * DEFAULT_CAPACITY) / WORD_BITS + 1],
            edges_transpose: vec![0; (DEFAULT_CAPACITY * DEFAULT_CAPACITY) / WORD_BITS + 1],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        self.set_edge(from, to, set_bit)
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> bool {
        self.set_edge(from, to, unset_bit)
    }

    fn set_edge<F>(&mut self, from: usize, to: usize, fun: F) -> bool
    where
        F: Fn(usize, usize) -> usize,
    {
        // get proper word
        let row = (self.nodes.capacity() * from) / WORD_BITS;
        let column = to / WORD_BITS;
        let offset = to % WORD_BITS + ((self.nodes.capacity() * from) % WORD_BITS);

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
        let column = to / WORD_BITS;
        let offset = to % WORD_BITS + ((self.nodes.capacity() * from) % WORD_BITS);

        let word = self.edges_transpose[row + column];

        let new_word = fun(word, offset);

        self.edges_transpose[row + column] = new_word;
    }

    pub fn get_edge(&self, from: usize, to: usize) -> bool {
        let row = (self.nodes.capacity() * from) / WORD_BITS;
        let column = to / WORD_BITS;
        let offset = to % WORD_BITS + ((self.nodes.capacity() * from) % WORD_BITS);

        let word = self.edges[row + column];

        get_bit(word, offset)
    }

    pub fn outgoing_edges_of(&self, node_index: usize) -> Vec<usize> {
        let mut index = (self.nodes.capacity() * node_index) / WORD_BITS;
        let mut offset = (self.nodes.capacity() * node_index) % WORD_BITS;

        let mut word = self.edges[index];
        let mut out = Vec::new();
        for i in 0..=self.count {
            if get_bit(word, offset) {
                out.push(i);
            }

            offset += 1;

            if offset == WORD_BITS - 1 {
                word = self.edges[index + 1];
                index += 1;
                offset = 0;
            }
        }

        out
    }

    pub fn incoming_edges_of(&self, node_index: usize) -> Vec<usize> {
        let mut index = (self.nodes.capacity() * node_index) / WORD_BITS;
        let mut offset = (self.nodes.capacity() * node_index) % WORD_BITS;

        let mut word = self.edges_transpose[index];
        let mut out = Vec::new();
        for i in 0..=self.count {
            if get_bit(word, offset) {
                out.push(i);
            }

            offset += 1;

            if offset == WORD_BITS - 1 {
                word = self.edges_transpose[index + 1];
                index += 1;
                offset = 0;
            }
        }

        out
    }

    pub fn push_node(&mut self, value: u64) {
        self.count += 1;
        self.nodes.push(value);
    }
}

fn get_bit(n: usize, k: usize) -> bool {
    if (n >> k) & 1 == 0 {
        false
    } else {
        true
    }
}

pub fn set_bit(n: usize, k: usize) -> usize {
    n | (1 << k)
}

pub fn unset_bit(n: usize, k: usize) -> usize {
    n & !(1 << k)
}

pub fn toggle_bit(n: usize, k: usize) -> usize {
    n ^ (1 << k)
}

mod tests {
    #[allow(unused_imports)]
    use crate::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new();

        for i in 1..16 {
            graph.push_node(i);
        }

        graph.add_edge(0, 1);

        graph.add_edge(2, 0);

        assert!(graph.get_edge(2, 0));
        assert!(!graph.get_edge(4, 3));
    }

    #[test]
    fn outgoing_edges_test() {
        let mut graph = Graph::new();

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

        graph.add_edge(10, 5);
        graph.add_edge(10, 5);
        graph.add_edge(10, 5);
        assert!(graph.outgoing_edges_of(10).len() == 8);
    }

    #[test]
    fn incoming_edges_test() {
        let mut graph = Graph::new();

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
    }
}
