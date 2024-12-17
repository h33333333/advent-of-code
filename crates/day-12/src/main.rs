use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_perimeter_for_region(region: &HashSet<(isize, isize)>) -> usize {
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

fn calculate_corners_for_region(region: &HashSet<(isize, isize)>) -> usize {
    let mut corners = 0;
    for (row_idx, col_idx) in region.iter().copied() {
        let mut cell_corners = 0;

        let all_corners = [
            // Top left
            [
                region.contains(&(row_idx - 1, col_idx)),
                region.contains(&(row_idx - 1, col_idx - 1)),
                region.contains(&(row_idx, col_idx - 1)),
            ],
            // Bottom left
            [
                region.contains(&(row_idx, col_idx - 1)),
                region.contains(&(row_idx + 1, col_idx - 1)),
                region.contains(&(row_idx + 1, col_idx)),
            ],
            // Bottom right
            [
                region.contains(&(row_idx + 1, col_idx)),
                region.contains(&(row_idx + 1, col_idx + 1)),
                region.contains(&(row_idx, col_idx + 1)),
            ],
            // Top right
            [
                region.contains(&(row_idx, col_idx + 1)),
                region.contains(&(row_idx - 1, col_idx + 1)),
                region.contains(&(row_idx - 1, col_idx)),
            ],
        ];
        for corner_idxs in all_corners {
            if corner_idxs == [true, false, true]
                || corner_idxs == [false, false, false]
                || corner_idxs == [false, true, false]
            {
                cell_corners += 1;
            }
        }

        corners += cell_corners
    }

    corners
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

    let mut regions: HashMap<i32, HashSet<(isize, isize)>> = HashMap::new();
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
                region.insert((row_idx as isize, col_idx as isize));
            }

            current_region += 1;
        }
    }

    println!(
        "part 1: {}",
        regions
            .values()
            .map(|region| region.len() * calculate_perimeter_for_region(region))
            .sum::<usize>()
    );

    println!(
        "part 2: {}",
        regions
            .values()
            .map(|region| { region.len() * calculate_corners_for_region(region) })
            .sum::<usize>()
    );
}
