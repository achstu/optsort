// use std::cmp::max;

use itertools::Itertools;

#[derive(Clone)]
pub struct Graph {
    pub adj: Vec<Vec<usize>>, // adjacency list
}

// _approx
// _heuristic
// _heur

// _exact
// _exhaustive
// _precise
// _complete

impl Graph {
    pub fn empty(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
        }
    }

    pub fn size(&self) -> usize {
        self.adj.len()
    }

    pub fn nodes(&self) -> impl Iterator<Item = usize> {
        0..self.size()
    }

    pub fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.adj.iter().enumerate().flat_map(|(node, neighbors)| {
            std::iter::repeat(node)
                .take(neighbors.len())
                .zip(neighbors.iter().copied())
        })
    }

    pub fn neighbors(&self, node: usize) -> impl Iterator<Item = usize> + '_ {
        self.adj[node].iter().copied()
    }

    pub fn is_reachable(&self, src: usize, dst: usize) -> bool {
        self.neighbors(src)
            .any(|neighbor| neighbor == dst || self.is_reachable(neighbor, dst))
    }

    pub fn add_edge(&mut self, src: usize, dst: usize) {
        self.adj[src].push(dst);
    }

    pub fn transposed(&self) -> Self {
        let mut transpose = Self::empty(self.size());
        self.edges()
            .for_each(|(src, dst)| transpose.add_edge(dst, src));
        transpose
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut sum = Self::empty(std::cmp::max(self.size(), other.size()));
        for (src, dst) in self.edges().chain(other.edges()) {
            sum.add_edge(src, dst);
        }
        sum
    }

    pub fn to_string(&self) -> String {
        self.nodes()
            .map(|node| {
                format!(
                    "{}: {}",
                    node.to_string(),
                    self.neighbors(node)
                        .map(|neighbor| neighbor.to_string())
                        .join(", ")
                )
            })
            .join("\n")
    }
}
