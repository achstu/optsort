use super::Graph;

impl Graph {
    /**
        // family of sets to be precise
        // TODO iterator over graphs not vec<vec<...>>
        fn connected_components(&self) -> Vec<Vec<usize>> {
            fn traverse(graph: &Graph, node: usize, c: i32, color: &mut [i32]) {
                color[node] = c;
                for neibour in graph.neighbors(node) {
                    if color[neibour] == -1 {
                        traverse(graph, neibour, c, color);
                    }
                }
            }

            let undirected = self.add(&self.transposed());
            let mut color = vec![-1; self.size()];
            let mut current = 0;
            for node in self.nodes() {
                if color[node] == -1 {
                    traverse(&undirected, node, current, &mut color);
                    current += 1;
                }
            }

            let mut result = Vec::new();
            for c in 0..current {
                let (nodes, _): (Vec<_>, Vec<_>) = color
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|(_, node_color)| *node_color == c)
                    .unzip();
                result.push(nodes);
            }
            result
        }
    */

    pub fn connected_components(&self) -> impl Iterator<Item = Graph> + '_ {
        let mut vis = vec![false; self.size()];
        pub fn dfs(node: usize) {
            vis[node] = true;
        }
        for node in self.nodes() {}
    }
}
