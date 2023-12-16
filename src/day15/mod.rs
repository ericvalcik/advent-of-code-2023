mod consts;

pub fn run_hash_algorithm() -> usize {
    let sequences = consts::INPUT.trim().split(',');
    let mut boxes: Vec<Box> = vec![Box::new(); 255];
    for seq in sequences {
        let index = process_sequence(seq);
        let label = get_label(seq);
        if seq.contains('-') {
            boxes[index].lens.retain(|lens| lens.label != label);
        } else {
            let focal_length = seq.split('=').last().unwrap().parse::<usize>().unwrap();
            if boxes[index].lens.iter().any(|lens| lens.label == label) {
                boxes[index].lens.iter_mut().find(|lens| lens.label == label).unwrap().focal_length = focal_length;
            } else {
                boxes[index].lens.push(Lens {
                    label: label.to_string(),
                    focal_length
                });
            }
        }
        // println!("STEP: {seq}");
        // print_boxes(&boxes);
    }
    // calculate the focusing power
    let mut result: usize = 0;
    for (i, box_) in boxes.iter().enumerate() {
        for (lens_index, lens) in box_.lens.iter().enumerate() {
            result += (i + 1) * lens.focal_length * (lens_index + 1);
        }
    }
    result
}

#[derive(Debug, Clone)]
struct Box {
    lens: Vec<Lens>
}

impl Box {
    const fn new() -> Box {
        Box {
            lens: Vec::new()
        }
    }

}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn process_sequence(seq: &str) -> usize {
    let mut current_value: usize = 0;
    for char in seq.as_bytes() {
        if *char == b'-' || *char == b'=' {
            break;
        }
        current_value += *char as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn get_label(seq: &str) -> &str {
    let mut index: usize = 0;
    for char in seq.as_bytes() {
        if *char == b'-' || *char == b'=' {
            break;
        }
        index += 1;
    }
    &seq[..index]
}

fn print_boxes(boxes: &[Box]) {
    for (i, box_) in boxes.iter().enumerate() {
        if box_.lens.is_empty() {
            continue;
        }
        print!("Box {}: ", i);
        for lens in &box_.lens {
            print!("[{} {}] ", lens.label, lens.focal_length);
        }
        println!();
    }
}