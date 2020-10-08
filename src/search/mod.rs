pub mod a_star;
pub mod bfs;
pub mod dfs;

use crate::Graph;

pub trait Pathfinder<V, W> {
    fn next(&mut self, graph: &dyn Graph<V, W>) -> Option<(usize, usize)>;
    fn path_to(&mut self, graph: &dyn Graph<V, W>, to_idx: usize) -> Option<Vec<usize>>;
    fn is_solved(&self) -> bool;
    fn set_solved(&mut self);
    fn from_index_of(&self, index: usize) -> usize;
}
