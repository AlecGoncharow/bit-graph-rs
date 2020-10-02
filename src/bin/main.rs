use bit_graph::{BitGraph, Graph};

pub fn main() {
    let mut graph = BitGraph::with_capacity(16);

    for i in 0..15 {
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
