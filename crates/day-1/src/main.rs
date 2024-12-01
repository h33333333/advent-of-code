use std::collections::HashMap;
use std::env;
use std::fs::File;

use day_1::{calculate_distance_and_similarity, read_input_into_lists};

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input_file = File::open(input_path).expect("error while reading the file");

    // Part 2: similarity score
    let mut occurence_map = HashMap::new();
    let (list_1, list_2) = read_input_into_lists(&input_file, Some(&mut occurence_map));
    let (distance_sum, similarity_score) = calculate_distance_and_similarity(&list_1, &list_2, Some(&occurence_map));

    println!(
        "Total distance between the left list and the right list: {}\nSimilarity score: {}",
        distance_sum, similarity_score
    )
}
