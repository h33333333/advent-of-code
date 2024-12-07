use std::io::{BufRead as _, BufReader, Read};

#[derive(Default, Clone, Copy)]
pub enum Operation {
    #[default]
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn next(&mut self, with_concat: bool) -> bool {
        let (new_state, should_update_previous) = match self {
            Operation::Add => (Operation::Mul, false),
            Operation::Mul => {
                if with_concat {
                    (Operation::Concat, false)
                } else {
                    (Operation::Add, true)
                }
            }
            Operation::Concat => (Operation::Add, true),
        };
        *self = new_state;
        should_update_previous
    }
}

pub struct OperationSet {
    operations: Vec<Operation>,
    with_concat: bool,
}

impl OperationSet {
    pub fn new(size: usize, with_concat: bool) -> Self {
        OperationSet {
            operations: (0..size).map(|_| Operation::default()).collect(),
            with_concat,
        }
    }

    pub fn to_next_set(&mut self) -> bool {
        let mut idx = self.operations.len() - 1;
        while self.operations[idx].next(self.with_concat) {
            if idx == 0 {
                // We exhausted all the options
                return false;
            }
            idx -= 1;
        }
        true
    }

    pub fn get_operation_at_idx(&self, index: usize) -> Operation {
        self.operations[index]
    }
}

pub fn solve_both_parts<R: Read>(src: R) -> [u64; 2] {
    let reader = BufReader::new(src);

    let mut solutions = [0, 0];
    let mut numbers = Vec::new();
    for line in reader.lines() {
        let line = line.expect("error while reading a line");
        let (test_value, equation) = line.split_once(':').expect("malformed input");
        let test_value: u64 = test_value.parse().expect("not a valid test value");

        numbers.clear();
        for number in equation.trim().split(' ') {
            let number: u64 = number.parse().expect("invalid number in equation");
            numbers.push(number);
        }

        'outer: for (idx, solution) in solutions.iter_mut().enumerate() {
            let mut operations = OperationSet::new(numbers.len(), idx != 0);
            while operations.to_next_set() {
                let result = numbers[1..]
                    .iter()
                    .enumerate()
                    .fold(numbers[0], |mut acc, (idx, number)| {
                        let operation = operations.get_operation_at_idx(idx);
                        match operation {
                            Operation::Add => acc += number,
                            Operation::Mul => {
                                acc *= number;
                            }
                            Operation::Concat => acc = concat(acc, *number),
                        }
                        acc
                    });
                if result == test_value {
                    *solution += test_value;
                    continue 'outer;
                }
            }
        }
    }
    solutions
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

#[cfg(test)]
mod tests {
    use crate::solve_both_parts;

    #[test]
    fn correctly_solves_both_parts() {
        let input = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";

        let [part_1, part_2] = solve_both_parts(input.as_bytes());

        assert_eq!(part_1, 3749);
        assert_eq!(part_2, 11387);
    }
}
