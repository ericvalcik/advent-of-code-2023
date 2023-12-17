mod consts;

pub fn count_heated_squares() -> usize {
    let input = consts::INPUT.trim();
    let mut grid = Grid::new(input);
    grid.reflect(Beam {
        x: 0,
        y: 0,
        direction: Direction::Right,
    });
    grid.print_grid();
    grid.count_heated_squares()
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for line in input.lines() {
            tiles.push(line.chars().map(|char| Tile::new(char)).collect());
        }
        Self {
            tiles
        }
    }

    fn print_grid(&self) {
        for line in &self.tiles {
            for tile in line {
                print!("{}", tile.char);
            }
            print!(" ");
            for tile in line {
                print!("{}", if tile.heated { '#' } else { '.' });
            }
            println!();
        }
    }

    fn reflect(&mut self, beam: Beam) {
        let Beam { x, y, direction } = beam;
        if x < 0 || y < 0 || x >= self.tiles[0].len() as i32 || y >= self.tiles.len() as i32 {
            return;
        }
        if self.tiles[y as usize][x as usize].directions.contains(&direction) {
            return;
        }
        let mut current_tile = &mut self.tiles[y as usize][x as usize];
        current_tile.heated = true;
        current_tile.directions.push(direction.clone());
        match direction {
            Direction::Right => match current_tile.char {
                '-' | '.' => self.reflect(Beam {
                    x: x + 1,
                    y,
                    direction: Direction::Right,
                }),
                '\\' => self.reflect(Beam {
                    x,
                    y: y + 1,
                    direction: Direction::Down,
                }),
                '/' => self.reflect(Beam {
                    x,
                    y: y - 1,
                    direction: Direction::Up,
                }),
                '|' => {
                    self.reflect(Beam {
                        x,
                        y: y - 1,
                        direction: Direction::Up,
                    });
                    self.reflect(Beam {
                        x,
                        y: y + 1,
                        direction: Direction::Down,
                    });
                }
                _ => panic!("Invalid character: {}, x: {}, y: {}", current_tile.char, x, y),
            },
            Direction::Down => match current_tile.char {
                '.' | '|' => self.reflect(Beam {
                    x,
                    y: y + 1,
                    direction: Direction::Down,
                }),
                '\\' => self.reflect(Beam {
                    x: x + 1,
                    y,
                    direction: Direction::Right,
                }),
                '/' => self.reflect(Beam {
                    x: x - 1,
                    y,
                    direction: Direction::Left,
                }),
                '-' => {
                    self.reflect(Beam {
                        x: x - 1,
                        y,
                        direction: Direction::Left,
                    });
                    self.reflect(Beam {
                        x: x + 1,
                        y,
                        direction: Direction::Right,
                    });
                }
                _ => panic!("Invalid character: {}, x: {}, y: {}", current_tile.char, x, y),
            },
            Direction::Left => match current_tile.char {
                '-' | '.' => self.reflect(Beam {
                    x: x - 1,
                    y,
                    direction: Direction::Left,
                }),
                '\\' => self.reflect(Beam {
                    x,
                    y: y - 1,
                    direction: Direction::Up,
                }),
                '/' => self.reflect(Beam {
                    x,
                    y: y + 1,
                    direction: Direction::Down,
                }),
                '|' => {
                    self.reflect(Beam {
                        x,
                        y: y - 1,
                        direction: Direction::Up,
                    });
                    self.reflect(Beam {
                        x,
                        y: y + 1,
                        direction: Direction::Down,
                    });
                }
                _ => panic!("Invalid character: {}, x: {}, y: {}", current_tile.char, x, y),
            },
            Direction::Up => match current_tile.char {
                '.' | '|' => self.reflect(Beam {
                    x,
                    y: y - 1,
                    direction: Direction::Up,
                }),
                '\\' => self.reflect(Beam {
                    x: x - 1,
                    y,
                    direction: Direction::Left,
                }),
                '/' => self.reflect(Beam {
                    x: x + 1,
                    y,
                    direction: Direction::Right,
                }),
                '-' => {
                    self.reflect(Beam {
                        x: x - 1,
                        y,
                        direction: Direction::Left,
                    });
                    self.reflect(Beam {
                        x: x + 1,
                        y,
                        direction: Direction::Right,
                    });
                }
                _ => panic!("Invalid character: {}, x: {}, y: {}", current_tile.char, x, y),
            }
        };
    }

    fn count_heated_squares(&self) -> usize {
        let mut result = 0;
        for line in &self.tiles {
            for tile in line {
                if tile.heated {
                    result += 1;
                }
            }
        }
        result
    }
}

struct Tile {
    char: char,
    heated: bool,
    directions: Vec<Direction>
}

impl Tile {
    const fn new(char: char) -> Self {
        Self {
            char,
            heated: false,
            directions: Vec::new(),
        }
    }
}

struct Beam {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}