const DEFAULT_CAPACITY: usize = 16;

pub struct AdjGraph {
    count: usize,

    nodes: Vec<u64>,
    edges: Vec<bool>,
    edges_transpose: Vec<bool>,
}

impl AdjGraph {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(size: usize) -> Self {
        Self {
            count: 0,

            nodes: Vec::with_capacity(size),
            edges: vec![false; size * size],
            edges_transpose: vec![false; size * size],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        self.set_edge(from, to, true)
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> bool {
        self.set_edge(from, to, false)
    }

    fn set_edge(&mut self, from: usize, to: usize, val: bool) -> bool {
        // get proper word
        let row = self.nodes.capacity() * from;
        let column = to;

        let prev = self.edges[row + column];

        self.edges[row + column] = val;

        self.set_edge_of_tranpose(to, from, val);

        prev
    }

    pub fn get_edge(&mut self, from: usize, to: usize) -> bool {
        // get proper word
        let row = self.nodes.capacity() * from;
        let column = to;

        self.edges[row + column]
    }

    fn set_edge_of_tranpose(&mut self, from: usize, to: usize, val: bool) {
        // get proper word
        let row = self.nodes.capacity() * from;
        let column = to;

        self.edges_transpose[row + column] = val;
    }

    pub fn outgoing_edges_of(&self, node_index: usize) -> Vec<usize> {
        let index = self.nodes.capacity() * node_index;

        let mut out = Vec::new();
        for i in 0..self.count {
            if self.edges[index + i] {
                out.push(i);
            }
        }

        out
    }

    pub fn incoming_edges_of(&self, node_index: usize) -> Vec<usize> {
        let index = self.nodes.capacity() * node_index;

        let mut out = Vec::new();
        for i in 0..self.count {
            if self.edges_transpose[index + i] {
                out.push(i);
            }
        }

        out
    }

    pub fn push_node(&mut self, value: u64) {
        self.count += 1;
        self.nodes.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::AdjGraph;

    #[test]
    fn it_works() {
        let mut graph = AdjGraph::new();

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
        let mut graph = AdjGraph::new();

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
        let mut graph = AdjGraph::with_capacity(521);

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
        let mut graph = AdjGraph::with_capacity(100_000);

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
        let mut graph = AdjGraph::with_capacity(100_000);

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
        let mut graph = AdjGraph::new();

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