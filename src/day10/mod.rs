use std::collections::VecDeque;

mod consts;

pub fn find_furthest() -> usize {
    let mut map: Map = Map::new(consts::INPUT);
    let max_y = map.map.len() - 1;
    let max_x = map.map[0].len() - 1;
    let start = map.find_start().unwrap();
    let mut queue: VecDeque<(Coords, usize)> = VecDeque::new();
    queue.push_back((start, 0));
    while !queue.is_empty() {
        let (coords, distance) = queue.pop_front().unwrap();
        let mut tiles = &mut map.map;
        if coords.x > max_x || coords.y > max_y {
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
                    if shifted_coords.x > max_x || shifted_coords.y > max_y {
                        continue;
                    }
                    if tiles[shifted_coords.y][shifted_coords.x].is_connected(&get_opposite_direction(direction)) {
                        queue.push_back((shifted_coords, distance + 1));
                    }
                }
            }
        }
    }
    let ret = map.find_highest_loop_val();
    map.print_loop();
    let start_coords = map.extend_map();
    if !map.flood(Coords { x: start_coords.x + 1, y: start_coords.y + 1 }, b'a') {
        println!("a: {}", map.count_flooded_tiles(b'a'));
    }
    if !map.flood(Coords { x: start_coords.x - 1, y: start_coords.y + 1 }, b'b') {
        println!("b: {}", map.count_flooded_tiles(b'b'));
    }
    if !map.flood(Coords { x: start_coords.x + 1, y: start_coords.y - 1 }, b'c') {
        println!("c: {}", map.count_flooded_tiles(b'c'));
    }
    if !map.flood(Coords { x: start_coords.x - 1, y: start_coords.y - 1 }, b'd') {
        println!("d: {}", map.count_flooded_tiles(b'd'));
    }
    // map.print_extended_map();
    println!();
    // map.print_flooded_small();
    ret
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
    extended_map: Option<Vec<Vec<u8>>>,
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
            map,
            extended_map: None,
        }
    }

    // returns location of b'S'
    fn extend_map(&mut self) -> Coords {
        let mut start_coords = Coords { x: 0, y: 0 };
        let mut extended_map = Vec::new();
        for (y, row) in self.map.iter().enumerate() {
            let mut extended_row_up = vec![b'e'; row.len() * 3];
            let mut extended_row_middle = vec![b'e'; row.len() * 3];
            let mut extended_row_down = vec![b'e'; row.len() * 3];
            for (x, col) in row.iter().enumerate() {
                if col.char == b'S' {
                    start_coords = Coords { x: x * 3 + 1, y: y * 3 + 1 };
                }
                let array = char_to_array(if col.part_of_loop { col.char } else { b'.' });
                extended_row_up[x * 3] = array[0][0];
                extended_row_up[x * 3 + 1] = array[0][1];
                extended_row_up[x * 3 + 2] = array[0][2];
                extended_row_middle[x * 3] = array[1][0];
                extended_row_middle[x * 3 + 1] = array[1][1];
                extended_row_middle[x * 3 + 2] = array[1][2];
                extended_row_down[x * 3] = array[2][0];
                extended_row_down[x * 3 + 1] = array[2][1];
                extended_row_down[x * 3 + 2] = array[2][2];
            }
            extended_map.push(extended_row_up);
            extended_map.push(extended_row_middle);
            extended_map.push(extended_row_down);
        }
        self.extended_map = Some(extended_map);
        start_coords
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

    fn find_highest_loop_val(&mut self) -> usize {
        let mut highest_coords: Option<Coords> = None;
        let mut highest = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                let tile = self.get_tile(&Coords { x: col, y: row }).unwrap();
                if let Some(distance) = tile.distance {
                    if distance > highest && self.can_tile_be_highest(&Coords { x: col, y: row }) {
                        highest_coords = Some(Coords { x: col, y: row });
                        highest = distance;
                    }
                }
            }
        }
        let mut queue: VecDeque<Coords> = VecDeque::new();
        queue.push_back(highest_coords.unwrap());
        while !queue.is_empty() {
            let coords = queue.pop_front().unwrap();
            self.get_tile_mut(&coords).unwrap().part_of_loop = true;
            if self.get_tile(&coords).unwrap().char == b'S' {
                continue;
            }
            for direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                if !self.get_tile(&coords).unwrap().is_connected(direction) {
                    continue;
                }
                if let Some(shifted_coords) = get_shifted_coords(&coords, direction) {
                    if let Some(tile) = self.get_tile(&shifted_coords) {
                        if tile.is_connected(&get_opposite_direction(direction))
                            && !tile.part_of_loop {
                            queue.push_back(shifted_coords);
                        }
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
                tile.distance.map_or_else(|| print!(".. "), |distance| print!("{distance:02} "));
            }
            println!();
        }
    }

    fn print_loop(&self) {
        for row in &self.map {
            for tile in row {
                if tile.part_of_loop {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn print_extended_map(&self) {
        for row in self.extended_map.as_ref().unwrap() {
            for tile in row {
                print!("{}", *tile as char);
            }
            println!();
        }
    }

    fn flood(&mut self, coords: Coords, char: u8) -> bool {
        let mut queue: VecDeque<Coords> = VecDeque::new();
        let mut touched_wall = false;
        queue.push_back(coords);
        while !queue.is_empty() {
            let coords = queue.pop_front().unwrap();
            if coords.y >= self.extended_map.as_ref().unwrap().len() || coords.x >= self.extended_map.as_ref().unwrap()[coords.y].len() {
                touched_wall = true;
                continue;
            }
            if self.extended_map.as_ref().unwrap()[coords.y][coords.x] != b'.' {
                continue;
            }
            self.extended_map.as_mut().unwrap()[coords.y][coords.x] = char;
            for direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                get_shifted_coords(&coords, direction).map_or_else(|| {
                    touched_wall = true;
                }, |shifted_coords| {
                    queue.push_back(shifted_coords);
                });
            }
        }
        touched_wall
    }

    fn count_flooded_tiles(&self, char: u8) -> usize {
        let mut count: usize = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if self.extended_map.as_ref().unwrap()[row * 3 + 1][col * 3 + 1] == char {
                    count += 1;
                }
            }
        }
        count
    }

    fn print_flooded_small(&self) {
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                print!("{}", self.extended_map.as_ref().unwrap()[row * 3 + 1][col * 3 + 1] as char);
            }
            println!();
        }
    }
}

