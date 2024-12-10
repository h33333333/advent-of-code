use std::env;
use std::fs::File;

use day_10::{parse_input, solve_both_parts};

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input_file = File::open(&input_path).expect("error while reading the file");

    let map = parse_input(input_file);
    let (part_1, part_2) = solve_both_parts(map);

    println!("Sum of scores of all trailheads (part 1): {}", part_1);
    println!("Sum of ratings of all trailheads (part 2): {}", part_2);
}
