use std::collections::HashSet;

mod consts;

pub fn calc_points() -> u32 {
    let mut points: u32 = 0;
    let lines = consts::INPUT.trim().lines();
    for line in lines {
        let game = parse_line(line);
        points += calc_points_for_game(game);
    }
    points
}

#[derive(Debug)]
struct Game {
    winning: Vec<u32>,
    actual: Vec<u32>,
}

fn parse_line(line: &str) -> Game {
    let mut winning: Vec<u32> = Vec::new();
    let mut actual: Vec<u32> = Vec::new();
    let numbers = line.split(':').last().unwrap().trim();
    let winning_str = numbers.split('|').next().unwrap().trim();
    for num in winning_str.split(' ') {
        match num.trim().parse::<u32>() {
            Ok(n) => winning.push(n),
            Err(_) => continue,
        }
    }
    let actual_str = numbers.split('|').last().unwrap().trim();
    for num in actual_str.split(' ') {
        match num.trim().parse::<u32>() {
            Ok(n) => actual.push(n),
            Err(_) => continue,
        }
    }
    Game { winning, actual }
}

fn calc_points_for_game(game: Game) -> u32 {
    let mut points: u32 = 0;
    let winning_set: HashSet<u32> = game.winning.into_iter().collect();
    let actual_set: HashSet<u32> = game.actual.into_iter().collect();
    for num in actual_set {
        if winning_set.contains(&num) {
            if (points == 0) {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }
    points
}