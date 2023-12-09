mod consts;

pub fn extrapolate_numbers() -> i32 {
    let mut lines = consts::INPUT.trim().lines();
    let mut sum: i32 = 0;
    for line in lines {
        let mut vec: Vec<i32> = Vec::new();
        for num in line.split_whitespace() {
            vec.push(num.parse::<i32>().unwrap());
        }
        sum += predict_number(&vec);
    }
    sum
}

fn predict_number(vec: &Vec<i32>) -> i32 {
    if is_all_zeroes(vec) {
        return 0;
    }
    let mut nested_vec: Vec<i32> = Vec::new();
    for i in 0..vec.len()-1 {
        nested_vec.push(vec[i + 1] - vec[i]);
    }
    let nested_next_number = predict_number(&nested_vec);
    vec[0] - nested_next_number
}

fn is_all_zeroes(vec: &Vec<i32>) -> bool {
    for num in vec {
        if *num != 0 {
            return false;
        }
    }
    true
}