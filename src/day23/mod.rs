use std::collections::{HashSet, VecDeque};

mod consts;

pub fn day23() -> usize {
    let input = consts::INPUT.trim();
    let mut map = Map::new(input);
    map.nodes.push(Coords { x: 1, y: 0 });
    for y in 0..map.tiles.len() {
        for x in 0..map.tiles[0].len() {
            if map.tiles[y][x] != '#' && map.get_neighbors(&Coords { x, y }).len() > 2 {
                map.nodes.push(Coords { x, y });
            }
        }
    }
    map.nodes.push(Coords { x: map.tiles[0].len() - 2, y: map.tiles.len() - 1 });

    let mut seen: HashSet<Coords> = HashSet::new();
    let mut graph: Vec<Vec<Option<usize>>> = vec![vec![None; map.nodes.len()]; map.nodes.len()];
    create_graph(&map, &mut graph, &mut seen);
    println!("{:?}", map.nodes.len());
    println!("{:?}", map.nodes);
    fill_graph(&mut graph);
    print_graph(&graph);
    find_most_expensive_path(&graph)
}

fn create_graph(map: &Map, graph: &mut [Vec<Option<usize>>], seen: &mut HashSet<Coords>) {
    let mut nodes: VecDeque<Coords> = VecDeque::new();
    nodes.push_back(Coords { x: 1, y: 0 });
    while !nodes.is_empty() {
        let coords = nodes.pop_front().unwrap();
        seen.insert(coords.clone());
        let neighbors = map.get_neighbors(&coords);
        for neighbor in neighbors {
            if !seen.contains(&neighbor) {
                let (weight, new_coords) = map.walk(seen, neighbor);
                graph[map.nodes.iter().position(|c| c == &coords).unwrap()][map.nodes.iter().position(|coords| coords == &new_coords).unwrap()] = Some(weight + 1);
                nodes.push_back(new_coords);
            }
        }
        seen.remove(&coords);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

struct Map {
    tiles: Vec<Vec<char>>,
    nodes: Vec<Coords>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut tiles = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for char in line.chars() {
                row.push(char);
            }
            tiles.push(row);
        }
        Self {
            tiles,
            nodes: Vec::new(),
        }
    }

    fn get(&self, coords: &Coords) -> Option<&char> {
        self.tiles.get(coords.y)?.get(coords.x)
    }

    fn get_neighbors(&self, coords: &Coords) -> Vec<Coords> {
        let mut neighbors = Vec::new();
        if coords.x > 0 {
            let coords = Coords { x: coords.x - 1, y: coords.y };
            if self.get(&coords) != Some(&'#') {
                neighbors.push(coords);
            }
        }
        if coords.x < self.tiles[0].len() - 1 {
            let coords = Coords { x: coords.x + 1, y: coords.y };
            if self.get(&coords) != Some(&'#') {
                neighbors.push(coords);
            }
        }
        if coords.y > 0 {
            let coords = Coords { x: coords.x, y: coords.y - 1 };
            if self.get(&coords) != Some(&'#') {
                neighbors.push(coords);
            }
        }
        if coords.y < self.tiles.len() - 1 {
            let coords = Coords { x: coords.x, y: coords.y + 1 };
            if self.get(&coords) != Some(&'#') {
                neighbors.push(coords);
            }
        }
        neighbors
    }

    fn get_next_step(&self, coords: &Coords, seen: &HashSet<Coords>) -> Coords {
        let mut neighbors = self.get_neighbors(coords);
        neighbors.retain(|coords| !seen.contains(coords));
        neighbors.into_iter().next().unwrap()
    }

    fn walk(&self, seen: &mut HashSet<Coords>, coords: Coords) -> (usize, Coords) {
        if self.is_node(&coords) {
            return (0, coords);
        }
        seen.insert(coords.clone());
        let Coords { x, y } = self.get_next_step(&coords, seen);
        let (weight, node) = self.walk(seen, Coords { x, y });
        (1 + weight, node)
    }

    fn is_node(&self, coords: &Coords) -> bool {
        self.nodes.contains(coords)
    }
}

fn print_graph(graph: &[Vec<Option<usize>>]) {
    print!("   ");
    for i in 0..graph.len() {
        print!("{i:2} ");
    }
    println!();
    for (i, row) in graph.iter().enumerate() {
        print!("{i:3?}  ");
        for col in row {
            col.as_ref().map_or_else(|| print!(".   "), |weight| print!("{:3?} ", weight));
        }
        println!();
    }
}

fn fill_graph(graph: &mut [Vec<Option<usize>>]) {
    for y in 0..graph.len() {
        for x in 0..graph[0].len() {
            if let Some(val) = graph[y][x] {
                graph[x][y] = Some(val);
            }
        }
    }
}

fn find_most_expensive_path(graph: &[Vec<Option<usize>>]) -> usize {
    walk_to_end_graph(graph, 0, HashSet::new()).unwrap()
}

fn walk_to_end_graph(graph: &[Vec<Option<usize>>], node: usize, mut seen: HashSet<usize>) -> Option<usize> {
    if node == graph.len() - 1 {
        return Some(0);
    }
    seen.insert(node);
    let mut vec = Vec::new();
    for (neighbor, weight) in graph[node].iter().enumerate() {
        if let Some(weight) = weight {
            if !seen.contains(&neighbor) {
                if let Some(w) = walk_to_end_graph(graph, neighbor, seen.clone()) {
                    vec.push(weight + w);
                }
            }
        }
    }
    vec.into_iter().max()
}

struct Tile {
    char: char,
}

impl Tile {
    const fn new(char: char) -> Self {
        Self {
            char
        }
    }
}