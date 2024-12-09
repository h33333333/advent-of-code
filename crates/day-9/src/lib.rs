use std::collections::VecDeque;

pub fn part_1(mut data: Vec<Option<usize>>, mut free_slots: VecDeque<(usize, usize)>) -> usize {
    let (mut free_slot_start, mut free_slot_end) = free_slots.pop_front().expect("not a single free slot?");
    for idx in (0..data.len()).rev() {
        if free_slot_start == free_slot_end {
            let Some(slot) = free_slots.pop_front() else {
                // No free slots left, the size was optimized fully
                break;
            };
            (free_slot_start, free_slot_end) = slot;
        }
        if idx > free_slot_start {
            let Some(item) = data[idx].take() else {
                // Current item is a free slot
                continue;
            };
            data[free_slot_start].replace(item);
            free_slot_start += 1;
        }
    }

    data.iter()
        .enumerate()
        .filter(|(_, file)| file.is_some())
        .fold(0, |acc, (idx, file)| acc + idx * file.unwrap())
}

pub fn part_2(mut data: Vec<Option<usize>>, mut free_slots: VecDeque<(usize, usize)>) -> usize {
    let mut inside_file = None;
    for idx in (0..data.len()).rev() {
        let Some(item) = data[idx] else {
            // Current item is a free slot
            continue;
        };
        if inside_file.is_some_and(|current_file| current_file == item) {
            continue;
        }
        inside_file = Some(item);
        let current_file = data[0..=idx]
            .iter_mut()
            .rev()
            .take_while(|num| num.is_some_and(|num| num == item))
            .collect::<Vec<_>>();
        let file_size = current_file.len();
        let Some((free_slot_start, _)) = free_slots
            .iter_mut()
            .find(|slot| idx > slot.0 && slot.1 - slot.0 >= file_size)
        else {
            // No slot for this file, continue to the next one
            continue;
        };
        current_file.into_iter().for_each(|item| {
            item.take();
        });

        for _ in 0..file_size {
            data[*free_slot_start].replace(item);
            *free_slot_start += 1;
        }
    }

    data.iter()
        .enumerate()
        .filter(|(_, file)| file.is_some())
        .fold(0, |acc, (idx, file)| acc + idx * file.unwrap())
}

pub fn parse_input(input: &str) -> (Vec<Option<usize>>, VecDeque<(usize, usize)>) {
    input.chars().enumerate().fold(
        (Vec::new(), VecDeque::new()),
        |(mut acc, mut free_slots), (idx, num)| {
            let value = if idx % 2 == 0 {
                Some(idx / 2 + idx % 2)
            } else {
                // Empty slot
                None
            };

            let size = num.to_digit(10).expect("Invalid input");

            if size > 0 && idx % 2 == 1 {
                free_slots.push_back((acc.len(), acc.len() + size as usize))
            }

            acc.extend((0..size).map(|_| value));

            (acc, free_slots)
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_1, part_2};

    #[test]
    fn correctly_calculates_both_parts() {
        let test_input = "2333133121414131402";
        let (data, free_slots) = parse_input(test_input);

        let part_1 = part_1(data.clone(), free_slots.clone());
        assert_eq!(part_1, 1928);
        let part_2 = part_2(data, free_slots);
        assert_eq!(part_2, 2858);
    }
}
