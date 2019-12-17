use std::collections::HashSet;

#[derive(Debug)]
struct Node {
    name: String,
    dep: Vec<Node>,
}

impl Node {
    fn new(name: String, dep: Vec<Node>) -> Node {
        Node { name, dep }
    }
}

fn main() {
    let mut graph = Vec::default();
    graph.push(Node::new(
        "5".to_string(),
        vec![Node::new(
            "11".to_string(),
            vec![
                Node::new("2".to_string(), vec![]),
                Node::new("9".to_string(), vec![]),
                Node::new("10".to_string(), vec![]),
            ],
        )],
    ));
    graph.push(Node::new(
        "7".to_string(),
        vec![
            Node::new(
                "11".to_string(),
                vec![
                    Node::new("2".to_string(), vec![]),
                    Node::new("9".to_string(), vec![]),
                    Node::new("10".to_string(), vec![]),
                ],
            ),
            Node::new("8".to_string(), vec![Node::new("9".to_string(), vec![])]),
        ],
    ));
    graph.push(Node::new(
        "3".to_string(),
        vec![
            Node::new("10".to_string(), vec![]),
            Node::new("8".to_string(), vec![Node::new("9".to_string(), vec![])]),
        ],
    ));

    let node_names = sort(graph);

    for name in node_names {
        print!("{}, ", name);
    }
}

fn sort(graph: Vec<Node>) -> HashSet<String> {
    let mut result: HashSet<String> = HashSet::new();
    for node in graph {
        if node.dep.len() == 0 {
            result.insert(node.name);
        } else {
            for sub_node in node.dep {
                for name in sort(sub_node.dep) {
                    result.insert(name);
                }
                result.insert(sub_node.name);
            }
        }
    }
    result
}
