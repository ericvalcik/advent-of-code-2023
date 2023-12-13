use std::cmp::min;

mod consts;

pub fn find_reflections() -> usize {
    let tiles = consts::INPUT.trim().split("\n\n").collect::<Vec<&str>>();
    let mut result = 0;
    for tile_input in tiles {
        let tile = Tile::new(tile_input);
        if let Some(count) = tile.check_vertical_reflection() {
            result += count;
        }
        if let Some(count) = tile.check_horizontal_reflection() {
            result += 100 * count;
        }
    }
    result
}

struct Tile {
    map: Vec<Vec<char>>,
}

impl Tile {
    pub fn new(input: &str) -> Self {
        let map = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        Self { map }
    }

    fn check_vertical_reflection(&self) -> Option<usize> {
        'start: for i in 1..self.map[0].len() {
            let mut count = 0;
            for j in 0..min(i, self.map[0].len() - i) {
                for line in &self.map {
                    if line[i-j-1] != line[i+j] {
                        count += 1;
                        if count > 1 {
                            continue 'start;
                        }
                    }
                }
            }
            if count == 1 {
                return Some(i);
            }
        }
        None
    }

    fn check_horizontal_reflection(&self) -> Option<usize> {
        'start: for i in 1..self.map.len() {
            let mut count = 0;
            for j in 0..min(i, self.map.len() - i) {
                for col in 0..self.map[0].len() {
                    if self.map[i-j-1][col] != self.map[i+j][col] {
                        count += 1;
                        if count > 1 {
                            continue 'start;
                        }
                    }
                }
            }
            if count == 1 {
                return Some(i);
            }
        }
        None
    }
}