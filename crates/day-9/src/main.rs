use std::{env, fs};

use day_9::{parse_input, part_1, part_2};

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input = fs::read_to_string(input_path).expect("error while readng input");

    let (data, free_slots) = parse_input(&input);

    let part_1 = part_1(data.clone(), free_slots.clone());
    let part_2 = part_2(data, free_slots);

    println!("File checksum (part 1): {}", part_1);
    println!("File checksum (part 2): {}", part_2);
}
