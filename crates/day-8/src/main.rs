use std::env;
use std::fs::File;

use day_8::{parse, solve};

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input_file = File::open(&input_path).expect("error while reading the file");

    let (antennas, map_bounds) = parse(input_file);

    let part_1 = solve(&antennas, map_bounds, false);
    let part_2 = solve(&antennas, map_bounds, true);

    println!(
        "Number of unique locations within the bounds of the map that contain an antinode (part 1): {}",
        part_1
    );

    println!(
        "Number of unique locations within the bounds of the map that contain an antinode (part 2): {}",
        part_2
    );
}
