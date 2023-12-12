use crate::day12::{consts, prepare_part2};

pub fn count_part2() -> usize {
    let lines = consts::INPUT.trim().lines();
    let mut result: usize = 0;
    for line in lines {
        let mut first_part: Vec<char> = line.split(' ').next().unwrap().trim().chars().collect::<Vec<char>>();
        let mut second_part = line.split(' ').last().unwrap().trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        prepare_part2(&mut first_part, &mut second_part);
        let line_count = count(&first_part, 0, &second_part, 0);
        println!("{line_count:2} {} {}", first_part.iter().collect::<String>(), second_part.iter().map(std::string::ToString::to_string).collect::<String>());
        result += line_count;
    }
    result
}

fn count(line: &Vec<char>, line_index: usize, nums: &Vec<usize>, nums_index: usize) -> usize {
    if line_index >= line.len() {
        return usize::from(nums_index == nums.len());
    }
    if nums_index >= nums.len() {
        return usize::from(line[line_index..].iter().all(|x| *x == '.' || *x == '?'));
    }

    let mut result: usize = 0;
    if line[line_index] != '#' {
        result += count(line, line_index + 1, nums, nums_index);
    }
    if line[line_index] != '.'
        && nums[nums_index] + line_index <= line.len()
        && line[line_index..nums[nums_index] + line_index].iter().all(|x| *x == '#' || *x == '?')
        && (line_index + nums[nums_index] == line.len()
            || line[line_index + nums[nums_index]] != '#') {
        result += count(line, line_index + nums[nums_index] + 1, nums, nums_index + 1);
    }

    result
}