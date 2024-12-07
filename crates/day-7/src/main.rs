use std::env;
use std::fs::File;

use day_7::solve_both_parts;

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input_file = File::open(&input_path).expect("error while reading the file");

    let [part_1, part_2] = solve_both_parts(input_file);

    println!("Total calibration result of all possibly true equations: {}", part_1);
    println!(
        "Total calibration result of all possibly true equations (with concat): {}",
        part_2
    );
}
