mod consts;

pub fn count_configurations() -> usize {
    let mut lines = consts::INPUT.trim().lines();
    let mut count: usize = 0;
    // lines.next();
    // let line = lines.next().unwrap();
    for line in lines {
        let mut first_part: Vec<char> = line.split(' ').next().unwrap().trim().chars().collect::<Vec<char>>();
        let second_part = line.split(' ').last().unwrap().trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let line_count = init_guessing(&mut first_part, &second_part);
        println!("{line_count:2} {line}");
        count += line_count;
    }
    count
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