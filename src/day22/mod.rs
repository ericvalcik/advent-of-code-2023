use std::collections::HashSet;

mod consts;

pub fn day22() -> usize {
    let input = consts::INPUT.trim();
    let mut cubes: Vec<Cube> = Vec::new();
    for line in input.lines() {
        cubes.push(Cube::new(line));
    }
    cubes.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    let mut grid = Grid::new();
    for cube in cubes {
        drop_cube_in_grid(&mut grid, &cube);
    }
    let mut result = 0;
    for cube in 0..grid.cubes.len() {
        let mut new_grid = grid.clone();
        new_grid.cubes[cube].support_count = Some(0);
        result += disintegrate_cube(&mut new_grid, cube) - 1;
    }
    result
}

fn disintegrate_cube(grid: &mut Grid, cube_index: usize) -> usize {
    let cube = grid.cubes[cube_index].clone();
    if cube.support_count.unwrap() != 0 {
        return 0;
    }
    let mut result = 1;
    if cube.start.x == cube.end.x && cube.start.y == cube.end.y {
        // vertical cube
        if let Some(index) = grid.grid[cube.start.x][cube.start.y][cube.end.z + 1] {
            grid.cubes[index].support_count = Some(grid.cubes[index].support_count.unwrap() - 1);
            result += disintegrate_cube(grid, index);
        }
    } else {
        let mut set: HashSet<usize> = HashSet::new();
        // horizontal cube
        for x in cube.start.x..=cube.end.x {
            for y in cube.start.y..=cube.end.y {
                if let Some(index) = grid.grid[x][y][cube.end.z + 1] {
                    if set.contains(&index) {
                        continue;
                    }
                    grid.cubes[index].support_count = Some(grid.cubes[index].support_count.unwrap() - 1);
                    result += disintegrate_cube(grid, index);
                    set.insert(index);
                }
            }
        }
    }
    result
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
struct Coords {
    x: usize,
    y: usize,
    z: usize,
}

impl Coords {
    fn new(input: &str) -> Self {
        let mut digits = input.trim().split(',').map(|s| s.parse::<usize>().unwrap());
        Self {
            x: digits.next().unwrap(),
            y: digits.next().unwrap(),
            z: digits.next().unwrap(),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Cube {
    start: Coords,
    end: Coords,
    support_count: Option<usize>,
}

impl Cube {
    fn new(input: &str) -> Self {
        let mut coords = input.trim().split('~');
        let start = Coords::new(coords.next().unwrap());
        let end = Coords::new(coords.next().unwrap());
        if start > end {
            Self {
                start: end,
                end: start,
                support_count: None,
            }
        } else {
            Self {
                start,
                end,
                support_count: None,
            }
        }
    }

    const fn occupies(&self, coords: &Coords) -> bool {
        coords.x >= self.start.x && coords.x <= self.end.x &&
            coords.y >= self.start.y && coords.y <= self.end.y &&
            coords.z >= self.start.z && coords.z <= self.end.z
    }

    fn iter_coords(&self) -> Vec<Coords> {
        let mut coords = Vec::new();
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    coords.push(Coords { x, y, z });
                }
            }
        }
        coords
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cubes: Vec<Cube>,
    grid: Vec<Vec<Vec<Option<usize>>>>, // index in cubes
}

impl Grid {
    fn new() -> Self {
        Self {
            cubes: Vec::new(),
            grid: vec![vec![vec![None; 400]; 10]; 10],
        }
    }

    fn print(&self, from_z: usize) {
        println!("start z = {from_z}");
        for z in (1..=from_z).rev() {
            'x: for x in 0..10 {
                for y in 0..10 {
                    if self.grid[x][y][z].is_some() {
                        print!("#");
                        continue 'x;
                    }
                }
                print!(".");
            }
            print!(" ");
            'y: for y in 0..10 {
                for x in 0..10 {
                    if self.grid[x][y][z].is_some() {
                        print!("#");
                        continue 'y;
                    }
                }
                print!(".");
            }
            println!();
        }
    }
}

fn drop_cube_in_grid(grid: &mut Grid, cube: &Cube) {
    // calculate how much we can drop the cube until it hits another cube or ground
    let is_vertical = cube.start.x == cube.end.x && cube.start.y == cube.end.y;
    let mut drop: usize = 0;
    if cube.start.z != 1 {
        // if z is changing, we just need to check one direction (the cube is vertical)
        if is_vertical {
            for z in (1..cube.start.z).rev() {
                if grid.grid[cube.start.x][cube.start.y][z].is_none() {
                    drop += 1;
                } else {
                    break;
                }
            }
            // else - cube is horizontal, we need to check the every coord of the cube
        } else {
            'outer: for z in (1..cube.start.z).rev() {
                for x in cube.start.x..=cube.end.x {
                    for y in cube.start.y..=cube.end.y {
                        if grid.grid[x][y][z].is_some() {
                            break 'outer;
                        }
                    }
                }
                drop += 1;
            }
        }
    }

    // new cube
    let mut new_cube = Cube {
        start: Coords {
            x: cube.start.x,
            y: cube.start.y,
            z: cube.start.z - drop,
        },
        end: Coords {
            x: cube.end.x,
            y: cube.end.y,
            z: cube.end.z - drop,
        },
        support_count: None,
    };

    // get the number of cubes that support this cube
    let support_count = if new_cube.start.z == 1 {
        usize::MAX
    } else {
        let mut set: HashSet<usize> = HashSet::new();
        for x in new_cube.start.x..=new_cube.end.x {
            for y in new_cube.start.y..=new_cube.end.y {
                if let Some(index) = grid.grid[x][y][new_cube.start.z - 1] {
                    set.insert(index);
                }
            }
        }
        set.len()
    };
    new_cube.support_count = Some(support_count);

    // put the cube into the grid
    for x in new_cube.start.x..=new_cube.end.x {
        for y in new_cube.start.y..=new_cube.end.y {
            for z in new_cube.start.z..=new_cube.end.z {
                assert!(grid.grid[x][y][z].is_none(), "cube already exists at {:?} and shouldn't", Coords { x, y, z });
                grid.grid[x][y][z] = Some(grid.cubes.len());
            }
        }
    }
    grid.cubes.push(new_cube);
}