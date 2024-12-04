const XMAS: &[u8; 4] = b"XMAS";

fn has_word_at_indices<const N: usize>(
    data: &[Vec<u8>],
    word: &[u8; N],
    indices: [(i64, i64); N],
    is_spelled_backwards: bool,
) -> bool {
    for (letter_idx, (i, j)) in indices.into_iter().enumerate() {
        if j < 0 {
            // Can happen when looking for the left diagonal item
            return false;
        }

        let to_be_letter_idx = if is_spelled_backwards {
            word.len() - 1 - letter_idx
        } else {
            letter_idx
        };

        let Some(letter) = data.get(i as usize).and_then(|row| row.get(j as usize)) else {
            // One of the indices is not valid
            return false;
        };

        if *letter != word[to_be_letter_idx] {
            // Wrong letter encountered
            return false;
        }
    }
    true
}

pub fn find_words(data: &[Vec<u8>]) -> (usize, usize) {
    let mut xmas_occurnces = 0;
    let mut x_shaped_mas_occurences = 0;
    for (i, row) in data.iter().enumerate() {
        for (j, character) in row.iter().enumerate() {
            if *character != b'X' && *character != b'S' && *character != b'A' {
                continue;
            }
            if *character == b'A' {
                // Look for an X-shaped MAS

                let is_diagonal_left_spelled_backwards = {
                    let Some(row_idx) = i.checked_sub(1) else {
                        // No row above the current one
                        continue;
                    };
                    let Some(character_idx) = j.checked_sub(1) else {
                        // No character before the current one
                        continue;
                    };
                    data.get(row_idx)
                        .and_then(|row| row.get(character_idx))
                        .is_some_and(|character: &u8| *character == b'S')
                };

                let is_diagonal_right_spelled_backwards = {
                    let Some(row_idx) = i.checked_sub(1) else {
                        // No row above the current one
                        continue;
                    };
                    data.get(row_idx)
                        .and_then(|row| row.get(j + 1))
                        .is_some_and(|character: &u8| *character == b'S')
                };

                let i = i as i64;
                let j = j as i64;
                let diagonal_left_word_indices = [(i - 1, j - 1), (i, j), (i + 1, j + 1)];
                let diagonal_right_word_indices = [(i - 1, j + 1), (i, j), (i + 1, j - 1)];

                if has_word_at_indices(
                    data,
                    &XMAS[1..].try_into().unwrap(),
                    diagonal_left_word_indices,
                    is_diagonal_left_spelled_backwards,
                ) && has_word_at_indices(
                    data,
                    &XMAS[1..].try_into().unwrap(),
                    diagonal_right_word_indices,
                    is_diagonal_right_spelled_backwards,
                ) {
                    x_shaped_mas_occurences += 1;
                }
            } else {
                // Look for an XMAS
                let is_spelled_backwards = *character == b'S';

                // Convert character indices to integer to allow them becoming negative (when checking left diagonal items)
                let i = i as i64;
                let j = j as i64;

                let horizontal_word_indices = [(i, j), (i, j + 1), (i, j + 2), (i, j + 3)];
                let vertical_word_indices = [(i, j), (i + 1, j), (i + 2, j), (i + 3, j)];
                let diagonal_left_word_indices = [(i, j), (i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3)];
                let diagonal_right_word_indices = [(i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)];

                for indices_set in [
                    horizontal_word_indices,
                    vertical_word_indices,
                    diagonal_left_word_indices,
                    diagonal_right_word_indices,
                ] {
                    if has_word_at_indices(data, XMAS, indices_set, is_spelled_backwards) {
                        xmas_occurnces += 1;
                    }
                }
            }
        }
    }

    (xmas_occurnces, x_shaped_mas_occurences)
}

#[cfg(test)]
mod tests {
    use crate::find_words;

    #[test]
    fn correctly_calculates_both_parts() {
        let test_data: &[Vec<u8>] = &[
            b"MMMSXXMASM".to_vec(),
            b"MSAMXMSMSA".to_vec(),
            b"AMXSXMAAMM".to_vec(),
            b"MSAMASMSMX".to_vec(),
            b"XMASAMXAMM".to_vec(),
            b"XXAMMXXAMA".to_vec(),
            b"SMSMSASXSS".to_vec(),
            b"SAXAMASAAA".to_vec(),
            b"MAMMMXMMMM".to_vec(),
            b"MXMXAXMASX".to_vec(),
        ];

        let (part_1, part_2) = find_words(test_data);

        assert_eq!(part_1, 18);
        assert_eq!(part_2, 9)
    }
}
