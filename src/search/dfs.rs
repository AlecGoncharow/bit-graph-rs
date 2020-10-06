const WORD_BYTES: usize = std::mem::size_of::<usize>();
const WORD_BITS: usize = WORD_BYTES * 8;

use crate::bit::single_bit_mask;
use crate::Graph;

pub struct DFS {
    root_idx: usize,

    stack: Vec<(usize, usize)>,

    discovered: Vec<usize>,
}

impl DFS {
    pub fn new<V, W>(graph: &dyn Graph<V, W>, root_idx: usize) -> Self {
        Self {
            root_idx,
            discovered: vec![0; graph.node_count() / WORD_BITS + 1],
            stack: vec![(root_idx, root_idx)],
        }
    }

    pub fn next<V, W>(&mut self, graph: &dyn Graph<V, W>) -> Option<(usize, usize)> {
        while let Some((idx, from)) = self.stack.pop() {
            if self.visit_node(idx) {
                for out in graph.outgoing_edges_of(idx) {
                    if !self.is_discovered(out) {
                        self.stack.push((out, idx));
                    }
                }

                return Some((idx, from));
            }
        }
        None
    }

    pub fn path_to<V, W>(&mut self, to_idx: usize, graph: &dyn Graph<V, W>) -> Option<Vec<usize>> {
        let mut from_map = vec![std::usize::MAX; graph.node_count()];
        let mut out = Vec::new();

        while let Some((idx, from)) = self.stack.pop() {
            if idx == to_idx {
                let mut from_tmp = from;
                out.push(idx);
                out.push(from);
                loop {
                    if from_tmp == self.root_idx {
                        break;
                    }

                    let from_idx = from_map[from_tmp];
                    out.push(from_idx);

                    from_tmp = from_idx;
                }

                break;
            }

            if self.visit_node(idx) {
                from_map[idx] = from;

                for out in graph.outgoing_edges_of(idx) {
                    if !self.is_discovered(out) {
                        self.stack.push((out, idx));
                    }
                }
            }
        }

        if out.len() == 0 {
            None
        } else {
            out.reverse();
            Some(out)
        }
    }

    /// Visits node, returns true if first visit, else false
    fn visit_node(&mut self, node_idx: usize) -> bool {
        let first_visit = self.is_discovered(node_idx);

        if first_visit {
            false
        } else {
            self.set_discovered(node_idx);
            true
        }
    }

    fn is_discovered(&self, node_idx: usize) -> bool {
        (self.discovered[node_idx / WORD_BITS] & single_bit_mask(node_idx % WORD_BITS)) == 1
    }

    fn set_discovered(&mut self, node_idx: usize) {
        self.discovered[node_idx / WORD_BITS] |= single_bit_mask(node_idx % WORD_BITS);
    }
}

#[cfg(test)]
mod test_dfs {
    use super::*;
    use crate::bit::BitGraph;

    #[test]
    fn it_works() {
        let mut graph = BitGraph::new();

        for i in 0..15 {
            graph.push_node(i);
        }

        graph.add_edge(0, 2);
        graph.add_edge(0, 1);
        graph.add_edge(2, 4);
        graph.add_edge(3, 8);
        graph.add_edge(8, 5);
        graph.add_edge(1, 3);
        graph.add_edge(3, 5);
        graph.add_edge(5, 0);

        let mut dfs = DFS::new(&graph, 0);
        let found = loop {
            if let Some((idx, from)) = dfs.next(&graph) {
                if idx == 5 {
                    assert_eq!(from, 8);
                    break true;
                }
            } else {
                break false;
            }
        };

        dfs = DFS::new(&graph, 0);
        let path = dfs.path_to(5, &graph).unwrap();

        assert!(found);
        assert!(path.len() == 5);

        let mut dfs = DFS::new(&graph, 0);
        let not_found = loop {
            if let Some((idx, _from)) = dfs.next(&graph) {
                if idx == 10 {
                    break false;
                }
            } else {
                break true;
            }
        };
        assert!(not_found);
    }
}
