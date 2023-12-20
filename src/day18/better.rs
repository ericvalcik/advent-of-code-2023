use crate::day18::consts;

pub fn get_area() -> i64 {
    let input = consts::INPUT.trim();
    let (points, b) = get_points(input);
    let a = get_a_sholace(&points);
    a + 1 + b / 2
}

fn get_a_sholace(points: &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    for i in 0..points.len()-1 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];
        sum += (y1 + y2) * (x1 - x2);
    }
    sum / 2
}

fn get_points(input: &str) -> (Vec<(i64, i64)>, i64) {
    let mut vec = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    vec.push((x, y));
    for line in input.lines() {
        let mut split_line = line.split(" ");
        let dir = split_line.next().unwrap();
        let num = split_line.next().unwrap().parse::<i64>().unwrap();
        count += num;
        match dir {
            "R" => x += num,
            "L" => x -= num,
            "U" => y -= num,
            "D" => y += num,
            _ => panic!("Unknown direction: {dir}")
        }
        vec.push((x, y));
    }
    (vec, count)
}