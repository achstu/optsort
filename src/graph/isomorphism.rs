use itertools::Itertools;
use std::collections::HashMap;

use super::Graph;
use crate::permutation::Permutation;

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        std::iter::zip(self.adj.iter(), other.adj.iter())
            .all(|(a, b)| itertools::sorted(a).eq(itertools::sorted(b)))
    }
}

impl Graph {
    pub fn relabeled(&self, perm: &Permutation) -> Self {
        let mut result = self.clone();
        perm.apply(&result.adj);
        result
            .adj
            .iter_mut()
            .flatten()
            .for_each(|node| *node = perm.maps(*node));
        result
    }

    pub fn isomorphism_exhaustive(&self, other: &Self) -> Option<Permutation> {
        Permutation::all(self.size()).find(|perm| &self.relabeled(perm) == other)
    }

    fn color_refinement(&self) -> Vec<usize> {
        // let mut coloring = vec![0; self.size()];
        let mut coloring: Vec<usize> = self.nodes().collect();
        return loop {
            // generate coloring
            let mut new_colors = vec![Vec::new(); self.size()];
            for node in self.nodes() {
                let new_color: Vec<usize> = self
                    .neighbors(node)
                    .map(|neighbor| coloring[neighbor])
                    .sorted()
                    .chain(std::iter::once(coloring[node]))
                    .collect();
                new_colors[node] = new_color;
            }
            println!("new_colors: {:?}", new_colors);
            // normalize coloring
            let new_coloring: Vec<usize> = {
                let mut mapping = HashMap::new();
                new_colors
                    .into_iter()
                    .map(|color| {
                        let index = mapping.len();
                        *mapping.entry(color).or_insert(index)
                    })
                    .collect()
            };
            println!("normalized: {:?}", new_coloring);

            // check if it is stable
            if coloring == new_coloring {
                break coloring;
            }
            coloring = new_coloring;
        };
    }

    fn degree_sequence(&self) -> Vec<usize> {
        itertools::sorted(self.adj.iter().map(|neighbors| neighbors.len())).collect()
    }

    pub fn isomorphism(&self, other: &Self) -> Option<Permutation> {
        if self.degree_sequence() != other.degree_sequence() {
            return None;
        }
        self.isomorphism_exhaustive(other)
    }
}

#[cfg(test)]
mod test {
    use crate::graph::Graph;

    #[test]
    fn isomorphism0() {
        let g = {
            let mut graph = Graph::empty(3);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph
        };

        let h = {
            let mut graph = Graph::empty(3);
            graph.add_edge(1, 0);
            graph.add_edge(1, 2);

            graph
        };

        println!("g -> {:?}", g.color_refinement());
        println!("h -> {:?}", h.color_refinement());
    }

    #[test]
    fn isomorphism1() {
        let g = {
            let mut graph = Graph::empty(4);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(0, 3);

            graph.add_edge(3, 1);
            graph.add_edge(3, 2);

            graph
        };

        let h = {
            let mut graph = Graph::empty(4);
            graph.add_edge(3, 0);
            graph.add_edge(3, 1);
            graph.add_edge(3, 2);

            graph.add_edge(1, 0);
            graph.add_edge(1, 2);

            graph
        };

        println!("g -> {:?}", g.color_refinement());
        println!("h -> {:?}", h.color_refinement());
    }
}
