use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Eq)]
struct Node {
    id: u32,

    // Meta Data
    // should not be used to compare or hash.
    name: String,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Node {
    fn new(id: u32) -> Node {
        Node {
            id,
            name: id.to_string(),
        }
    }
}

#[derive(Debug)]
struct Graph<'a> {
    pub nodes: Vec<&'a Node>,
    pub edges: HashMap<&'a Node, Vec<&'a Node>>,
}

impl<'a> Graph<'a> {
    fn new(nodes: Vec<&'a Node>, edges: HashMap<&'a Node, Vec<&'a Node>>) -> Graph<'a> {
        Graph { nodes, edges }
    }

    fn sort(&self) -> Vec<&Node> {
        let mut visited = HashSet::new();
        let mut result = Vec::with_capacity(self.nodes.len());

        // Make sure we traverse all nodes;
        for node in &self.nodes {
            // don't re-visit a node.
            if !visited.contains(node) {
                let mut stack = Vec::new();
                stack.push((*node, false));

                while !stack.is_empty() {
                    let (cur_node, is_parent) = stack.pop().unwrap();

                    // if we get back a parent node then we have
                    // already visited all child nodes.
                    if is_parent {
                        result.push(cur_node);
                        continue;
                    }

                    visited.insert(cur_node);
                    stack.push((cur_node, true));

                    if self.edges.get(&cur_node).is_none() {
                        continue;
                    }

                    for dep in &self.edges[&cur_node] {
                        // If a node's already on the stack then it's a cyclic graph
                        if stack.iter().any(|(node, _)| node == dep) {
                            panic!("Should not be a cyclic graph, cycle at node: {}", dep.id);
                        }

                        if !visited.contains(dep) {
                            stack.push((*dep, false));
                        }
                    }
                }
            }
        }

        result
    }
}

fn main() {
    let (_5, _7, _3, _8, _11, _2, _9, _10) = (
        Node::new(5),
        Node::new(7),
        Node::new(3),
        Node::new(8),
        Node::new(11),
        Node::new(2),
        Node::new(9),
        Node::new(10),
    );
    let nodes = vec![&_5, &_7, &_3, &_8, &_11, &_2, &_9, &_10];
    let mut edges = HashMap::new();
    edges.insert(&_5, vec![&_11]);
    edges.insert(&_7, vec![&_11, &_8]);
    edges.insert(&_3, vec![&_10, &_8]);
    edges.insert(&_11, vec![&_2, &_9, &_10]);
    edges.insert(&_8, vec![&_9]);
    let graph = Graph::new(nodes, edges);

    let result = graph.sort();

    for name in result.iter().rev() {
        print!("{}, ", name.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sorts_correctly_1() {
        let a = Node::new(1);
        let b = Node::new(2);
        let nodes = vec![&a, &b];
        let mut edges = HashMap::new();
        edges.insert(&a, vec![&b]);
        let graph = Graph::new(nodes, edges);
        let mut res = graph.sort();
        res.reverse();

        assert_eq!(res[0].id, 1);
        assert_eq!(res[1].id, 2);
    }

    #[test]
    fn it_sorts_correctly_2() {
        let a = Node::new(1);
        let b = Node::new(2);
        let c = Node::new(3);
        let nodes = vec![&a, &b, &c];
        let mut edges = HashMap::new();
        edges.insert(&a, vec![&b]);
        edges.insert(&b, vec![&c]);
        let graph = Graph::new(nodes, edges);
        let mut res = graph.sort();
        res.reverse();

        assert_eq!(res[0].id, 1);
        assert_eq!(res[1].id, 2);
        assert_eq!(res[2].id, 3);
    }

    #[test]
    #[should_panic(expected = "Should not be a cyclic graph")]
    fn detects_cycle() {
        let a = Node::new(1);
        let b = Node::new(2);
        let c = Node::new(3);
        let nodes = vec![&a, &b, &c];
        let mut edges = HashMap::new();
        edges.insert(&a, vec![&b]);
        edges.insert(&b, vec![&c]);
        edges.insert(&c, vec![&a]);
        let graph = Graph::new(nodes, edges);
        let mut res = graph.sort();
        res.reverse();

        assert_eq!(res[0].id, 1);
        assert_eq!(res[1].id, 2);
        assert_eq!(res[2].id, 3);
    }
}
