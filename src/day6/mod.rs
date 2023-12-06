mod consts;

pub fn calc_options() -> u64 {
    let input = consts::INPUT.trim();
    let races = get_races(input);
    let mut options: u64 = 1;
    for race in races {
        options *= calc_race_win_option(race);
    }
    options
}

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

fn get_races(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let mut fst_line = lines.next().unwrap().trim().split_whitespace();
    fst_line.next();
    let mut snd_line = lines.next().unwrap().trim().split_whitespace();
    snd_line.next();
    let mut races: Vec<Race> = Vec::new();
    for time_str in fst_line {
        let dist_str = snd_line.next().unwrap();
        let race = Race {
            time: time_str.trim().parse::<u64>().unwrap(),
            record_distance: dist_str.trim().parse::<u64>().unwrap(),
        };
        races.push(race);
    }
    races
}

fn calc_race_win_option(race: Race) -> u64 {
    let mut winning_options: u64 = 0;
    for speed in 1..race.time {
        if speed * (race.time - speed) > race.record_distance {
            winning_options += 1;
        }
    }
    winning_options
}