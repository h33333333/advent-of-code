use std::collections::HashMap;
use std::io::{BufRead as _, BufReader, Read};

fn process_rule(line: &str) -> (u64, u64) {
    line.split_once('|')
        .and_then(|(num_1, num_2)| {
            num_1
                .parse::<u64>()
                .ok()
                .and_then(|num_1| num_2.parse::<u64>().ok().map(|num_2| (num_1, num_2)))
        })
        .expect("failed to parse numbers from a line")
}

fn process_update(line: &str, page_numbers: &mut Vec<u64>, rules: &HashMap<u64, Vec<u64>>) -> bool {
    // Process the update
    line.split(',').for_each(|page| {
        let page = page.parse::<u64>().expect("failed to parse the page number");
        page_numbers.push(page);
    });

    let mut was_in_correct_order = true;
    'outer: for idx in 0..page_numbers.len() {
        let page_number = page_numbers[idx];
        let Some(number_rules) = rules.get(&page_number) else {
            // No rules for this number
            continue;
        };

        for idx_behind in 0..page_numbers[..idx].len() {
            let number_behind = page_numbers[idx_behind];
            if number_rules.contains(&number_behind) {
                // Skip this update, as numbers are in the wrong order
                was_in_correct_order = false;
                // Move the current element to its correct position
                page_numbers.insert(idx_behind, page_number);
                page_numbers.remove(idx + 1);
                // Continue to the next number, as we already found the earliest failing number and ther is no need
                // to check others
                continue 'outer;
            }
        }
    }

    was_in_correct_order
}

pub fn solve_puzzle<R: Read>(src: R) -> (u64, u64) {
    let reader = BufReader::new(src);

    // part 1
    let mut sum_of_middle_page_numbers_of_correct_updates = 0;
    // part 2
    let mut sum_of_middle_page_numbers_of_fixed_updates = 0;

    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut is_processing_instructions = true;
    let mut page_numbers = Vec::new();
    reader.lines().for_each(|line| {
        let line = line.expect("error while reading a line from the file");
        if line.is_empty() {
            // We processed the last instruction, proceed to processing the updates
            is_processing_instructions = false;
            return;
        }
        if is_processing_instructions {
            let (num_1, num_2) = process_rule(&line);
            rules.entry(num_1).or_default().push(num_2);
        } else {
            // Clear the vec from the previous update
            page_numbers.clear();

            let was_in_correct_order = process_update(&line, &mut page_numbers, &rules);

            let middle_page = page_numbers[page_numbers.len() / 2];

            if was_in_correct_order {
                sum_of_middle_page_numbers_of_correct_updates += middle_page;
            } else {
                sum_of_middle_page_numbers_of_fixed_updates += middle_page;
            }
        }
    });

    (
        sum_of_middle_page_numbers_of_correct_updates,
        sum_of_middle_page_numbers_of_fixed_updates,
    )
}

#[cfg(test)]
mod tests {
    use crate::{process_rule, solve_puzzle};

    #[test]
    fn parses_rule_correctly() {
        let line = "77|123";
        assert_eq!(process_rule(line), (77, 123));
    }

    #[test]
    fn solves_both_parts_correctly() {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

        let (part_1, part_2) = solve_puzzle(input.as_bytes());
        assert_eq!(part_1, 143);
        assert_eq!(part_2, 123);
    }
}
