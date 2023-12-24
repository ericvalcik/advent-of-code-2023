use std::io::{stdin, Read};

mod consts;

const INPUT: &str = consts::INPUT;

// STEPS = 202300 * SIZE + 65
const STEPS: usize = 26_501_365;

const FULL_TILES_WIDTH: usize = 202_299;

const SIZE: usize = 131;

pub fn walk_tiles() -> usize {
    let mut total = 0;

    // full tiles
    let even_full_tiles = ((FULL_TILES_WIDTH + 1) / 2 * 2) * ((FULL_TILES_WIDTH + 1) / 2 * 2);
    let odd_full_tiles = (FULL_TILES_WIDTH / 2 * 2 + 1) * (FULL_TILES_WIDTH / 2 * 2 + 1);
    total += even_full_tiles * fill(65, 65, SIZE * 2);
    total += odd_full_tiles * fill(65, 65, SIZE * 2 + 1);

    // top
    total += fill(65, 130, 130);
    // bottom
    total += fill(65, 0, 130);
    // left
    total += fill(130, 65, 130);
    // right
    total += fill(0, 65, 130);

    // there is 202300 tiles in all of the cases below
    let corner_cases = 202_300;
    // top left - outer
    total += fill(130, 130, 64) * corner_cases;
    // top left - inner
    total += fill(130, 130, 131 + 64) * (corner_cases-1);
    // top right - outer
    total += fill(0, 130, 64) * corner_cases;
    // top right - inner
    total += fill(0, 130, 131 + 64) * (corner_cases-1);
    // bottom left - outer
    total += fill(130, 0, 64) * corner_cases;
    // bottom left - inner
    total += fill(130, 0, 131 + 64) * (corner_cases-1);
    // bottom right - outer
    total += fill(0, 0, 64) * corner_cases;
    // bottom right - inner
    total += fill(0, 0, 131 + 64) * (corner_cases-1);
    total
}

struct Map {
    tiles: Vec<Vec<char>>
}

impl Map {
    fn new(input: &str) -> Self {
        let mut tiles: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                if c == 'S' {
                    row.push('.');
                } else {
                    row.push(c);
                }
            }
            tiles.push(row);
        }
        Self {
            tiles
        }
    }

    fn step(&mut self) {
        let mut check_tiles: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                if self.tiles[y][x] == 'O' {
                    check_tiles.push((x + 1, y));
                    if x > 0 {
                        check_tiles.push((x - 1, y));
                    }
                    check_tiles.push((x, y + 1));
                    if y > 0 {
                        check_tiles.push((x, y - 1));
                    }
                    self.tiles[y][x] = '.';
                }
            }
        }
        for tile in check_tiles {
            if tile.1 < self.tiles.len() && tile.0 < self.tiles[tile.1].len() && self.tiles[tile.1][tile.0] == '.' {
                    self.tiles[tile.1][tile.0] = 'O';
            }
        }
    }

    fn count_o(&self) -> usize {
        let mut count = 0;
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                if self.tiles[y][x] == 'O' {
                    count += 1;
                }
            }
        }
        count
    }

    fn print(&self) {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                print!("{}", self.tiles[y][x]);
            }
            println!();
        }
    }
}

fn fill(x: usize, y: usize, steps: usize) -> usize {
    let mut map = Map::new(INPUT);
    map.tiles[y][x] = 'O';
    for _ in 0..steps {
        map.step();
    }
    println!("{x} {y} {steps}");
    let count = map.count_o();
    println!("{count}");
    count
}
