mod consts;

use std::cmp::{min, Reverse};
use priority_queue::PriorityQueue;

pub fn get_cheapest_path() -> usize {
    println!("{}", consts::EXAMPLE1);
    let mut map = Map::new(consts::EXAMPLE1);
    let mut pq = map.get_init_pq();
    while !pq.is_empty() {
        let ((x, y), Reverse(w)) = pq.pop().unwrap();
        for (nx, ny) in  map.get_neighbors(x, y) {
            if let Some(Reverse(old_w)) = pq.get_priority(&(nx, ny)) {
                let node = &mut map.nodes[ny][nx];
                let new_w = min(w + node.weight, *old_w);
                if node.node_type == NodeType::End {
                    map.print();
                    return new_w;
                }
                pq.change_priority(&(nx, ny), Reverse(new_w));
                node.distance = Some(new_w);
            }
        }
    }
    map.print();
    0
}

struct Map {
    nodes: Vec<Vec<Node>>,
}

impl Map {
    pub fn new(input: &str) -> Map {
        let mut nodes: Vec<Vec<Node>> = Vec::new();
        for line in input.trim().lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Node {
                    weight: c as usize - 48,
                    node_type: NodeType::Regular,
                    distance: None,
                });
            }
            nodes.push(row);
        }
        nodes[0][0].node_type = NodeType::Start;
        let last_row = nodes.len() - 1;
        let last_col = nodes[0].len() - 1;
        nodes[last_row][last_col].node_type = NodeType::End;
        Map {
            nodes
        }
    }

    fn get_init_pq(&self) -> PriorityQueue<(usize, usize), Reverse<usize>> {
        let mut pq = PriorityQueue::new();
        for y in 0..self.nodes.len() {
            for x in 0..self.nodes[0].len() {
                if self.nodes[y][x].node_type != NodeType::Start {
                    pq.push((x, y), Reverse(usize::MAX));
                }
            }
        }
        pq.push((0, 0), Reverse(0));
        pq
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.nodes[0].len() - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.nodes.len() - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    fn print(&self) {
        for row in &self.nodes {
            for node in row {
                node.print();
            }
            println!();
        }
    }
}

struct Node {
    weight: usize,
    node_type: NodeType,
    distance: Option<usize>
}

impl Node {
    fn print(&self) {
        print!("{} ", self.weight);
        match self.distance {
            Some(d) => print!("{:4} ", d),
            None => print!("none "),
        }
    }
}

#[derive(PartialEq, Eq)]
enum NodeType {
    Regular,
    Start,
    End,
}
