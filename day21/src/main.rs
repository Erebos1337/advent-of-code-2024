use std::collections::HashMap;

use cached::proc_macro::cached;

#[cached]
fn key_pos_numeric(key: char) -> (usize, usize) {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    match key {
        '0' => (1, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        'A' => (2, 3),
        _ => panic!("Invalid key: {}", key),
    }
}

#[cached]
fn key_pos_directional(key: char) -> (usize, usize) {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    match key {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("Invalid key: {}", key),
    }
}

#[cached]
fn get_key_paths_numeric() -> HashMap<(char, char), Vec<String>> {
    let key_options: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'];

    let mut sub_paths: HashMap<(char, char), Vec<String>> =
        HashMap::<(char, char), Vec<String>>::new();
    for start_key in key_options {
        let (start_x, start_y) = key_pos_numeric(start_key);
        for end_key in key_options {
            let (end_x, end_y) = key_pos_numeric(end_key);

            let mut curr_paths = vec![];

            let x_diff = end_x as i8 - start_x as i8;
            let y_diff = end_y as i8 - start_y as i8;

            let x_keys: String = if x_diff > 0 {
                ">".repeat(x_diff as usize)
            } else {
                "<".repeat(x_diff.abs() as usize)
            };
            let y_keys: String = if y_diff > 0 {
                "v".repeat(y_diff as usize)
            } else {
                "^".repeat(y_diff.abs() as usize)
            };

            let y_path = [y_keys.clone(), x_keys.clone(), "A".to_string()].concat();
            let x_path = [x_keys.clone(), y_keys.clone(), "A".to_string()].concat();

            if x_diff == 0 || (y_diff < 0 && start_y == 3 && end_x == 0) {
                curr_paths.push(y_path);
            } else if y_diff == 0 || (y_diff > 0 && start_x == 0 && end_y == 3) {
                curr_paths.push(x_path);
            } else {
                curr_paths.push(y_path);
                curr_paths.push(x_path);
            }

            sub_paths.insert((start_key, end_key), curr_paths);
        }
    }
    sub_paths
}

#[cached]
fn get_key_paths_directional() -> HashMap<(char, char), Vec<String>> {
    let key_options: [char; 5] = ['<', '^', '>', 'v', 'A'];

    let mut sub_paths: HashMap<(char, char), Vec<String>> =
        HashMap::<(char, char), Vec<String>>::new();
    for start_key in key_options {
        let (start_x, start_y) = key_pos_directional(start_key);
        for end_key in key_options {
            let (end_x, end_y) = key_pos_directional(end_key);

            let mut curr_paths = vec![];

            let x_diff = end_x as i8 - start_x as i8;
            let y_diff = end_y as i8 - start_y as i8;

            let x_keys: String = if x_diff > 0 {
                ">".repeat(x_diff as usize)
            } else {
                "<".repeat(x_diff.abs() as usize)
            };
            let y_keys: String = if y_diff > 0 {
                "v".repeat(y_diff as usize)
            } else {
                "^".repeat(y_diff.abs() as usize)
            };

            let y_path = [y_keys.clone(), x_keys.clone(), "A".to_string()].concat();
            let x_path = [x_keys.clone(), y_keys.clone(), "A".to_string()].concat();

            if x_diff == 0 || (y_diff > 0 && start_y == 0 && end_x == 0) {
                curr_paths.push(y_path);
            } else if y_diff == 0 || (y_diff < 0 && start_x == 0 && end_y == 0) {
                curr_paths.push(x_path);
            } else {
                curr_paths.push(y_path);
                curr_paths.push(x_path);
            }

            sub_paths.insert((start_key, end_key), curr_paths);
        }
    }
    sub_paths
}

#[cached]
fn solve_dfs_directional(from: char, to: char, depth: usize) -> usize {
    if from == to {
        return 1;
    }

    let key_paths = get_key_paths_directional();
    let paths = &key_paths[&(from, to)];

    let mut min = usize::MAX;
    if depth == 1 {
        for path in paths {
            let path_length = path.len();
            if path_length < min {
                min = path_length;
            }
        }
    } else {
        for path in paths {
            let path_chars: Box<[char]> = path.chars().collect();
            let mut total = 0;
            let mut curr_char = 'A';
            for i in 0..path_chars.len() {
                let next_char = path_chars[i];
                total += solve_dfs_directional(curr_char, next_char, depth - 1);
                curr_char = next_char;
            }
            if total < min {
                min = total;
            }
        }
    }

    min
}

#[cached]
fn solve_dfs_numeric(from: char, to: char, depth: usize) -> usize {
    if from == to {
        return 1;
    }

    let key_paths = get_key_paths_numeric();
    let paths = &key_paths[&(from, to)];

    let mut min = usize::MAX;
    for path in paths {
        let path_chars: Box<[char]> = path.chars().collect();
        let mut total = 0;
        let mut curr_char = 'A';
        for i in 0..path_chars.len() {
            let next_char = path_chars[i];
            total += solve_dfs_directional(curr_char, next_char, depth);
            curr_char = next_char;
        }
        if total < min {
            min = total;
        }
    }

    min
}

fn solve_dfs(codes: &[&str], depth: usize) -> usize {
    let mut complexity = 0usize;

    for code in codes {
        let code_num = code.strip_suffix("A").unwrap().parse::<usize>().unwrap();
        let mut total = 0;
        let code_chars: Box<[char]> = code.chars().collect();
        let mut curr_char = 'A';
        for i in 0..code_chars.len() {
            let next_char = code_chars[i];
            total += solve_dfs_numeric(curr_char, next_char, depth);
            curr_char = next_char;
        }
        complexity += total * code_num;
    }
    complexity
}

fn solve1(codes: &[&str]) -> usize {
    solve_dfs(codes, 2)
}

fn solve2(codes: &[&str]) -> usize {
    solve_dfs(codes, 25)
}

fn main() {
    let input = include_str!("../input.txt");
    let codes: Box<[&str]> = input.lines().map(|s| s.trim()).collect();

    let solution1 = solve1(&codes);
    let solution2 = solve2(&codes);

    println!("day 21");
    println!("  - part 1: {}", solution1); // 270084
    println!("  - part 2: {}", solution2); // 329431019997766
}
