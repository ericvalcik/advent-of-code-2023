use std::fs::File;
use std::io::Write;

mod consts;
pub mod better;

pub fn calc_pool_size() -> usize {
    let input = consts::INPUT.trim();
    let mut map = Map::new();
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        let mut split_str = line.split(" ");
        let dir = split_str.next().unwrap();
        let num = split_str.next().unwrap().parse::<i32>().unwrap();
        match dir {
            "R" => {
                for i in 0..num {
                    *map.get_mut(x + i, y) = true;
                }
                x += num;
            }
            "L" => {
                for i in 0..num {
                    *map.get_mut(x - i, y) = true;
                }
                x -= num;
            }
            "U" => {
                for i in 0..num {
                    *map.get_mut(x, y - i) = true;
                }
                y -= num;
            }
            "D" => {
                for i in 0..num {
                    *map.get_mut(x, y + i) = true;
                }
                y += num;
            }
            _ => panic!("Unknown direction: {dir}")
        }
    }
    map.flood(1, -1);
    map.print().expect("IO Error");
    map.count_true()
}

struct Map {
    map: Vec<Vec<bool>>
}

impl Map {
    fn new() -> Self {
        Self {
            map: vec![vec![false; 1001]; 1001]
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> &mut bool {
        assert!((-500..=500).contains(&x) && (-500..=500).contains(&y), "Out of bounds: {x}, {y}");
        &mut self.map[(x + 500) as usize][(y + 500) as usize]
    }

    fn get(&self, x: i32, y: i32) -> &bool {
        assert!((-500..=500).contains(&x) && (-500..=500).contains(&y), "Out of bounds: {x}, {y}");
        &self.map[(x + 500) as usize][(y + 500) as usize]
    }


    fn flood(&mut self, x: i32, y: i32) {
        if *self.get(x, y) {
            return;
        }
        *self.get_mut(x, y) = true;
        self.flood(x + 1, y);
        self.flood(x - 1, y);
        self.flood(x, y + 1);
        self.flood(x, y - 1);
    }

    fn print(&self) -> std::io::Result<()> {
        let mut min_x = 500;
        let mut max_x = -500;
        let mut min_y = 500;
        let mut max_y = -500;
        for x in -500..500 {
            for y in -500..500 {
                if *self.get(x, y) {
                    if x < min_x {
                        min_x = x;
                    }
                    if x > max_x {
                        max_x = x;
                    }
                    if y < min_y {
                        min_y = y;
                    }
                    if y > max_y {
                        max_y = y;
                    }
                }
            }
        }
        let mut file = File::create("foo.txt")?;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x == 0 && y == 0 {
                    file.write_all(b"X")?;
                    continue;
                }
                if *self.get(x, y) {
                    file.write_all(b"#")?;
                } else {
                    file.write_all(b".")?;
                }
            }
            file.write_all(b"\n")?;
        }
        Ok(())
    }

    fn count_true(&self) -> usize {
        let mut count = 0;
        for x in -500..=500 {
            for y in -500..=500 {
                if *self.get(x, y) {
                    count += 1;
                }
            }
        }
        count
    }
}