use std::collections::HashMap;
use std::io::{BufRead as _, BufReader, Read};

type TopoMap = HashMap<(usize, usize), u32>;

fn find_trails(
    start_pos: (usize, usize),
    look_for_num: u32,
    map: &TopoMap,
    already_found_trail_ends: &mut Vec<(usize, usize)>,
) -> (u32, u32) {
    if look_for_num == 10 {
        if !already_found_trail_ends.contains(&start_pos) {
            // We found a unique trail
            already_found_trail_ends.push(start_pos);
            return (1, 1);
        }
        return (0, 1);
    }

    let mut trailhead_score = 0;
    let mut trailhead_rating = 0;

    if start_pos.0 != 0 {
        let upper_cell = (start_pos.0 - 1, start_pos.1);
        if map.get(&upper_cell).is_some_and(|num| *num == look_for_num) {
            let (part_1, part_2) = find_trails(upper_cell, look_for_num + 1, map, already_found_trail_ends);
            trailhead_score += part_1;
            trailhead_rating += part_2;
        }
    }

    if start_pos.1 != 0 {
        let left_cell = (start_pos.0, start_pos.1 - 1);
        if map.get(&left_cell).is_some_and(|num| *num == look_for_num) {
            let (part_1, part_2) = find_trails(left_cell, look_for_num + 1, map, already_found_trail_ends);
            trailhead_score += part_1;
            trailhead_rating += part_2;
        }
    }

    let map_size = (map.len() as f64).sqrt() as usize - 1;
    if start_pos.0 != map_size {
        let down_cell = (start_pos.0 + 1, start_pos.1);
        if map.get(&down_cell).is_some_and(|num| *num == look_for_num) {
            let (part_1, part_2) = find_trails(down_cell, look_for_num + 1, map, already_found_trail_ends);
            trailhead_score += part_1;
            trailhead_rating += part_2;
        }
    }

    if start_pos.1 != map_size {
        let right_cell = (start_pos.0, start_pos.1 + 1);
        if map.get(&right_cell).is_some_and(|num| *num == look_for_num) {
            let (part_1, part_2) = find_trails(right_cell, look_for_num + 1, map, already_found_trail_ends);
            trailhead_score += part_1;
            trailhead_rating += part_2;
        }
    }

    (trailhead_score, trailhead_rating)
}

pub fn solve_both_parts(input: TopoMap) -> (u32, u32) {
    let mut score_sum = 0;
    let mut rating_sum = 0;

    let mut found_trail_ends = Vec::new();
    for ((row_idx, cell_idx), num) in input.iter() {
        if *num == 0 {
            found_trail_ends.clear();
            let (trailhead_score, trailhead_rating) =
                find_trails((*row_idx, *cell_idx), 1, &input, &mut found_trail_ends);
            score_sum += trailhead_score;
            rating_sum += trailhead_rating;
        }
    }

    (score_sum, rating_sum)
}

pub fn parse_input<R: Read>(src: R) -> TopoMap {
    let reader = BufReader::new(src);
    reader
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (row_idx, line)| {
            let line = line.expect("error while reading the file");
            line.chars().enumerate().for_each(|(cell_idx, num)| {
                let num = num.to_digit(10).expect("not a valid number");
                acc.insert((row_idx, cell_idx), num);
            });
            acc
        })
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_both_parts};

    #[test]
    fn correctly_solves_both_parts() {
        let input = "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        let map = parse_input(input.as_bytes());

        let (part_1, part_2) = solve_both_parts(map);

        assert_eq!(part_1, 36);
        assert_eq!(part_2, 81);
    }
}
