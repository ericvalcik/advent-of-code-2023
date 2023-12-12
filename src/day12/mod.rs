mod consts;
pub mod different_solution;

pub fn count_configurations() -> usize {
    let lines = consts::EXAMPLE1.trim().lines();
    let mut count: usize = 0;
    // lines.next();
    // let line = lines.next().unwrap();
    for line in lines {
        let mut first_part: Vec<char> = line.split(' ').next().unwrap().trim().chars().collect::<Vec<char>>();
        let mut second_part = line.split(' ').last().unwrap().trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        prepare_part2(&mut first_part, &mut second_part);
        let line_count = init_guessing(&mut first_part, &second_part);
        println!("{line_count:2} {} {}", first_part.iter().collect::<String>(), second_part.iter().map(std::string::ToString::to_string).collect::<String>());
        count += line_count;
    }
    count
}

fn prepare_part2(line: &mut Vec<char>, nums: &mut Vec<usize>) {
    let line_copy = line.clone();
    let nums_copy = nums.clone();
    for _ in 0..4 {
        line.push('?');
        line.append(&mut line_copy.clone());
        nums.append(&mut nums_copy.clone());
    }
}

fn init_guessing(line: &mut Vec<char>, snd_part: &Vec<usize>) -> usize {
    guess_configuration(line, snd_part, snd_part.iter().sum::<usize>(), 0)
}

fn guess_configuration(line: &mut Vec<char>, snd_part: &Vec<usize>, hashes_should_be: usize, index: usize) -> usize {
    // println!("{line:?}");
    let count_hashes = line.iter().filter(|x| **x == '#').count();
    if count_hashes == hashes_should_be {
        return usize::from(validate_configuration(line, snd_part));
    }
    let mut count: usize = 0;
    for i in index..line.len() {
        if line[i] == '?' {
            line[i] = '#';
            count += guess_configuration(line, snd_part, hashes_should_be, i + 1);
            line[i] = '?';
        }
    }
    count
}

fn validate_configuration(line: &Vec<char>, snd_part: &Vec<usize>) -> bool {
    let mut created_vec = Vec::new();
    let mut count: usize = 0;
    for char in line {
        if *char == '#' {
            count += 1;
        }
        // assume ? is .
        if *char == '.' || *char == '?' {
            if count > 0 {
                created_vec.push(count);
            }
            count = 0;
        }
    }
    if count > 0 {
        created_vec.push(count);
    }
    created_vec == *snd_part
}