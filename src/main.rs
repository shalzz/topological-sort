use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Graph {
    pub nodes: Vec<u32>,
    pub edges: HashMap<u32, Vec<u32>>,
}

impl Graph {
    fn sort(&self, node: u32, visited: &mut HashSet<u32>, result: &mut Vec<u32>) {
        visited.insert(node);
        result.push(node);

        for dep in &self.edges[&node] {
            if !visited.contains(dep) {
                self.sort(*dep, visited, result)
            }
        }
    }
}

fn main() {
    let nodes = vec![5, 7, 3, 8, 11, 2, 9, 10];
    let mut edges = HashMap::new();
    edges.insert(5, vec![11]);
    edges.insert(7, vec![11, 8]);
    edges.insert(3, vec![10, 8]);
    edges.insert(11, vec![2, 9, 10]);
    edges.insert(8, vec![9]);
    edges.insert(2, vec![]);
    edges.insert(9, vec![]);
    edges.insert(10, vec![]);

    let mut visited = HashSet::new();
    let mut result = Vec::with_capacity(nodes.len());
    let graph = Graph { nodes, edges };

    // Make sure we traverse all nodes;
    for node in &graph.nodes {
        // don't re-visit a node.
        if !visited.contains(node) {
            graph.sort(*node, &mut visited, &mut result);
        }
    }

    for name in result {
        print!("{}, ", name);
    }
}