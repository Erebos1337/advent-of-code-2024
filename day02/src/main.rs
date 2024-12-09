use utils::{conversion::to_int, inputs::read_lines, vectors::remove_element};

pub fn main() {
    let lines = read_lines("./day02/input.txt").unwrap();

    let mut num_valid_reports: u32 = 0;
    let mut num_valid_reports_damped: u32 = 0;

    const INCREASING: &str = "increasing";
    const EQUAL: &str = "equal";
    const DIFF: &str = "diff";

    fn is_valid_report(report: &Vec<u32>) -> (bool, usize, &str) {
        if report[0] == report[1] {
            return (false, 1, EQUAL);
        };
        if (report[0]).abs_diff(report[1]) > 3 {
            return (false, 1, DIFF);
        };

        let is_increasing = report[0] < report[1];
        let mut last_level = report[1];

        for index in 2..report.len() {
            let curr_level = report[index];
            if curr_level == last_level {
                return (false, 1, EQUAL);
            };
            if (last_level < curr_level) != is_increasing {
                return (false, index, INCREASING);
            };
            if (curr_level).abs_diff(last_level) > 3 {
                return (false, index, DIFF);
            }
            last_level = curr_level;
        }
        return (true, 0, "");
    }

    fn is_valid_report_after_damped(report: &Vec<u32>, error_at: usize, error_type: &str) -> bool {
        let levels_to_remove: Vec<usize> = match error_type {
            // if there are two equal levels, remove one of them
            EQUAL => vec![error_at],
            // only at position 2, we can run into a situation where the initial assumption
            // about if we have an increasing or decreasing sequence can be wrong
            // thus try to remove the first element as well
            INCREASING if error_at == 2 => vec![error_at - 2, error_at - 1, error_at],
            // in all other cases the erronous level must be one of the last two that were checked
            _ => vec![error_at - 1, error_at],
        };
        for index in levels_to_remove {
            let (is_valid, _, _) = is_valid_report(&remove_element(&report, index));
            if is_valid {
                return true;
            }
        }

        return false;
    }

    for line in lines.flatten() {
        let levels = line.split_terminator(" ").map(to_int).collect();

        let (is_valid, error_at, error_type) = is_valid_report(&levels);
        if is_valid {
            num_valid_reports += 1;
        } else if is_valid_report_after_damped(&levels, error_at, error_type) {
            num_valid_reports_damped += 1;
        }
    }

    println!("day  2");
    println!("  - part 1: {}", num_valid_reports); // 680
    println!(
        "  - part 2: {}",
        num_valid_reports + num_valid_reports_damped
    ); // 710
}
