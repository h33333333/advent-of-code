use std::env;

use day_3::process_instructions;

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let data = std::fs::read_to_string(input_path).expect("error while reading the file");

    let part_1 = process_instructions(&data, false);
    let part_2 = process_instructions(&data, true);

    println!("Sum of results of all mul instructions: {}", part_1);
    println!(
        "Sum of results of all mul instructions with enable/disable instructions: {}",
        part_2
    );
}
