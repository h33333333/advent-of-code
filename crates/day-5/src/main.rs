use std::env;
use std::fs::File;

use day_5::solve_puzzle;

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input_file = File::open(&input_path).expect("error while reading the file");

    let (part_1, part_2) = solve_puzzle(input_file);

    println!(
        "Sum of middle page numbers in correctly-ordered updates (part 1): {}",
        part_1
    );
    println!(
        "Sum of middle page numbers in updates that were fixed (part 2): {}",
        part_2
    );
}
