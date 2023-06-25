use std::vec;

enum EdgeType {
    Resistor,
    VoltageSource,
    CurrentSource,
}

struct Node {
    id: i32,
    hr_name: char,

    voltage: f32,
}

struct Edge {
    node1: i32,
    node2: i32,
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
    fn init_node(&mut self, hr_name:char) {
        let last_id: i32 = match self.nodes.is_empty() {
            true => -1,
            false => self.nodes.last().unwrap().id,
        };
        let new_node = Node {
            id: last_id + 1,
            hr_name: hr_name,
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

    a.init_node('a');
    a.init_node('b');
    a.init_node('c');

    println!("{}", a.nodes.get(2).unwrap().hr_name);
}
