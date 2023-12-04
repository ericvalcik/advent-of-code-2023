use std::collections::HashSet;

mod consts;

pub fn calc_points() -> u32 {
    let mut games: Vec<Game>= Vec::new();
    let lines = consts::INPUT.trim().lines();
    for line in lines {
        games.push(parse_line(line));
    }
    for i in 0..games.len() {
        let points = calc_points_for_game(&games[i]);
        for j in i+1..i+(points as usize)+1 {
            games[j].copies += games[i].copies;
        }
    }
    games.into_iter().map(|g| g.copies).sum()
}

#[derive(Debug)]
struct Game {
    copies: u32,
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
    Game { copies: 1, winning, actual }
}

fn calc_points_for_game(game: &Game) -> u32 {
    let mut points: u32 = 0;
    let winning_set: HashSet<u32> = game.winning.clone().into_iter().collect();
    let actual_set: HashSet<u32> = game.actual.clone().into_iter().collect();
    for num in actual_set {
        if winning_set.contains(&num) {
            points += 1;
        }
    }
    points
}