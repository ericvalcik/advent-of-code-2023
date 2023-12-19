mod consts;

use std::cmp::Reverse;
use std::collections::HashSet;
use std::collections::BinaryHeap;

pub fn get_cheapest_path() {
    // println!("{}", consts::EXAMPLE1.trim());
    let mut map = Map::new(consts::INPUT.trim());
    let mut seen: HashSet<(usize, usize, i32, i32, usize)> = HashSet::new();
    let mut pq: BinaryHeap<(Reverse<usize>, usize, usize, i32, i32, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), 0, 0, 0, 0, 0));
    while !pq.is_empty() {
        let (Reverse(w), x, y, dx, dy, n) = pq.pop().unwrap();
        if seen.contains(&(x, y, dx, dy, n)) {
            continue;
        }
        seen.insert((x, y, dx, dy, n));
        if x == map.nodes[0].len() - 1 && y == map.nodes.len() - 1 {
            map.nodes[y][x].distance = Some(w);
            map.print();
            println!("{w}");
            return;
        }
        for (nx, ny) in map.get_neighbors(x, y) {
            let ndx = nx as i32 - x as i32;
            let ndy = ny as i32 - y as i32;
            let node = &map.nodes[ny][nx];
            if (ndx, ndy) == (dx, dy) {
                if n < 3 {
                    pq.push((Reverse(w + node.weight), nx, ny, ndx, ndy, n + 1));
                }
            } else if (ndx, ndy) != (-dx, -dy) {
                pq.push((Reverse(w + node.weight), nx, ny, ndx, ndy, 1));
            }
        }
        if map.nodes[y][x].distance.is_some() {
            continue;
        }
        map.nodes[y][x].distance = Some(w);
    }
}

struct Map {
    nodes: Vec<Vec<Node>>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let mut nodes: Vec<Vec<Node>> = Vec::new();
        for line in input.trim().lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Node {
                    weight: c as usize - 48,
                    distance: None,
                });
            }
            nodes.push(row);
        }
        Self {
            nodes
        }
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
    distance: Option<usize>,
}

impl Node {
    fn print(&self) {
        print!("{} ", self.weight);
        self.distance.map_or_else(|| print!("none "), |d| print!("{d:<4} "));
    }
}
