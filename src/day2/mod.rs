mod consts;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

pub fn calc_number() -> Result<u32, ()> {
    let mut sum_powers: u32 = 0;
    let lines = consts::INPUT.lines();
    for line in lines {
        let game = parse_game(line)?;
        let cubes = get_min_amount_of_cubes(&game);
        sum_powers += cubes.red * cubes.green * cubes.blue;
    }
    Ok(sum_powers)
}

struct Turn {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    turns: Vec<Turn>,
}

fn get_min_amount_of_cubes(game: &Game) -> Turn {
    let mut min_turn = Turn {
        red: 0,
        green: 0,
        blue: 0,
    };
    for turn in &game.turns {
        if turn.red > min_turn.red {
            min_turn.red = turn.red;
        }
        if turn.green > min_turn.green {
            min_turn.green = turn.green;
        }
        if turn.blue > min_turn.blue {
            min_turn.blue = turn.blue;
        }
    }
    min_turn
}

fn validate_game(game: &Game) -> bool {
    for turn in &game.turns {
        if turn.red > RED_MAX || turn.green > GREEN_MAX || turn.blue > BLUE_MAX {
            return false;
        }
    }
    true
}

fn parse_game(input: &str) -> Result<Game, ()> {
    let mut game = Game {
        id: 0,
        turns: vec![],
    };
    let mut split_by_colon = input.split(":");
    let game_str = split_by_colon.next().unwrap().split(" ");
    game.id = game_str.last().unwrap().parse().ok().unwrap();
    let mut turn_strings = split_by_colon.next().unwrap().trim().split(";");
    for turn_str in turn_strings {
        let turn = parse_turn(turn_str.trim())?;
        game.turns.push(turn);
    }
    Ok(game)
}

fn parse_turn(input: &str) -> Result<Turn, ()> {
    let mut turn = Turn {
        red: 0,
        green: 0,
        blue: 0,
    };
    let split_by_comma = input.split(",");
    for color_str in split_by_comma {
        let mut color_str_split = color_str.trim().split(" ");
        let number = color_str_split.next().unwrap().parse().ok().unwrap();
        let color = color_str_split.next().unwrap();
        match color {
            "red" => turn.red = number,
            "green" => turn.green = number,
            "blue" => turn.blue = number,
            _ => return Err(()),
        }
    }
    Ok(turn)
}
