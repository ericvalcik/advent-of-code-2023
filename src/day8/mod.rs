use std::str::Lines;

mod consts;

pub fn walk_network() -> u32 {
    let mut lines = consts::INPUT.trim().lines();
    let mut count: u32 = 0;
    let mut steps = Steps::new(lines.next().unwrap().trim().as_bytes());
    lines.next();

    let mut network = Network::new(lines);
    loop {
        count += 1;
        network.walk(&mut steps);
        if steps.get_index() == 0 && network.current == "ZZZ" {
            break;
        }
    }
    count
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
    current: String,
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
        let current = "AAA".to_string();
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
        Self {
            nodes,
            current,
        }
    }

    fn walk(&mut self, steps: &mut Steps) {
        let step = steps.next();
        let node = self.nodes.iter().find(|n| n.from == self.current).unwrap();
        match step {
            'L' => self.current = node.left.clone(),
            'R' => self.current = node.right.clone(),
            _ => panic!("Invalid step: {}", step),
        }
    }
}