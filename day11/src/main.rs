use std::collections::HashMap;

use utils::inputs::read_lines;

fn solve(input: &Box<[u64]>, initial_depth: u8) -> u64 {
    let mut solution: u64 = 0;

    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();

    fn solve_recursive(num: u64, depth: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
        if depth == 0 {
            return 1;
        }

        if let Some(&result) = cache.get(&(num, depth)) {
            return result;
        }

        let result: u64;

        if num == 0 {
            result = solve_recursive(1, depth - 1, cache);
        } else {
            let num_str = num.to_string();
            if num_str.len() % 2 == 0 {
                let l_half = num_str[..num_str.len() / 2].parse::<u64>().unwrap();
                let r_half = num_str[num_str.len() / 2..].parse::<u64>().unwrap();
                result = solve_recursive(l_half, depth - 1, cache)
                    + solve_recursive(r_half, depth - 1, cache);
            } else {
                result = solve_recursive(num * 2024, depth - 1, cache);
            }
        }

        cache.insert((num, depth), result);

        return result;
    }

    for num in input {
        solution += solve_recursive(*num, initial_depth, &mut cache);
    }

    return solution;
}

fn main() {
    let lines: Vec<String> = read_lines("./day11/input.txt").unwrap().flatten().collect();
    let input = lines[0]
        .split_whitespace()
        .flat_map(|value| u64::from_str_radix(value, 10))
        .collect::<Vec<u64>>()
        .into_boxed_slice();

    let solution1: u64 = solve(&input, 25);
    let solution2: u64 = solve(&input, 75);

    println!("day  11");
    println!("  - part 1: {}", solution1); // 203228
    println!("  - part 2: {}", solution2); // 240884656550923
}
