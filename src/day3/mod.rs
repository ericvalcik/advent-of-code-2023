use std::collections::{HashSet};

mod consts;

type Schema = Vec<Vec<u8>>;
type StarCords = (usize, usize);

pub fn compute_number() -> u32 {
    let mut array: Schema = consts::INPUT.trim().lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();
    let numbers = find_all_numbers(&array);
    let motor_numbers = numbers.iter().filter(|n| is_motor_part(&array, n)).collect::<Vec<_>>();
    motor_numbers.iter().map(|n| get_number_value(&array, n)).sum()
}

pub fn compute_gear_ratios() -> u32 {
    let mut array: Schema = consts::INPUT.trim().lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();
    let mut numbers = find_all_numbers(&array);
    let star_numbers = numbers.into_iter().filter_map(|n| is_star_part(&array, n)).collect::<Vec<_>>();
    let mut stars: HashSet<StarCords> = star_numbers.iter().map(|n| n.star.unwrap()).collect();
    let mut sum: u32 = 0;
    for star in &stars {
        let mut ratio: u32 = 1;
        let mut count: u32 = 0;
        for n in &star_numbers {
            if n.star.unwrap() == *star {
                ratio *= get_number_value(&array, n);
                count += 1;
            }
        }
        if count == 2 {
            sum += ratio;
        }
    }
    sum
}

#[derive(Debug)]
struct Number {
    row: usize,
    col_start: usize,
    col_end: usize,
    star: Option<StarCords>
}

fn find_all_numbers(schema: &Schema) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    for (row, line) in schema.iter().enumerate() {
        let mut index: usize = 0;
        let mut number_start: Option<usize> = None;
        while index < line.len() {
            if line[index].is_ascii_digit() {
                if number_start.is_none() {
                    number_start = Some(index);
                }
            } else if let Some(start) = number_start {
                numbers.push(Number {
                    row,
                    col_start: start,
                    col_end: index - 1,
                    star: None
                });
                number_start = None;
            }
            index += 1;
        }
        if let Some(start) = number_start {
            numbers.push(Number {
                row,
                col_start: start,
                col_end: index - 1,
                star: None
            });
        }
    }
    numbers
}

fn is_star_part(schema: &Schema, mut number: Number) -> Option<Number> {
    for row in usize_minus_one(number.row)..number.row+2 {
        for col in usize_minus_one(number.col_start)..number.col_end+2 {
            if row >= schema.len() || col >= schema[row].len() {
                continue;
            }
            if schema[row][col] == b'*' {
                number.star = Some((row, col));
                return Some(number);
            }
        }
    }
    None
}

fn is_motor_part(schema: &Schema, number: &Number) -> bool {
    for row in usize_minus_one(number.row)..number.row+2 {
        for col in usize_minus_one(number.col_start)..number.col_end+2 {
            if row >= schema.len() || col >= schema[row].len() {
                continue;
            }
            if !schema[row][col].is_ascii_digit() && schema[row][col] != b'.' {
                return true;
            }
        }
    }
    false
}

fn get_number_value(schema: &Schema, number: &Number) -> u32 {
    let mut value: u32 = 0;
    for col in number.col_start..number.col_end+1 {
        value *= 10;
        value += u32::from(schema[number.row][col] - b'0');
    }
    value
}

const fn usize_minus_one(a: usize) -> usize {
    if a == 0 {
        return a;
    }
    a-1
}