use crate::{graph::Graph, permutation::Permutation};
// use itertools::{sorted, Itertools};

// reduced DAG
#[derive(Clone)]
pub struct PartialOrder {
    graph: Graph,
}
impl PartialEq for PartialOrder {
    fn eq(&self, _other: &Self) -> bool {
        false
        // self.smaller
        // .iter()
        // .zip(other.smaller.iter())
        // .all(|(a, b)| sorted(a).eq(sorted(b)))
    }
}

// O(n! * n^2)
pub fn isomorphism(p: &PartialOrder, q: &PartialOrder) -> Option<Permutation> {
    if p.size() != q.size() {
        return None;
    }
    let n = p.size();
    for perm in Permutation::all(n) {
        if p.with_permutation(&perm).eq(q) {
            return Some(perm);
        }
    }
    None
}

pub fn isomorphic(p: &PartialOrder, q: &PartialOrder) -> bool {
    isomorphism(p, q).is_some()
}

impl PartialOrder {
    pub fn empty(n: usize) -> Self {
        PartialOrder {
            graph: Graph::empty(n),
        }
    }

    pub fn size(&self) -> usize {
        self.graph.size()
    }

    // TODO: remove in production
    pub fn with_comparison(&self, smaller: usize, larger: usize) -> Self {
        self.with_comparison_opt(smaller, larger).unwrap()
    }

    pub fn with_comparison_opt(&self, smaller: usize, larger: usize) -> Option<Self> {
        if self.is_comparable(smaller, larger) {
            return None;
        }
        let mut result = self.clone();
        result.graph.add_edge(smaller, larger);
        Some(result)
    }

    pub fn with_permutation(&self, perm: &Permutation) -> Self {
        PartialOrder {
            graph: self.graph.relabeled(perm),
        }
    }

    pub fn is_before(&self, x: usize, y: usize) -> bool {
        self.graph.is_reachable(x, y)
    }

    pub fn is_comparable(&self, x: usize, y: usize) -> bool {
        self.is_before(x, y) || self.is_before(y, x)
    }

    pub fn to_string(&self) -> String {
        self.graph.to_string()
    }
}

/*
#[cfg(test)]
mod test {
    use super::*;
    // use crate::utils::apply_permutation;

    #[test]
    fn test0() {
        let p = PartialOrder::empty(4)
            .with_edge(0, 1)
            .with_edge(0, 2)
            .with_edge(0, 3)
            .with_edge(1, 3)
            .with_edge(3, 2);
        let perm = Permutation::new(vec![2, 1, 3, 0]);
        println!("{}", p.to_string());
        println!();
        println!("{}", p.with_permutation(&perm).to_string());
        // assert!(p == p.with_permutation(&perm));
    }

    #[test]
    fn test1() {
        let p = PartialOrder::empty(4)
            .with_edge(0, 1)
            .with_edge(0, 2)
            .with_edge(0, 3)
            .with_edge(1, 3)
            .with_edge(3, 2);

        let q = PartialOrder::empty(4)
            .with_edge(0, 1)
            .with_edge(0, 3)
            .with_edge(0, 2)
            .with_edge(1, 3)
            .with_edge(3, 2);

        println!("{}", p.to_string());
        println!();
        println!("{}", q.to_string());
        assert!(p == q);
    }
}
 */
