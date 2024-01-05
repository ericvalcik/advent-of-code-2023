mod consts;
use std::ops::{Add, Sub};

const INPUT: &str = consts::INPUT;
const MIN: f64 = 200_000_000_000_000.0;
const MAX: f64 = 400_000_000_000_000.0;


pub fn day24() -> usize {
    let lines = INPUT.trim().lines();
    let lines: Vec<Line> = lines.map(Line::new).collect();

    let mut count = 0;
    for i in 0..lines.len() {
        for j in i+1..lines.len() {
            if let Some(intersection) = get_intersection(&lines[i], &lines[j]) {
                if intersection.0 >= MIN && intersection.0 <= MAX && intersection.1 >= MIN && intersection.1 <= MAX {
                    // println!("{:?} {:?}", lines[i], lines[j]);
                    // println!("Intersection: {:?}", intersection);
                    count += 1;
                }
            }
        }
    }
    count
}

fn get_intersection(a: &Line, b: &Line) -> Option<(f64, f64)> {
    let a1 = a.b.y - a.a.y;
    let b1 = a.a.x - a.b.x;
    let c1 = a1*(a.a.x) + b1*(a.a.y);

    let a2 = b.b.y - b.a.y;
    let b2 = b.a.x - b.b.x;
    let c2 = a2*(b.a.x) + b2*(b.a.y);

    let det = a1*b2 - a2*b1;

    if det == 0 {
        None
    } else {
        let x = ((b2*c1 - b1*c2) as f64) / (det as f64);
        let y = ((a1*c2 - a2*c1) as f64) / (det as f64);
        // check if intersection is on the past for a
        let a_diff = a.b.clone() - a.a.clone();
        if a_diff.x > 0 && x < a.a.x as f64 {
            return None;
        }
        if a_diff.x < 0 && x > a.a.x as f64 {
            return None;
        }
        if a_diff.y > 0 && y < a.a.y as f64 {
            return None;
        }
        if a_diff.y < 0 && y > a.a.y as f64 {
            return None;
        }
        // check if intersection is on the past for b
        let b_diff = b.b.clone() - b.a.clone();
        if b_diff.x > 0 && x < b.a.x as f64 {
            return None;
        }
        if b_diff.x < 0 && x > b.a.x as f64 {
            return None;
        }
        if b_diff.y > 0 && y < b.a.y as f64 {
            return None;
        }
        if b_diff.y < 0 && y > b.a.y as f64 {
            return None;
        }
        Some((x, y))
    }
}

const fn det2d(a: &Coords, b: &Coords) -> i128 {
    a.x * b.y - a.y * b.x
}

#[derive(Debug, Clone)]
struct Coords {
    x: i128,
    y: i128,
    z: i128,
}

impl Coords {
    fn new(input: &str) -> Self {
        let mut char_coords = input.trim().split(',');
        Self {
            x: char_coords.next().unwrap().trim().parse().unwrap(),
            y: char_coords.next().unwrap().trim().parse().unwrap(),
            z: char_coords.next().unwrap().trim().parse().unwrap(),
        }
    }

    const fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    a: Coords,
    b: Coords
}

impl Line {
    fn new(line: &str) -> Self {
        let mut coords = line.trim().split('@');
        let a = Coords::new(coords.next().unwrap());
        Self {
            a: a.clone(),
            b: a + Coords::new(coords.next().unwrap()),
        }
    }
}
