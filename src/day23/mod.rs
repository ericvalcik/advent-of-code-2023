mod consts;

pub fn day23() -> usize {
    let input = consts::INPUT.trim();
    let map = Map::new(input);
    map.walk(Coords { x: 1, y: 0 }, Vec::new()).unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut tiles = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for char in line.chars() {
                row.push(Tile::new(char));
            }
            tiles.push(row);
        }
        Self {
            tiles,
        }
    }

    fn get_next_steps(&self, coords: &Coords) -> Vec<Coords> {
        let mut neighbors = Vec::new();
        let tile = &self.tiles[coords.y][coords.x];
        if tile.char == '>' {
            neighbors.push(Coords { x: coords.x + 1, y: coords.y });
            return neighbors;
        } else if tile.char == '<' {
            neighbors.push(Coords { x: coords.x - 1, y: coords.y });
            return neighbors;
        } else if tile.char == '^' {
            neighbors.push(Coords { x: coords.x, y: coords.y - 1 });
            return neighbors;
        } else if tile.char == 'v' {
            neighbors.push(Coords { x: coords.x, y: coords.y + 1 });
            return neighbors;
        }
        if coords.x > 0 {
            neighbors.push(Coords { x: coords.x - 1, y: coords.y });
        }
        if coords.x < self.tiles[0].len() - 1 {
            neighbors.push(Coords { x: coords.x + 1, y: coords.y });
        }
        if coords.y > 0 {
            neighbors.push(Coords { x: coords.x, y: coords.y - 1 });
        }
        if coords.y < self.tiles.len() - 1 {
            neighbors.push(Coords { x: coords.x, y: coords.y + 1 });
        }
        neighbors
    }

    fn walk(&self, coords: Coords, mut seen: Vec<Coords>) -> Option<usize> {
        if coords.y == self.tiles.len() - 1 {
            return Some(0);
        }
        seen.push(coords.clone());
        let mut vec = Vec::new();
        for Coords { x, y } in self.get_next_steps(&coords) {
            if self.tiles[y][x].char == '#' || seen.contains(&Coords { x, y }) {
                continue;
            }
            match self.walk(Coords { x, y }, seen.clone()) {
                Some(steps) => vec.push(steps + 1),
                None => continue,
            }
        }
        if vec.is_empty() {
            return None;
        }
        Some(*vec.iter().max().unwrap())
    }
}

struct Tile {
    char: char,
}

impl Tile {
    fn new(char: char) -> Self {
        Self {
            char
        }
    }
}