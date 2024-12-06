use std::env;

use day_6::{parse_map_and_start_position, solve_both_parts};

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input = std::fs::read_to_string(input_path).expect("error while reading the input file");

    let (mut map, start_pos) = parse_map_and_start_position(&input);

    let (part_1, part_2) = solve_both_parts(&mut map, start_pos);

    println!(
        "Total distinct positions the guard visited before leaving the map: {}",
        part_1
    );
    println!("Total number of positions where a loop can be formed: {}", part_2);
}
