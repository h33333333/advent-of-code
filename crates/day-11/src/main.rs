use std::{env, fs};

use day_11::{parse_input, solve_both_parts};

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input = fs::read_to_string(input_path).expect("error while reading the input");

    let map = parse_input(&input);

    let (part_1, part_2) = solve_both_parts(map);

    println!("Number of stones after 25 bliks: {}", part_1);
    println!("Number of stones after 75 bliks: {}", part_2);
}
