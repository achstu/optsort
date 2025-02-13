use rug::Complete;
use std::collections::HashSet;

use super::Graph;

impl Graph {
    pub fn toposort(&self) -> Vec<usize> {
        let n = self.size();
        let mut indeg = vec![0; n];
        for (_src, dst) in self.edges() {
            indeg[dst] += 1;
        }

        let mut stack = Vec::new();
        for node in self.nodes() {
            if indeg[node] == 0 {
                stack.push(node);
            }
        }

        let mut order = Vec::new();
        order.reserve(n);

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            order.push(node);

            for neighbor in self.neighbors(node) {
                indeg[neighbor] -= 1;
                if indeg[neighbor] == 0 {
                    stack.push(neighbor);
                }
            }
        }

        order
    }

    // will be apllied to connected component
    fn brute_toposort_count(&self, nodes: Vec<usize>) -> usize {
        let mut indeg = vec![0; self.size()];
        let node_set: HashSet<_> = nodes.into_iter().collect();

        for (_src, dst) in self.edges() {
            if node_set.contains(&dst) {
                indeg[dst] += 1;
            }
        }

        fn backtrack(graph: &Graph, roots: &Vec<usize>, indeg: &mut [i32]) -> usize {
            if roots.is_empty() {
                return 1;
            }

            let mut count = 0;
            for (index, &root) in roots.iter().enumerate() {
                let mut new_roots = roots.clone();
                for node in graph.neighbors(root) {
                    indeg[node] -= 1;
                    if indeg[node] == 0 {
                        new_roots.push(node);
                    }
                }

                new_roots.swap_remove(index);
                count += backtrack(graph, &new_roots, indeg);

                for node in graph.neighbors(root) {
                    indeg[node] += 1;
                }
            }
            count
        }

        let roots = self
            .nodes()
            .filter(|&node| node_set.contains(&node) && indeg[node] == 0)
            .collect();
        backtrack(self, &roots, &mut indeg)
    }

    pub fn toposort_count(&self) -> usize {
        // count orders for all components and merge results
        let (sizes, ways): (Vec<_>, Vec<_>) = self
            .connected_components()
            .into_iter()
            .map(|component| (component.len(), self.brute_toposort_count(component)))
            .unzip();

        println!("{:?}", self.connected_components());
        println!("{:?}", ways);

        // (a1 + ... + an)! / (a1! * ... * an!)
        fn calculate_expr(a: &[usize]) -> usize {
            let lhs = rug::Integer::factorial(a.iter().sum::<usize>() as u32).complete();
            let rhs = a
                .iter()
                .map(|&x| rug::Integer::factorial(x as u32).complete())
                .product::<rug::Integer>();
            (lhs / rhs).to_usize().unwrap()
        }

        ways.iter().product::<usize>() * calculate_expr(&sizes)
    }
}

/*
#[cfg(test)]
mod test {
    use crate::graph::Graph;

    #[test]
    fn graph0() {
        let mut graph = Graph::empty(8);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);

        graph.add_edge(4, 5);

        graph.add_edge(0, 7);
        graph.add_edge(7, 6);
        graph.add_edge(6, 0);

        println!("{:?}", graph.connected_components());
    }

    #[test]
    fn graph1() {
        let mut graph = Graph::empty(8);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);

        graph.add_edge(4, 5);

        graph.add_edge(0, 7);
        graph.add_edge(7, 6);

        println!("{:?}", graph.toposort());
    }

    #[test]
    fn graph2() {
        let mut graph = Graph::empty(8);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);

        graph.add_edge(4, 5);

        graph.add_edge(0, 7);
        graph.add_edge(7, 6);

        println!(
            "orders count: {}",
            graph.brute_toposort_count(vec![1, 2, 3])
        );
    }

    #[test]
    fn graph3() {
        let mut graph = Graph::empty(7);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(0, 4);
        graph.add_edge(0, 5);
        graph.add_edge(0, 6);

        println!(
            "orders count: {}",
            graph.brute_toposort_count(graph.nodes().collect())
        );
    }

    #[test]
    fn graph4() {
        let mut graph = Graph::empty(7);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(0, 4);
        graph.add_edge(0, 5);
        graph.add_edge(0, 6);

        println!("orders count: {}", graph.toposort_count());
    }

    #[test]
    fn graph5() {
        let mut graph = Graph::empty(8);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);

        graph.add_edge(4, 5);

        graph.add_edge(0, 7);
        graph.add_edge(7, 6);

        println!("orders count: {}", graph.toposort_count());
    }

    #[test]
    fn graph6() {
        let mut graph = Graph::empty(4);

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);

        println!("orders count: {}", graph.toposort_count());
    }
}
 */
