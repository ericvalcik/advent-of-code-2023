mod consts;

pub fn run_hash_algorithm() -> usize {
    let sequences = consts::INPUT.trim().split(',');
    let mut result: usize = 0;
    for seq in sequences {
        result += process_sequence(seq);
    }
    result
}

fn process_sequence(seq: &str) -> usize {
    let mut current_value: usize = 0;
    for char in seq.as_bytes() {
        current_value += *char as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}