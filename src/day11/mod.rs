use std::io::Lines;

mod consts;

const EXPAND_MULTIPLIER: usize = 1_000_000;

pub fn calc_distances() -> usize {
    let mut map = Map::new(consts::INPUT);
    map.expand_map();
    let galaxies = map.find_galaxies();
    let mut count: usize = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            count += map.get_distance(&galaxies[i], &galaxies[j]);
        }
    }
    count
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
}

impl Map {
    fn new(input: &str) -> Self {
        let lines = input.trim().lines();
        let mut map = Vec::new();
        for line in lines {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            map.push(row);
        }
        Self { map, expanded_rows: Vec::new(), expanded_cols: Vec::new() }
    }

    fn print_map(&self) {
        for row in &self.map {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
        println!("Exapnded rows: {:?}", self.expanded_rows);
        println!("Exapnded cols: {:?}", self.expanded_cols);
    }

    fn expand_map(&mut self) {
        // expand rows
        let mut row: usize = 0;
        while row < self.map.len() {
            let no_galaxy = self.map[row].iter().all(|c| *c == '.');
            if no_galaxy {
                self.expanded_rows.push(row);
            }
            row += 1;
        }
        // expand cols
        let mut col: usize = 0;
        while col < self.map[0].len() {
            let no_galaxy = self.map.iter().all(|row| row[col] == '.');
            if no_galaxy {
                self.expanded_cols.push(col);
            }
            col += 1;
        }
    }

    fn find_galaxies(&self) -> Vec<Galaxy> {
        let mut galaxies = Vec::new();
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if self.map[row][col] == '#' {
                    galaxies.push(Galaxy { x: col, y: row });
                }
            }
        }
        galaxies
    }

    fn get_distance(&self, galaxy1: &Galaxy, galaxy2: &Galaxy) -> usize {
        let x = if galaxy1.x > galaxy2.x {
            galaxy1.x - galaxy2.x + (EXPAND_MULTIPLIER - 1) * self.expanded_cols.iter().filter(|&&col| col < galaxy1.x && col > galaxy2.x).count()
        } else {
            galaxy2.x - galaxy1.x + (EXPAND_MULTIPLIER - 1) * self.expanded_cols.iter().filter(|&&col| col < galaxy2.x && col > galaxy1.x).count()
        };
        let y = if galaxy1.y > galaxy2.y {
            galaxy1.y - galaxy2.y + (EXPAND_MULTIPLIER - 1) * self.expanded_rows.iter().filter(|&&row| row < galaxy1.y && row > galaxy2.y).count()
        } else {
            galaxy2.y - galaxy1.y + (EXPAND_MULTIPLIER - 1) * self.expanded_rows.iter().filter(|&&row| row < galaxy2.y && row > galaxy1.y).count()
        };
        x + y
    }
}

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}