type Map = Vec<Vec<(bool, bool, bool)>>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MovementDirection {
    Up,
    Right,
    Down,
    Left,
}

impl MovementDirection {
    fn turn_90_degrees_right(self) -> MovementDirection {
        match self {
            MovementDirection::Up => MovementDirection::Right,
            MovementDirection::Right => MovementDirection::Down,
            MovementDirection::Down => MovementDirection::Left,
            MovementDirection::Left => MovementDirection::Up,
        }
    }
}

fn take_a_step(
    map: &Map,
    mut position: (i64, i64),
    mut direction: MovementDirection,
    imaginary_obstacle_pos: Option<(i64, i64)>,
) -> Option<((i64, i64), MovementDirection)> {
    let (new_row, new_cell) = match direction {
        MovementDirection::Up => (position.0 - 1, position.1),
        MovementDirection::Right => (position.0, position.1 + 1),
        MovementDirection::Down => (position.0 + 1, position.1),
        MovementDirection::Left => (position.0, position.1 - 1),
    };
    let (cell, _, _) = map.get(new_row as usize).and_then(|row| row.get(new_cell as usize))?;

    if *cell
        && (imaginary_obstacle_pos.is_none() || imaginary_obstacle_pos.is_some_and(|pos| pos != (new_row, new_cell)))
    {
        position = (new_row, new_cell);
    } else {
        direction = direction.turn_90_degrees_right();
    }

    Some((position, direction))
}

fn has_loop(
    map: &Map,
    starting_position: (i64, i64),
    starting_movement_direction: MovementDirection,
    obstacle_pos: (i64, i64),
) -> bool {
    let mut steps = 0;

    let mut guards = [
        (starting_position, starting_movement_direction, 1),
        (starting_position, starting_movement_direction, 2),
    ];

    loop {
        if guards[0].0 == guards[1].0 && guards[0].1 == guards[1].1 && steps != 0 {
            // We found a loop
            return true;
        }

        for (position, direction, speed) in guards.as_mut() {
            let Some((new_position, new_direction)) = (0..*speed)
                .try_fold((*position, *direction), |(current_pos, direction), _| {
                    take_a_step(map, current_pos, direction, Some(obstacle_pos))
                })
            else {
                // We exited the map, thus there is no loop
                return false;
            };
            *direction = new_direction;
            *position = new_position;
        }

        steps += 1;
    }
}

pub fn parse_map_and_start_position(src: &str) -> (Map, (i64, i64)) {
    let mut guard_start_pos = (0, 0);
    let map = src
        .split("\n")
        .enumerate()
        .map(|(row_idx, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .fold(Vec::with_capacity(line.len()), |mut level, (cell_idx, cell)| {
                    if *cell == b'^' {
                        guard_start_pos = (row_idx, cell_idx);
                    }
                    level.push((*cell != b'#', false, false));
                    level
                })
        })
        .collect::<Vec<_>>();

    (map, (guard_start_pos.0 as i64, guard_start_pos.1 as i64))
}

pub fn solve_both_parts(map: &mut Map, starting_position: (i64, i64)) -> (u32, u32) {
    // Part 1
    let mut distinct_visited_cells = 0;
    // Part 2
    let mut loops = 0;

    let mut guard_pos = starting_position;
    let mut movement_direction = MovementDirection::Up;
    loop {
        let Some(((new_row, new_cell), new_direction)) = take_a_step(map, guard_pos, movement_direction, None) else {
            // Leave map
            break;
        };

        let encountered_obstacle = new_direction != movement_direction;
        if !encountered_obstacle {
            // This is safe to do as we would have returned above if any index was out of bounds
            let (_, is_cell_visited, tested_for_loop) = &mut map[new_row as usize][new_cell as usize];
            // Part 1
            if !*is_cell_visited {
                *is_cell_visited = true;
                distinct_visited_cells += 1;
            }
            // Part 2
            if !*tested_for_loop {
                *tested_for_loop = true;
                if has_loop(map, guard_pos, movement_direction, (new_row, new_cell)) {
                    loops += 1;
                }
            }
        }

        guard_pos = (new_row, new_cell);
        movement_direction = new_direction;
    }

    (distinct_visited_cells, loops)
}

#[cfg(test)]
mod tests {
    use crate::{parse_map_and_start_position, solve_both_parts};

    #[test]
    fn correctly_solves_both_parts() {
        let test_input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let (mut map, start_pos) = parse_map_and_start_position(test_input);

        let (part_1, part_2) = solve_both_parts(&mut map, start_pos);
        assert_eq!(part_1, 41);
        assert_eq!(part_2, 6);
    }
}
