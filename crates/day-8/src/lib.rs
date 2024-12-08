use std::collections::{HashMap, HashSet};
use std::io::{BufRead as _, BufReader, Read};

type Antennas = HashMap<u8, Vec<(i64, i64)>>;

pub fn solve(antennas: &Antennas, map_bounds: (i64, i64), part_2: bool) -> usize {
    let mut antinodes = HashSet::<(i64, i64)>::new();
    for (_, antenna_positions) in antennas.iter() {
        for (idx, position) in antenna_positions.iter().enumerate() {
            for second_antenna_position in antenna_positions[idx + 1..].iter() {
                if part_2 {
                    // Part 2: Antinodes always appear at antennas positions unless there is no second antenna
                    antinodes.insert(*position);
                    antinodes.insert(*second_antenna_position);
                }

                let distance = (
                    second_antenna_position.0 - position.0,
                    second_antenna_position.1 - position.1,
                );

                let mut topmost_antinode_pos = (position.0 - distance.0, position.1 - distance.1);
                while topmost_antinode_pos.0 >= 0
                    && topmost_antinode_pos.0 < map_bounds.0
                    && topmost_antinode_pos.1 >= 0
                    && topmost_antinode_pos.1 < map_bounds.1
                {
                    antinodes.insert(topmost_antinode_pos);
                    if !part_2 {
                        // Run only once in part 2
                        break;
                    } else {
                        // Proceed to the next antinode in a line
                        topmost_antinode_pos =
                            (topmost_antinode_pos.0 - distance.0, topmost_antinode_pos.1 - distance.1);
                    }
                }

                let mut bottommost_antinode_pos = (
                    second_antenna_position.0 + distance.0,
                    second_antenna_position.1 + distance.1,
                );
                while bottommost_antinode_pos.0 >= 0
                    && bottommost_antinode_pos.0 < map_bounds.0
                    && bottommost_antinode_pos.1 >= 0
                    && bottommost_antinode_pos.1 < map_bounds.1
                {
                    antinodes.insert(bottommost_antinode_pos);
                    if !part_2 {
                        // Run only once in part 2
                        break;
                    } else {
                        // Proceed to the next antinode in a line
                        bottommost_antinode_pos = (
                            bottommost_antinode_pos.0 + distance.0,
                            bottommost_antinode_pos.1 + distance.1,
                        );
                    }
                }
            }
        }
    }

    antinodes.len()
}

pub fn parse<R: Read>(src: R) -> (Antennas, (i64, i64)) {
    let reader = BufReader::new(src);

    let mut antennas: Antennas = HashMap::new();
    let mut map_bounds = (0, 0);
    for (row, line) in reader.lines().enumerate() {
        let line = line.expect("error while reading a line");

        line.as_bytes().iter().enumerate().for_each(|(cell, value)| {
            if *value != b'.' {
                antennas.entry(*value).or_default().push((row as i64, cell as i64));
            }
            if cell >= map_bounds.1 as usize {
                map_bounds.1 += 1;
            }
        });

        map_bounds.0 += 1;
    }

    (antennas, map_bounds)
}

#[cfg(test)]
mod tests {
    use crate::{parse, solve};

    #[test]
    fn correctly_solves_both_parts() {
        let test_input = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";

        let (antennas, map_bounds) = parse(test_input.as_bytes());

        let part_1 = solve(&antennas, map_bounds, false);
        assert_eq!(part_1, 14);

        let part_2 = solve(&antennas, map_bounds, true);
        assert_eq!(part_2, 34);
    }
}
