use std::collections::HashMap;

use cached::proc_macro::cached;

#[cached]
fn press_keys_directional(keys: String) -> String {
    let mut pressed_keys = String::new();

    let grid = [
        ['#', '^', 'A'], // first row
        ['<', 'v', '>'], // second row
    ];

    let (mut curr_x, mut curr_y) = (2usize, 0usize);
    for key in keys.chars() {
        match key {
            '^' => {
                curr_y -= 1;
            }
            'v' => {
                curr_y += 1;
            }
            '<' => {
                curr_x -= 1;
            }
            '>' => {
                curr_x += 1;
            }
            'A' => {
                pressed_keys.push(grid[curr_y][curr_x]);
            }
            _ => panic!("Invalid key: {}", key),
        }
    }

    pressed_keys
}

#[cached]
fn press_keys_numeric(keys: String) -> String {
    let mut pressed_keys = String::new();

    let grid = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        ['#', '0', 'A'],
    ];

    let (mut curr_x, mut curr_y) = (2usize, 3usize);
    for key in keys.chars() {
        match key {
            '^' => {
                curr_y -= 1;
            }
            'v' => {
                curr_y += 1;
            }
            '<' => {
                curr_x -= 1;
            }
            '>' => {
                curr_x += 1;
            }
            'A' => {
                pressed_keys.push(grid[curr_y][curr_x]);
            }
            _ => panic!("Invalid key: {}", key),
        }
    }

    pressed_keys
}

#[cached]
fn key_pos_numeric(key: char) -> (usize, usize) {
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
    let key_options: [char; 11] = ['7', '8', '9', '4', '5', '6', '1', '2', '3', '0', 'A'];

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
fn get_chars(code: String) -> Box<[char]> {
    code.chars().collect()
}

#[cached]
fn get_char_at(code: String, ptr: usize) -> char {
    get_chars(code)[ptr]
}

fn build_all_paths(
    code: Box<[char]>,
    ptr: usize,
    key_paths: &HashMap<(char, char), Vec<String>>,
) -> Vec<String> {
    if ptr == code.len() {
        return vec!["".to_string()];
    }
    let mut all_paths: Vec<String> = vec![];

    let curr_char = if ptr > 0 { code[ptr - 1] } else { 'A' };
    let next_char = code[ptr];

    let suffixes = build_all_paths(code, ptr + 1, key_paths);
    let prefixes = key_paths.get(&(curr_char, next_char)).unwrap();

    for prefix in prefixes {
        for suffix in &suffixes {
            all_paths.push([prefix.clone(), suffix.clone()].concat());
        }
    }

    all_paths
}

#[cached]
fn build_all_paths_directional(code: Box<[char]>, ptr: usize) -> Vec<String> {
    build_all_paths(code, ptr, &get_key_paths_directional())
}

#[cached]
fn build_all_paths_numeric(code: Box<[char]>, ptr: usize) -> Vec<String> {
    build_all_paths(code, ptr, &get_key_paths_numeric())
}

fn shortest_paths_numeric(my_code: String) -> Vec<String> {
    let mut shortest_path_length = usize::MAX;
    let mut shortest_paths: Vec<String> = vec![];

    let all_paths = build_all_paths_numeric(my_code.chars().collect(), 0);

    for path in all_paths {
        let path_length = path.len();
        if path_length < shortest_path_length {
            shortest_path_length = path_length;
            shortest_paths.clear();
            shortest_paths.push(path);
        } else if path.len() == shortest_path_length {
            shortest_paths.push(path);
        }
    }

    shortest_paths
}

fn shortest_paths_directional(code: String, depth: u8) -> Vec<String> {
    let my_codes: Vec<String> = if depth == 1 {
        shortest_paths_numeric(code)
    } else {
        shortest_paths_directional(code, depth - 1)
    };

    let mut shortest_path_length = usize::MAX;
    let mut shortest_paths: Vec<String> = vec![];

    for my_code in my_codes {
        let my_code_chars: Box<[char]> = my_code.chars().collect();
        let all_paths = build_all_paths_directional(my_code_chars, 0);

        for path in all_paths {
            let path_length = path.len();
            if path_length < shortest_path_length {
                shortest_path_length = path_length;
                shortest_paths.clear();
                shortest_paths.push(path);
            } else if path.len() == shortest_path_length {
                shortest_paths.push(path);
            }
        }
    }

    shortest_paths
}

fn solve1(codes: &[&str]) -> usize {
    let mut complexity = 0usize;

    for code in codes {
        let code_num = code.strip_suffix("A").unwrap().parse::<usize>().unwrap();
        let sequences = shortest_paths_directional(code.to_string(), 2);
        let sequence = &sequences[0];
        complexity += sequence.len() * code_num;
    }
    complexity
}

fn main() {
    let input = include_str!("../input.txt");
    let codes: Box<[&str]> = input.lines().map(|s| s.trim()).collect();

    let solution1 = solve1(&codes);

    println!("day 21");
    println!("  - part 1: {}", solution1); // 270084
}
