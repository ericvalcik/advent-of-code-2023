use std::str::Lines;

mod consts;

pub fn calc_lowest_location() -> u64 {
    let mut lines = consts::INPUT.trim().lines();
    let seeds = get_seeds(lines.next().unwrap());
    lines.next();
    let mut maps: Vec<Map> = Vec::new();
    while let Some(map) = get_next_map(&mut lines) {
        maps.push(map);
    }

    let mut lowest_location = 446_744_073_709_551_615_u64;
    for seed in seeds {
        let final_value = get_final_value_for_seed(seed, &maps);
        if final_value < lowest_location {
            lowest_location = final_value;
        }
    }
    lowest_location
}

type Seeds = Vec<u64>;

fn get_seeds(line: &str) -> Seeds {
    let mut seeds: Seeds = Vec::new();
    line.split(':').last().unwrap().trim().split(' ').for_each(|s| {
        seeds.push(s.trim().parse::<u64>().unwrap());
    });
    seeds
}

#[derive(Debug)]
struct Rule {
    source: u64,
    destination: u64,
    range: u64,
}

#[derive(Debug)]
struct Map {
    name: String,
    rules: Vec<Rule>,
}

impl Map {
    pub fn new() -> Self {
        Self { rules: Vec::new(), name: String::new() }
    }

    pub fn get_next(&self, value: u64) -> u64 {
        for rule in &self.rules {
            println!("{} <= {} < {} + {}", rule.source, value, rule.source, rule.range);
            if rule.source <= value && value < rule.source + rule.range {
                println!("{} + ({} - {})", rule.destination, rule.source, value);
                return rule.destination + (value - rule.source);
            }
        }
        value
    }

}

fn get_next_map(lines: &mut Lines) -> Option<Map> {
    let mut map: Map = Map::new();
    let first_line = lines.next();
    first_line?;
    map.name = first_line.unwrap().trim().split(' ').next().unwrap().trim().to_string();
    for line in lines {
        if line.trim() == "" {
            break;
        }
        let mut numbers = line.split(' ');
        let destination = numbers.next().unwrap().parse::<u64>().unwrap();
        let source = numbers.next().unwrap().parse::<u64>().unwrap();
        let range = numbers.next().unwrap().parse::<u64>().unwrap();
        map.rules.push(Rule {source, destination, range });
    }
    Some(map)
}

fn get_final_value_for_seed(seed: u64, maps: &Vec<Map>) -> u64 {
    let mut value = seed;
    for map in maps {
        value = map.get_next(value);
    }
    value
}