use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_perimeter_for_region(region: &HashSet<(usize, usize)>) -> usize {
    let mut perimeter = 0;
    for (row_idx, col_idx) in region {
        let mut cell_perimeter = 4;
        if *row_idx > 0 && region.contains(&(row_idx - 1, *col_idx)) {
            cell_perimeter -= 1;
        }
        if *col_idx > 0 && region.contains(&(*row_idx, col_idx - 1)) {
            cell_perimeter -= 1;
        }
        if region.contains(&(row_idx + 1, *col_idx)) {
            cell_perimeter -= 1;
        }
        if region.contains(&(*row_idx, col_idx + 1)) {
            cell_perimeter -= 1;
        }
        perimeter += cell_perimeter;
    }

    perimeter
}

fn main() {
    let input_path = env::args().nth(1).expect("input file path is missing");
    let input_file = File::open(&input_path).expect("error while reading the file");
    let reader = BufReader::new(input_file);

    let grid = reader
        .lines()
        .map(|line| {
            let line = line.expect("failed to read the file");
            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut regions: HashMap<i32, HashSet<(usize, usize)>> = HashMap::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut item_queue = VecDeque::from(vec![(0, 0)]);
    let mut current_region = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, crop) in row.iter().enumerate() {
            if seen.contains(&(row_idx, col_idx)) {
                continue;
            }

            let region = regions.entry(current_region).or_default();
            let mut region_queue = VecDeque::from(vec![(row_idx, col_idx)]);
            while let Some((row_idx, col_idx)) = region_queue.pop_front() {
                if &grid[row_idx][col_idx] != crop || seen.contains(&(row_idx, col_idx)) {
                    item_queue.push_back((row_idx, col_idx));
                    continue;
                }

                if row_idx > 0 {
                    region_queue.push_back((row_idx - 1, col_idx))
                }
                if col_idx > 0 {
                    region_queue.push_back((row_idx, col_idx - 1))
                }
                if row_idx < grid.len() - 1 {
                    region_queue.push_back((row_idx + 1, col_idx))
                }
                if col_idx < grid[0].len() - 1 {
                    region_queue.push_back((row_idx, col_idx + 1))
                }

                seen.insert((row_idx, col_idx));
                region.insert((row_idx, col_idx));
            }

            current_region += 1;
        }
    }

    dbg!(regions
        .values()
        .map(|region| region.len() * calculate_perimeter_for_region(region))
        .sum::<usize>());
}
