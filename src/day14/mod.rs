mod consts;

pub fn calculate_load() -> usize {
    let mut map = Map::new(consts::INPUT.trim());
    map.roll_up();
    map.print_map();
    map.calc_load()
}

struct Map  {
    grid: Vec<Vec<char>>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let grid = input.lines().map(|line| line.chars().collect()).collect();
        Self { grid }
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn roll_up(&mut self) {
        for col in 0..self.width() {
            let mut can_roll_to: Option<usize> = None;
            for row in 0..self.height() {
                if self.grid[row][col] == 'O' {
                    // try to roll it up
                    if let Some(roll_to) = can_roll_to {
                        self.grid[roll_to][col] = 'O';
                        self.grid[row][col] = '.';
                        can_roll_to = Some(roll_to + 1);
                    }
                } else if self.grid[row][col] == '#' {
                    can_roll_to = None;
                } else if can_roll_to.is_none() {
                    can_roll_to = Some(row);
                }
            }
        }
    }

    pub fn print_map(&self) {
        for row in 0..self.height() {
            for col in 0..self.width() {
                print!("{}", self.grid[row][col]);
            }
            println!();
        }
    }

    pub fn calc_load(&self) -> usize {
        let mut load = 0;
        for row in 0..self.height() {
            for col in 0..self.width() {
                if self.grid[row][col] == 'O' {
                    load += self.height() - row;
                }
            }
        }
        load
    }
}