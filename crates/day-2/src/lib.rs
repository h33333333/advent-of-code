use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read};

enum LinePattern {
    Unset,
    Decreasing,
    Increasing,
}

fn is_report_safe(line: &str, skip_idx: Option<usize>) -> (bool, usize) {
    let mut last_number = None;
    let mut pattern = LinePattern::Unset;
    for (idx, num) in line
        .split(" ")
        .enumerate()
        .filter(|&(idx, _)| skip_idx.is_none() || idx != skip_idx.unwrap())
    {
        let num: i64 = num.parse().expect("not a valid integer found in the input");

        if let Some(last_num) = last_number {
            match pattern {
                LinePattern::Unset => {
                    pattern = match num.cmp(&last_num) {
                        std::cmp::Ordering::Less => LinePattern::Decreasing,
                        std::cmp::Ordering::Greater => LinePattern::Increasing,
                        std::cmp::Ordering::Equal => {
                            // Not a safe report
                            return (false, idx);
                        }
                    };
                }
                LinePattern::Decreasing => {
                    if num.cmp(&last_num) != Ordering::Less {
                        return (false, idx);
                    }
                }
                LinePattern::Increasing => {
                    if num.cmp(&last_num) != Ordering::Greater {
                        return (false, idx);
                    }
                }
            }

            let number_diff = (num - last_num).abs();

            if !(1..=3).contains(&number_diff) {
                return (false, idx);
            }
        }

        last_number = Some(num);
    }

    (true, 0)
}

pub fn find_safe_reports<R: Read>(src: R) -> (i32, i32) {
    let reader = BufReader::new(src);

    let mut safe_reports_part_1 = 0;
    let mut safe_reports_part_2 = 0;
    'outer: for report in reader.lines() {
        let line = report.expect("error while reading a line from the input file");

        let mut tolerated_a_level = false;
        let mut indexes_to_skip = Vec::new();
        loop {
            let index = indexes_to_skip.pop();

            if tolerated_a_level && index.is_none() {
                // Tolerating a bad level didn't help, report is still unsafe
                continue 'outer;
            }

            let (safe, failing_idx) = is_report_safe(&line, index);

            if !safe && !tolerated_a_level {
                tolerated_a_level = true;
                indexes_to_skip.extend(0..=failing_idx);
                continue;
            }

            if safe {
                break;
            }
        }
        if !tolerated_a_level {
            safe_reports_part_1 += 1;
        }
        safe_reports_part_2 += 1;
    }

    (safe_reports_part_1, safe_reports_part_2)
}

#[cfg(test)]
mod tests {
    use crate::find_safe_reports;

    const TEST_DATA: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn correctly_determines_safe_reports() {
        let (part_1, part_2) = find_safe_reports(TEST_DATA.as_bytes());

        assert_eq!(part_1, 2);
        assert_eq!(part_2, 4);
    }
}
