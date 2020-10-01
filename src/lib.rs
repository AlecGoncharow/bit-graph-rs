pub mod baseline;
pub mod bit;
pub mod hash;

pub use baseline::AdjGraph;
pub use bit::Graph;

pub struct EdgeMeta {
    pub source: usize,
    pub destination: usize,
}

impl EdgeMeta {
    #[inline]
    pub fn key_pair(&self) -> (usize, usize) {
        (self.source, self.destination)
    }
}
