mod consts;

pub fn calc_numbers() -> u32 {
    consts::INPUT.split("\n").map(|x| line_to_number(x)).map(|x| {
        println!("{}", x);
        x
    }).sum()
}

fn line_to_number(line: &str) -> u32 {
    println!("{}", line);
    let chars = line.chars();
    let len = chars.clone().count();
    let mut x: u32 = 0;
    for i in 0..len {
        if chars.clone().nth(i).unwrap().is_digit(10) {
            x = chars.clone().nth(i).unwrap().to_digit(10).unwrap() * 10;
            break;
        }
        if let Some(y) = is_word_number(true, &line[i..]) {
            x = y * 10;
            break;
        }
    }
    for i in 0..len {
        if chars.clone().nth(len - i - 1).unwrap().is_digit(10) {
            x += chars.clone().nth(len - i - 1).unwrap().to_digit(10).unwrap();
            break;
        }
        if let Some(y) = is_word_number(false, &line[..len - i]) {
            x += y;
            break;
        }
    }
    x
}

const NUMS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9)
];

fn is_word_number(start: bool, string: &str) -> Option<u32> {
    for (word, num) in NUMS.iter() {
        if start && string.starts_with(word) {
            return Some(*num);
        }
        if !start && string.ends_with(word) {
            return Some(*num);
        }
    }
    None
}
