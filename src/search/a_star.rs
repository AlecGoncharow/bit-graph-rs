use crate::search::Pathfinder;
use crate::Graph;
use std::collections::binary_heap::BinaryHeap;
#[derive(PartialEq, Eq, Debug)]
struct HeapNode {
    index: usize,
    score: usize,
}

impl std::cmp::Ord for HeapNode {
    /// flip order to make it a min heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}
// `PartialOrd` needs to be implemented as well.
impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A star using manhattan distance as heuristic
/// indicies are assumed to be an index into a 2D Array
pub struct AStarMH {
    root_idx: usize,
    goal_idx: usize,

    open_set: BinaryHeap<HeapNode>,

    g_score: Vec<usize>,
    f_score: Vec<usize>,

    /// dims of environment
    dim: usize,

    pub from_map: Vec<usize>,
    pub solved: bool,
}

impl AStarMH {
    pub fn new<V, W>(
        graph: &dyn Graph<V, W>,
        root_idx: usize,
        goal_idx: usize,
        dim: usize,
    ) -> Self {
        let mut g_score = vec![std::usize::MAX; graph.node_count()];
        g_score[root_idx] = 0;

        let mut f_score = vec![std::usize::MAX; graph.node_count()];
        f_score[root_idx] = mh_distance(root_idx, goal_idx, dim);

        let mut open_set = BinaryHeap::new();
        open_set.push(HeapNode {
            index: root_idx,
            score: f_score[root_idx],
        });

        Self {
            root_idx,
            goal_idx,
            open_set,

            g_score,
            f_score,
            dim,

            from_map: vec![std::usize::MAX; graph.node_count()],
            solved: false,
        }
    }
}

fn mh_distance(from: usize, to: usize, dim: usize) -> usize {
    let (from_x, from_y) = (from / dim, from % dim);
    let (to_x, to_y) = (to / dim, to % dim);

    let diff_x = if from_x < to_x {
        to_x - from_x
    } else {
        from_x - to_x
    };

    let diff_y = if from_y < to_y {
        to_y - from_y
    } else {
        from_y - to_y
    };

    diff_x + diff_y
}

impl<V, W> Pathfinder<V, W> for AStarMH {
    fn next(&mut self, graph: &dyn Graph<V, W>) -> Option<(usize, usize)> {
        let current = match self.open_set.pop() {
            Some(inner) => inner,
            None => return None,
        };

        for idx in graph.outgoing_edges_of(current.index) {
            let tenantive_g_score = self.g_score[current.index] + 1;
            if tenantive_g_score < self.g_score[idx] {
                self.from_map[idx] = current.index;
                self.g_score[idx] = tenantive_g_score;
                self.f_score[idx] = tenantive_g_score + mh_distance(idx, self.goal_idx, self.dim);
                let neighbor = HeapNode {
                    index: idx,
                    score: self.f_score[idx],
                };
                if !self
                    .open_set
                    .iter()
                    .any(|node| node.index == neighbor.index)
                {
                    self.open_set.push(neighbor);
                }
            }
        }

        Some((current.index, std::usize::MAX))
    }

    fn path_to(&mut self, graph: &dyn Graph<V, W>, _to_idx: usize) -> Option<Vec<usize>> {
        let mut out = Vec::new();

        loop {
            let current = self.open_set.pop().unwrap();

            if current.index == self.goal_idx {
                let mut from_tmp = current.index;
                out.push(current.index);
                loop {
                    if from_tmp == self.root_idx {
                        break;
                    }
                    let from_idx = self.from_map[from_tmp];
                    out.push(from_idx);

                    from_tmp = from_idx;
                }

                break;
            }

            for idx in graph.outgoing_edges_of(current.index) {
                let tenantive_g_score = self.g_score[current.index] + 1;
                if tenantive_g_score < self.g_score[idx] {
                    self.from_map[idx] = current.index;
                    self.g_score[idx] = tenantive_g_score;
                    self.f_score[idx] =
                        tenantive_g_score + mh_distance(idx, self.goal_idx, self.dim);
                    let neighbor = HeapNode {
                        index: idx,
                        score: self.f_score[idx],
                    };
                    if !self
                        .open_set
                        .iter()
                        .any(|node| node.index == neighbor.index)
                    {
                        self.open_set.push(neighbor);
                    }
                }
            }

            if self.open_set.len() == 0 {
                break;
            }
        }

        if out.len() == 0 {
            None
        } else {
            out.reverse();
            Some(out)
        }
    }

    fn is_solved(&self) -> bool {
        self.solved
    }

    fn set_solved(&mut self) {
        self.solved = true;
    }

    fn from_index_of(&self, index: usize) -> usize {
        self.from_map[index]
    }
}

#[cfg(test)]
mod test_dfs {
    use super::*;
    use crate::bit::BitGraph;

    #[test]
    fn it_works() {
        let mut graph = BitGraph::with_capacity(16);

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

        let mut astar = AStarMH::new(&graph, 0, 5, 16);
        let found = loop {
            if let Some((idx, _from)) = astar.next(&graph) {
                if idx == 5 {
                    break true;
                }
            } else {
                break false;
            }
        };

        let mut bfs = AStarMH::new(&graph, 0, 5, 16);
        let path = bfs.path_to(&graph, 5).unwrap();

        assert!(found);
        assert!(path.len() == 4);
        assert!(path == vec![0, 1, 3, 5]);

        let mut astar = AStarMH::new(&graph, 0, 10, 16);
        let not_found = loop {
            if let Some((idx, _from)) = astar.next(&graph) {
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
