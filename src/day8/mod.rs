use std::str::Lines;

mod consts;

pub fn walk_network() -> usize {
    let mut lines = consts::INPUT.trim().lines();
    let mut steps = Steps::new(lines.next().unwrap().trim().as_bytes());
    lines.next();

    let mut network = Network::new(lines);
    println!("{:?}", network.current);
    loop {
        if steps.index % 1_000_000 == 0 {
            println!("step: {}, current: {:?}", steps.index, network.current);
        }
        network.walk(&mut steps);
        if steps.get_index() == 0 && network.current.iter().all(|node| network.nodes[*node].from.ends_with('Z')) {
            break;
        }
    }
    steps.index
}

#[derive(Debug)]
struct Steps {
    steps: Vec<u8>,
    index: usize,
}

impl Steps {
    fn new(steps: &[u8]) -> Self {
        Self {
            steps: steps.to_vec(),
            index: 0,
        }
    }

    fn next(&mut self) -> char {
        self.index += 1;
        self.steps[(self.index - 1) % self.steps.len()] as char
    }

    fn get_index(&self) -> usize {
        self.index % self.steps.len()
    }
}

#[derive(Debug)]
struct Network {
    nodes: Vec<Node>,
    current: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Node {
    from: String,
    left: usize,
    right: usize,
}

#[derive(Debug, Clone)]
struct NodeBuilder {
    from: String,
    left: String,
    right: String,
}

impl Network {
    fn new(lines: Lines) -> Self {
        let mut nodebuilders = Vec::new();
        for line in lines {
            let mut parts = line.trim().split(" = ");
            let from = parts.next().unwrap().trim().to_string();
            let mut sides = parts.next().unwrap().trim().split(", ");
            let left = sides.next().unwrap().trim().trim_start_matches('(').to_string();
            let right = sides.next().unwrap().trim().trim_end_matches(')').to_string();
            nodebuilders.push(NodeBuilder {
                from,
                left,
                right,
            });
        }
        let mut nodes = Vec::new();
        for nodebuilder in nodebuilders.clone() {
            let left = nodebuilders.iter().position(|n| n.from == nodebuilder.left).unwrap();
            let right = nodebuilders.iter().position(|n| n.from == nodebuilder.right).unwrap();
            nodes.push(Node {
                from: nodebuilder.from,
                left,
                right,
            });
        }
        let current_nodes: Vec<Node> = nodes.clone().into_iter().filter(|node| node.from.ends_with('A')).collect();
        let current: Vec<usize> = current_nodes.iter().map(|node| nodes.iter().position(|n| n.from == node.from).unwrap()).collect();
        Self {
            nodes,
            current,
        }
    }

    fn walk(&mut self, steps: &mut Steps) {
        let step = steps.next();
        for current in &mut self.current {
            *current = match step {
                'L' => self.nodes[*current].left,
                'R' => self.nodes[*current].right,
                _ => panic!("Invalid step: {}", step),
            };
        }
    }
}