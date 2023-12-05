use std::str::Lines;

mod consts;

// right answer = 84206669
pub fn calc_lowest_location() -> u64 {
    let mut lines = consts::INPUT.trim().lines();
    let mut seeds = get_seeds(lines.next().unwrap());
    println!("Seeds: {:?}", seeds.len());
    lines.next();
    let mut maps: Vec<Map> = Vec::new();
    while let Some(map) = get_next_map(&mut lines) {
        maps.push(map);
    }

    for map in &maps {
        println!("Map: {:?}", map.name);
        for seed in &mut seeds {
            let next_val = map.get_next(*seed);
            *seed = next_val;
        }
    }

    let mut lowest_location = 446_744_073_709_551_615_u64;
    for seed in seeds {
        if seed < lowest_location {
            lowest_location = seed;
        }
    }
    lowest_location
}

type Seeds = Vec<u64>;

fn get_seeds(line: &str) -> Seeds {
    let mut seeds: Seeds = Vec::new();
    let mut numbers = line.split(':').last().unwrap().trim().split(' ');
    while let Some(start_str) = numbers.next() {
        let range = numbers.next().unwrap().parse::<u64>().unwrap();
        let start = start_str.parse::<u64>().unwrap();
        for i in start..start+range {
            seeds.push(i);
        }
    }
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
            if rule.source <= value && value < rule.source + rule.range {
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