use std::vec;

enum EdgeType {
    Resistor,
    VoltageSource,
    CurrentSource,
}

struct Node {
    name: i32,
    voltage: f32,
}

struct Edge {
    node1: Node,
    node2: Node,
    uid: i32,

    edgetype: EdgeType,
    resistance: f32,

    current: f32,
}

struct Circuit {
    nodes: Vec<Node>,
    edges: Vec<Edge>,

    grounded_node: Option<Node>,
}

impl Circuit {
    fn init_node(&mut self) {
        let last_name: i32 = match self.nodes.is_empty() {
            true => -1,
            false => self.nodes.last().unwrap().name,
        };
        let new_node = Node {
            name: last_name + 1,
            voltage: 0.,
        };
        self.nodes.push(new_node);
    }
}

fn main() {
    let mut a = Circuit {
        nodes: vec![],
        edges: vec![],
        
        grounded_node: None,
    };

    a.init_node();
    a.init_node();
    a.init_node();

    println!("{}", a.nodes.get(2).unwrap().name);
}
