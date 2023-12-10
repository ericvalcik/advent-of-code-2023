use std::collections::VecDeque;

mod consts;

pub fn find_furthest() -> usize {
    let mut map: Map = Map::new(consts::INPUT);
    let maxY = map.map.len() - 1;
    let maxX = map.map[0].len() - 1;
    let start = map.find_start().unwrap();
    let mut queue: VecDeque<(Coords, usize)> = VecDeque::new();
    queue.push_back((start, 0));
    while !queue.is_empty() {
        let (coords, distance) = queue.pop_front().unwrap();
        let mut tiles = &mut map.map;
        if coords.x > maxX || coords.y > maxY {
            continue;
        }
        if tiles[coords.y][coords.x].distance.is_some() {
            continue;
        }
        if tiles[coords.y][coords.x].is_pipe() {
            tiles[coords.y][coords.x].distance = Some(distance);
        }
        for direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            if tiles[coords.y][coords.x].is_connected(direction) {
                if let Some(shifted_coords) = get_shifted_coords(&coords, direction) {
                    if shifted_coords.x > maxX || shifted_coords.y > maxY {
                        continue;
                    }
                    if tiles[shifted_coords.y][shifted_coords.x].is_connected(&get_opposite_direction(direction)) {
                        queue.push_back((shifted_coords, distance + 1));
                    }
                }
            }
        }
    }
    map.print_distance();
    map.find_highest_loop_val()
}

#[derive(Clone)]
struct Coords {
    x: usize,
    y: usize,
}

const fn get_shifted_coords(coords: &Coords, direction: &Direction) -> Option<Coords> {
    match direction {
        Direction::Up => {
            if coords.y == 0 {
                return None;
            }
            Some(Coords { x: coords.x, y: coords.y - 1 })
        }
        Direction::Down => {
            Some(Coords { x: coords.x, y: coords.y + 1 })
        }
        Direction::Left => {
            if coords.x == 0 {
                return None;
            }
            Some(Coords { x: coords.x - 1, y: coords.y })
        }
        Direction::Right => Some(Coords { x: coords.x + 1, y: coords.y })
    }
}

struct Map {
    map: Vec<Vec<Tile>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map = Vec::new();
        for line in input.trim().lines() {
            let mut row = Vec::new();
            for char in line.trim().bytes() {
                row.push(Tile::new(char));
            }
            map.push(row);
        }
        Self {
            map
        }
    }

    fn find_start(&self) -> Option<Coords> {
        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.char == b'S' {
                    return Some(Coords { x, y });
                }
            }
        }
        None
    }

    fn get_tile_mut(&mut self, coords: &Coords) -> Option<&mut Tile> {
        self.map.get_mut(coords.y)?.get_mut(coords.x)
    }

    fn get_tile(&self, coords: &Coords) -> Option<&Tile> {
        self.map.get(coords.y)?.get(coords.x)
    }

    fn find_highest_loop_val(&self) -> usize {
        let mut highest = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                let tile = self.get_tile(&Coords { x: col, y: row }).unwrap();
                if let Some(distance) = tile.distance {
                    if distance > highest && self.can_tile_be_highest(&Coords { x: col, y: row }) {
                        highest = distance;
                    }
                }
            }
        }
        highest
    }

    fn can_tile_be_highest(&self, coords: &Coords) -> bool {
        let distance = self.get_tile(coords).unwrap().distance.unwrap();
        let mut connected_pipes = 0;
        let dist_array = [distance, distance - 1];
        for direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            if !self.get_tile(coords).unwrap().is_connected(direction) {
                continue;
            }
            if let Some(shifted_coords) = get_shifted_coords(coords, direction) {
                if let Some(tile) = self.get_tile(&shifted_coords) {
                    if tile.is_pipe() && dist_array.contains(&tile.distance.unwrap()) {
                        connected_pipes += 1;
                    }
                }
            }
        }
        connected_pipes == 2
    }

    fn print_distance(&self) {
        for row in &self.map {
            for tile in row {
                tile.distance.map_or_else(|| print!(".. "), |distance| print!("{:02} ", distance))
            }
            println!();
        }
    }
}

struct Tile {
    char: u8,
    distance: Option<usize>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const fn get_opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

impl Tile {
    const fn new(char: u8) -> Self {
        Self {
            char,
            distance: None,
        }
    }

    const fn is_connected(&self, direction: &Direction) -> bool {
        if self.char == b'S' {
            return true;
        }
        match direction {
            Direction::Up => self.char == b'|' || self.char == b'L' || self.char == b'J',
            Direction::Down => self.char == b'|' || self.char == b'7' || self.char == b'F',
            Direction::Left => self.char == b'-' || self.char == b'J' || self.char == b'7',
            Direction::Right => self.char == b'-' || self.char == b'L' || self.char == b'F',
        }
    }

    fn is_pipe(&self) -> bool {
        [b'S', b'|', b'-', b'J', b'L', b'F', b'7'].contains(&self.char)
    }
}