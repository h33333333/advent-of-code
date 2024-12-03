pub fn process_instructions(data: &str, enable_additional_instructions: bool) -> i64 {
    let mut computation_result = 0;
    let mut remaining = data;
    let mut is_enabled = true;
    loop {
        let Some(start_pos) = remaining.find(['d', 'm']) else {
            // No more muls/dos :(
            break;
        };
        if remaining.as_bytes()[start_pos] == b'd' {
            // Process the enable/disable instruction
            if !&remaining
                .as_bytes()
                .get(start_pos..start_pos + 2)
                .is_some_and(|instructon| instructon == b"do")
            {
                // Not a valid instruction
                remaining = &remaining[start_pos + 1..];
                continue;
            }
            if remaining
                .as_bytes()
                .get(start_pos..start_pos + 5)
                .is_some_and(|instruction| instruction == b"don't")
            {
                // Disable all upcoming muls until enabled
                is_enabled = false;
                remaining = &remaining[start_pos + 5..];
                continue;
            } else {
                // Enable all upcoming muls until disabled
                is_enabled = true;
                remaining = &remaining[start_pos + 2..];
                continue;
            }
        } else {
            // Process mul
            if !remaining
                .get(start_pos..start_pos + 3)
                .is_some_and(|instruction| instruction == "mul")
            {
                // Not a mul :(
                remaining = &remaining[start_pos + 1..];
                continue;
            }
            remaining = &remaining[start_pos + 3..];

            if !is_enabled && enable_additional_instructions {
                // Instruction is disabled, continue to the next one
                continue;
            }
            if remaining.as_bytes()[0] != b'(' {
                // Malformed instruction, continue
                continue;
            }
            remaining = &remaining[1..];

            let Some(delim_pos) = remaining.find(',') else {
                // No delim?
                continue;
            };
            if delim_pos > 3 {
                // Invalid instruction, continue to the next one
                continue;
            }

            let Ok(first_num) = &remaining[..delim_pos].parse::<i64>() else {
                // Bad number, continue to the next instruction
                remaining = &remaining[delim_pos + 1..];
                continue;
            };
            remaining = &remaining[delim_pos + 1..];

            let Some(closing_bracket_pos) = remaining.find(')') else {
                // No closing bracket?
                continue;
            };
            if closing_bracket_pos > 3 {
                // Invalid instruction, continue to the next one
                continue;
            }

            let Ok(second_num) = &remaining[..closing_bracket_pos].parse::<i64>() else {
                // Bad number, continue to the next instruction
                remaining = &remaining[closing_bracket_pos + 1..];
                continue;
            };

            computation_result += first_num * second_num;
        }
    }

    computation_result
}

#[cfg(test)]
mod tests {
    use crate::process_instructions;

    #[test]
    fn correctly_calculates_part_1_result() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = process_instructions(test_input, false);
        assert_eq!(result, 161);
    }

    #[test]
    fn correctly_calculates_part_2_result() {
        let test_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = process_instructions(test_input, true);
        assert_eq!(result, 48);
    }
}
