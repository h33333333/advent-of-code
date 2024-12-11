use std::collections::HashMap;
use std::mem;

type FrequencyMap = HashMap<u128, u128>;

pub fn parse_input(src: &str) -> FrequencyMap {
    src.split(" ").fold(HashMap::new(), |mut acc, num| {
        let num = num.parse::<u128>().expect("not a valid integer");
        *acc.entry(num).or_insert(0) += 1;
        acc
    })
}

pub fn solve_both_parts(mut map: FrequencyMap) -> (u128, u128) {
    let mut part_1 = 0;
    let mut part_2 = 0;

    // Temp map is needed to iterate over the frequency map and update it at the same time
    let mut temp_map = HashMap::new();
    for n in 0..75 {
        if n == 25 {
            part_1 = map.values().sum::<u128>();
        }

        blink(&mut map, &mut temp_map);

        if n == 74 {
            part_2 = map.values().sum::<u128>();
        }
    }

    (part_1, part_2)
}

fn blink(map: &mut FrequencyMap, temp: &mut FrequencyMap) {
    for (item, occurences) in map.drain() {
        if occurences == 0 {
            continue;
        }
        let num_of_digits = ((item as f64).log10().floor() + 1.) as u32;

        if item == 0 {
            *temp.entry(1).or_insert(0) += occurences;
        } else if num_of_digits % 2 == 0 {
            let mut left_number = item;
            let mut right_number = 0;
            for idx in 0..num_of_digits {
                if idx == num_of_digits / 2 {
                    // We split the number already
                    break;
                }
                let digit = left_number % 10;
                right_number += digit * 10u128.pow(idx);
                left_number /= 10
            }
            *temp.entry(left_number).or_insert(0) += occurences;
            *temp.entry(right_number).or_insert(0) += occurences;
        } else {
            *temp.entry(item * 2024).or_insert(0) += occurences;
        }
    }

    mem::swap(map, temp);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_both_parts};

    #[test]
    fn correctly_solves_both_parts() {
        let map = parse_input("125 17");

        let (part_1, part_2) = solve_both_parts(map);
        assert_eq!(part_1, 55312);
        assert_eq!(part_2, 65601038650482);
    }
}
