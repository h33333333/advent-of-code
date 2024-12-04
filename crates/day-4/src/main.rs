use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use day_4::find_words;

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input_file = File::open(&input_path).expect("error while reading the file");
    let reader = BufReader::new(input_file);

    let mut data: Vec<Vec<u8>> = Vec::new();
    reader.lines().for_each(|line| {
        let line = line.expect("error while reading a line from the file");
        data.push(line.into_bytes());
    });

    let (part_1, part_2) = find_words(&data);
    println!("XMAS appears {} times", part_1);
    println!("X-shaped MASes appear {} times", part_2);
}
