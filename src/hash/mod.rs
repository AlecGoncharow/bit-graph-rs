use crate::{EdgeMeta, Graph};
use std::num::Wrapping;

const DEFAULT_CAPACITY: usize = 256;
const MAX_LOAD: f32 = 0.75;
const GROW_FACTOR: usize = 2;
const PRIME_OF_MATHS: Wrapping<usize> = Wrapping(97);

pub struct PairHashTable {
    count: usize,

    table: Vec<Option<Entry>>,
}

struct Entry {
    edge_meta: EdgeMeta<usize>,
    is_deleted: bool,
}

/// Source Index, Destination Index
type IndexPair = (usize, usize);

// http://web.archive.org/web/20071223173210/http://www.concentric.net/~Ttwang/tech/inthash.htm
// 64 bit shift/mix
#[inline(always)]
fn hash_usize(input: usize) -> Wrapping<usize> {
    let mut key = Wrapping(input);
    key = (!key) + (key << 21); // key = (key << 21) - key - 1;
    key = key ^ (key >> 24);
    key = (key + (key << 3)) + (key << 8); // key * 265
    key = key ^ (key >> 14);
    key = (key + (key << 2)) + (key << 4); // key * 21
    key = key ^ (key >> 28);
    key + (key << 31)
}

impl PairHashTable {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            count: 0,
            table: std::iter::repeat_with(|| None).take(capacity).collect(),
        }
    }

    fn index_of(&self, key: IndexPair) -> usize {
        let mut index = self.index_calc(key);

        loop {
            match &self.table[index] {
                Some(entry) => {
                    if !entry.is_deleted && entry.edge_meta.key_pair() == key {
                        return index;
                    }

                    index = (index + 1) % self.table.capacity();
                }
                None => return index,
            }
        }
    }

    fn index_of_insertion(&self, key: IndexPair) -> usize {
        let mut index = self.index_calc(key);
        let mut tombstone = None;

        loop {
            match &self.table[index] {
                Some(entry) => {
                    if !entry.is_deleted && entry.edge_meta.key_pair() == key {
                        return index;
                    }

                    // if found a tombstone, keep track of index
                    if entry.is_deleted && tombstone.is_none() {
                        tombstone = Some(index);
                    }

                    index = (index + 1) % self.table.capacity();
                }
                None => {
                    if let Some(tombstone) = tombstone {
                        return tombstone;
                    } else {
                        return index;
                    }
                }
            }
        }
    }

    fn insert(&mut self, key: IndexPair, weight: usize) -> bool {
        if self.count + 1 > (self.table.capacity() as f32 * MAX_LOAD) as usize {
            let new_capacity = self.table.capacity() * GROW_FACTOR;
            self.resize(new_capacity);
        }

        let index = self.index_of_insertion(key);

        let had_edge = if let Some(_) = self.table[index] {
            true
        } else {
            false
        };

        if !had_edge {
            self.count += 1;
        }

        self.table[index] = Some(Entry {
            is_deleted: false,

            edge_meta: EdgeMeta {
                source: key.0,
                destination: key.1,
                weight,
            },
        });

        had_edge
    }

    fn delete(&mut self, key: IndexPair) -> bool {
        if self.count == 0 {
            return false;
        }

        let index = self.index_of_insertion(key);

        let index = if let Some(entry) = &self.table[index] {
            if entry.is_deleted {
                None
            } else {
                Some(index)
            }
        } else {
            None
        };

        if let Some(index) = index {
            let mut entry = self.table[index].take().unwrap();
            entry.is_deleted = true;
            self.table[index] = Some(entry);
            true
        } else {
            false
        }
    }

    fn get(&self, key: IndexPair) -> Option<&EdgeMeta<usize>> {
        if let Some(entry) = &self.table[self.index_of(key)] {
            Some(&entry.edge_meta)
        } else {
            None
        }
    }

    fn resize(&mut self, capacity: usize) {
        let mut new_table = Self::with_capacity(capacity);

        for i in 0..self.table.capacity() {
            if let Some(entry) = &self.table[i] {
                if !entry.is_deleted {
                    new_table.insert(entry.edge_meta.key_pair(), entry.edge_meta.weight);
                }
            }
        }

        self.count = new_table.count;
        self.table = new_table.table;
    }

    /// helper
    #[inline(always)]
    fn index_calc(&self, key: IndexPair) -> usize {
        (PRIME_OF_MATHS * hash_usize(key.0) + PRIME_OF_MATHS + hash_usize(key.1)).0
            % self.table.capacity()
    }
}

pub struct HashGraph {
    count: usize,
    nodes: Vec<u64>,

    edges: PairHashTable,
}

impl HashGraph {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(size: usize) -> Self {
        Self {
            count: 0,

            nodes: Vec::with_capacity(size),
            edges: PairHashTable::with_capacity(size),
        }
    }
}

impl Graph<u64, usize> for HashGraph {
    fn add_edge(&mut self, from: usize, to: usize) -> bool {
        self.edges.insert((from, to), 1)
    }

    fn remove_edge(&mut self, from: usize, to: usize) -> bool {
        self.edges.delete((from, to))
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.edges.get((from, to)).is_some()
    }

    fn get_edge(&self, from: usize, to: usize) -> Option<EdgeMeta<usize>> {
        if let Some(edge) = self.edges.get((from, to)) {
            Some(*edge)
        } else {
            None
        }
    }

    fn outgoing_edges_of(&self, node_index: usize) -> Vec<usize> {
        let mut out = Vec::new();

        for i in 0..self.count {
            if self.edges.get((node_index, i)).is_some() {
                out.push(i);
            }
        }

        out
    }

    fn incoming_edges_of(&self, node_index: usize) -> Vec<usize> {
        let mut out = Vec::new();

        for i in 0..self.count {
            if self.edges.get((i, node_index)).is_some() {
                out.push(i);
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
        todo!();
    }

    fn get_node(&self, node_index: usize) -> &u64 {
        // @FIXME
        &self.nodes[node_index]
    }

    fn remove_node(&mut self, node_index: usize) -> u64 {
        for i in 0..self.count {
            self.remove_edge(i, node_index);
            self.remove_edge(node_index, i);
        }

        let val = self.nodes[node_index];

        self.nodes[node_index] = 0;

        val
    }

    fn node_count(&self) -> usize {
        self.count
    }

    fn set_count(&mut self, count: usize) {
        self.count = count;
    }

    fn set_edge(&mut self, from_to: (usize, usize), weight: usize) -> bool {
        self.edges.insert(from_to, weight)
    }
}

#[cfg(test)]
mod test_hashtable {
    use super::*;

    #[test]
    fn it_works() {
        let mut table = PairHashTable::new();

        table.insert((1, 0), 1);
        table.insert((4, 2), 1);
        table.insert((6, 2), 1);
        table.insert((9, 10), 1);
        table.insert((4, 23), 1);

        assert_eq!(table.count, 5);

        assert!(table.insert((1, 0), 1));
        assert!(table.get((9, 10)).is_some());
        assert!(table.get((9, 3)).is_none());
    }
}

#[cfg(test)]
mod test_hashgraph {
    use super::*;

    #[test]
    fn it_works() {
        let mut graph = HashGraph::new();

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
        let mut graph = HashGraph::new();

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
        let mut graph = HashGraph::with_capacity(521);

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
        let mut graph = HashGraph::with_capacity(100_000);

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
        let mut graph = HashGraph::with_capacity(100_000);

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
        let mut graph = HashGraph::new();

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
