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
        if network.current.iter().all(|node| node.ends_with('Z')) {
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
    current: Vec<String>,
}

#[derive(Debug)]
struct Node {
    from: String,
    left: String,
    right: String,
}

impl Network {
    fn new(lines: Lines) -> Self {
        let mut nodes = Vec::new();
        for line in lines {
            let mut parts = line.trim().split(" = ");
            let from = parts.next().unwrap().trim().to_string();
            let mut sides = parts.next().unwrap().trim().split(", ");
            let left = sides.next().unwrap().trim().trim_start_matches('(').to_string();
            let right = sides.next().unwrap().trim().trim_end_matches(')').to_string();
            nodes.push(Node {
                from,
                left,
                right,
            });
        }
        let current = nodes.iter().filter(|node| node.from.ends_with('A')).map(|node| node.from.clone()).collect();
        Self {
            nodes,
            current,
        }
    }

    fn walk(&mut self, steps: &mut Steps) {
        let step = steps.next();
        for current in &mut self.current {
            let node = self.nodes.iter().find(|n| n.from == *current).unwrap();
            *current = match step {
                'L' => node.left.clone(),
                'R' => node.right.clone(),
                _ => panic!("Invalid step: {}", step),
            };
        }
    }
}