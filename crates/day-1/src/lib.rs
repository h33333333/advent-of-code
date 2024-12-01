use std::collections::HashMap;
use std::io::{BufRead as _, BufReader, Read};

/// Reads the input data and splits it into two sorted lists
pub fn read_input_into_lists<R: Read>(
    reader: R,
    mut occurence_map: Option<&mut HashMap<i64, i64>>,
) -> (Vec<i64>, Vec<i64>) {
    let (mut list_1, mut list_2) =
        BufReader::new(reader)
            .lines()
            .fold((Vec::new(), Vec::new()), |(mut list_1, mut list_2), line| {
                let line = line.expect("failed to read a line");
                // Columns are delimited using 3 spaces
                let mut parts = line.split("   ");

                let list_1_item = parts
                    .next()
                    .expect("invalid input format: first list item is missing")
                    .parse::<i64>()
                    .expect("item is not a valid integer");
                list_1.push(list_1_item);

                let list_2_item = parts
                    .next()
                    .expect("invalid input format: second list item is missing")
                    .parse::<i64>()
                    .expect("item is not a valid integer");
                list_2.push(list_2_item);

                // Part 2: similarity score
                if let Some(occurence_map) = occurence_map.as_mut() {
                    *occurence_map.entry(list_2_item).or_default() += 1;
                }

                (list_1, list_2)
            });

    // Sort both lists
    list_1.sort();
    list_2.sort();

    (list_1, list_2)
}

/// Calculates the sum of distances between corresponding items in two lists and the similarity score
/// using the provided occurence map
pub fn calculate_distance_and_similarity(
    list_1: &[i64],
    list_2: &[i64],
    occurence_map: Option<&HashMap<i64, i64>>,
) -> (i64, i64) {
    // Find the sum of distances and the similarity score
    list_1
        .iter()
        .enumerate()
        .fold((0, 0), |(distance, similarity), (idx, item_1)| {
            let distance = distance + (item_1 - list_2[idx]).abs();
            let similarity = similarity
                + item_1
                    * occurence_map
                        .and_then(|hashmap| hashmap.get(item_1).copied())
                        .unwrap_or_default();

            (distance, similarity)
        })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{calculate_distance_and_similarity, read_input_into_lists};

    const TEST_INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    #[test]
    #[should_panic(expected = "invalid input format: second list item is missing")]
    fn panics_on_invalid_input() {
        let input = "1";
        read_input_into_lists(input.as_bytes(), None);
    }

    #[test]
    #[should_panic(expected = "item is not a valid integer")]
    fn panics_on_non_integer_data() {
        let input = "bad   input";
        read_input_into_lists(input.as_bytes(), None);
    }

    #[test]
    fn lists_are_parsed_correctly() {
        let (list_1, list_2) = read_input_into_lists(TEST_INPUT.as_bytes(), None);

        assert_eq!(list_1, [1, 2, 3, 3, 3, 4]);
        assert_eq!(list_2, [3, 3, 3, 4, 5, 9]);
    }

    #[test]
    fn correctly_calculates_occurences() {
        let mut occurence_map = HashMap::new();
        let _ = read_input_into_lists(TEST_INPUT.as_bytes(), Some(&mut occurence_map));

        assert_eq!(occurence_map.get(&1).copied().unwrap_or_default(), 0);
        assert_eq!(occurence_map.get(&2).copied().unwrap_or_default(), 0);
        assert_eq!(occurence_map.get(&3).copied().unwrap_or_default(), 3);
        assert_eq!(occurence_map.get(&4).copied().unwrap_or_default(), 1);
    }

    #[test]
    fn correctly_calculates_distance_and_similarity() {
        let mut occurence_map = HashMap::new();
        let (list_1, list_2) = read_input_into_lists(TEST_INPUT.as_bytes(), Some(&mut occurence_map));

        let (distance, similarity) = calculate_distance_and_similarity(&list_1, &list_2, Some(&occurence_map));

        assert_eq!(distance, 11);
        assert_eq!(similarity, 31);
    }
}