struct Tile {
    char: u8,
    distance: Option<usize>,
    part_of_loop: bool,
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
            part_of_loop: false,
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
    const fn is_empty(&self) -> bool {
        self.char == b' '
    }
}

fn char_to_array(char: u8) -> [[u8; 3]; 3] {
    match char {
        b'.' => [
            [b'.', b'.', b'.'],
            [b'.', b'.', b'.'],
            [b'.', b'.', b'.']
        ],
        b'S' => [
            [b'.', b'x', b'.'],
            [b'x', b'x', b'x'],
            [b'.', b'x', b'.']
        ],
        b'|' => [
            [b'.', b'x', b'.'],
            [b'.', b'x', b'.'],
            [b'.', b'x', b'.']
        ],
        b'-' => [
            [b'.', b'.', b'.'],
            [b'x', b'x', b'x'],
            [b'.', b'.', b'.']
        ],
        b'J' => [
            [b'.', b'x', b'.'],
            [b'x', b'x', b'.'],
            [b'.', b'.', b'.']
        ],
        b'L' => [
            [b'.', b'x', b'.'],
            [b'.', b'x', b'x'],
            [b'.', b'.', b'.']
        ],
        b'F' => [
            [b'.', b'.', b'.'],
            [b'.', b'x', b'x'],
            [b'.', b'x', b'.']
        ],
        b'7' => [
            [b'.', b'.', b'.'],
            [b'x', b'x', b'.'],
            [b'.', b'x', b'.']
        ],
        _ => panic!("Invalid char: {}", char)
    }
}