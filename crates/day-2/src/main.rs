use std::env;
use std::fs::File;

use day_2::find_safe_reports;

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input_file = File::open(&input_path).expect("error while reading the file");

    let (part_1, part_2) = find_safe_reports(&input_file);

    println!("Total number of safe reports for part 1: {}", part_1);
    println!("Total number of safe reports for part 2: {}", part_2);
}
