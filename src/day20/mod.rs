use std::collections::{HashMap, VecDeque};

mod consts;

pub fn day20() -> usize {
    let input = consts::INPUT.trim();
    let mut modules: HashMap<String, Module> = HashMap::new();
    for line in input.lines() {
        let first_part = line.split(" -> ").next().unwrap();
        if first_part.contains("broadcaster") {
            modules.insert("broadcaster".to_string(), Module::Broadcaster(Broadcaster::new(line)));
        } else if first_part.starts_with('%') {
            modules.insert(first_part.trim_start_matches('%').to_string(), Module::FlipFlop(FlipFlop::new(line)));
        } else if first_part.starts_with('&') {
            modules.insert(first_part.trim_start_matches('&').to_string(), Module::Conjunction(Conjunction::new(line)));
        }
    }
    for line in input.lines() {
        let name = line.split(" -> ").next().unwrap().trim().trim_start_matches('%').trim_start_matches('&');
        let connections = get_connections(line);
        for connection in connections {
            if let Some(Module::Conjunction(ref mut conjunction)) = modules.get_mut(&connection) {
                conjunction.state.insert(name.to_string(), Pulse::Low);
            }
        }
    }
    let mut low_pulses: usize = 0;
    let mut high_pulses: usize = 0;
    for _ in 0..1000 {
        let mut pulses: VecDeque<(String, Pulse, String)> = VecDeque::new();
        pulses.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
        while !pulses.is_empty() {
            let (from, pulse, to) = pulses.pop_front().unwrap();
            // println!("{} -{} -> {}", from, if pulse == Pulse::Low { "-low" } else { "-high" }, to);
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            }
            if let Some(new_pulses) = process_pulse(&mut modules, &from, &pulse, &to) {
                for new_pulse in new_pulses {
                    pulses.push_back(new_pulse);
                }
            }
        }
    }
    low_pulses * high_pulses
}

fn process_pulse(modules: &mut HashMap<String, Module>, from: &String, pulse: &Pulse, to: &String) -> Option<Vec<(String, Pulse, String)>> {
    if let Some(module) = modules.get_mut(to) {
        return match module {
            Module::FlipFlop(ref mut flip_flow) => {
                match pulse {
                    Pulse::Low => {
                        flip_flow.on = !flip_flow.on;
                        Some(flip_flow.connections.iter().map(|connection| if flip_flow.on { (to.to_string(), Pulse::High, connection.to_string()) } else { (to.to_string(), Pulse::Low, connection.to_string()) }).collect())
                    },
                    Pulse::High => {
                        None
                    }
                }
            }
            Module::Broadcaster(ref broadcaster) => {
                Some(broadcaster.connections.iter().map(|connection| (to.to_string(), pulse.clone(), connection.to_string())).collect())
            }
            Module::Conjunction(ref mut conjuction) => {
                conjuction.state.insert(from.to_string(), pulse.clone());
                if conjuction.state.values().all(|pulse| pulse == &Pulse::High) {
                    return Some(conjuction.connections.iter().map(|connection| (to.to_string(), Pulse::Low, connection.to_string())).collect());
                }
                Some(conjuction.connections.iter().map(|connection| (to.to_string(), Pulse::High, connection.to_string())).collect())
            }
        }
    }
    None
}

enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

fn get_connections(line: &str) -> Vec<String> {
    let mut connections: Vec<String> = Vec::new();
    for connection in line.split(" -> ").last().unwrap().trim().split(", ") {
        connections.push(connection.trim().to_string());
    }
    connections
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    on: bool,
    connections: Vec<String>,
}

impl FlipFlop {
    fn new(line: &str) -> Self {
        Self {
            name: line.split(" -> ").next().unwrap().trim().trim_start_matches('%').trim_start_matches('&').to_string(),
            on: false,
            connections: get_connections(line),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum Pulse {
    Low,
    High
}

struct Conjunction {
    name: String,
    state: HashMap<String, Pulse>,
    connections: Vec<String>,
}

impl Conjunction {
    fn new(line: &str) -> Self {
        Self {
            name: line.split(" -> ").next().unwrap().trim().trim_start_matches('%').trim_start_matches('&').to_string(),
            state: HashMap::new(),
            connections: get_connections(line),
        }
    }
}

struct Broadcaster {
    connections: Vec<String>,
}

impl Broadcaster {
    fn new(line: &str) -> Self {
        Self {
            connections: get_connections(line),
        }
    }
}
