fn read_locks_keys(input: &str) -> (Box<[&str]>, Box<[&str]>) {
    let blocks: Box<[&str]> = input.trim().split_terminator("\n\n").collect();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for block in blocks {
        if block.starts_with("#") {
            locks.push(block);
        } else {
            keys.push(block);
        }
    }
    (locks.into_boxed_slice(), keys.into_boxed_slice())
}

fn transform_locks(locks: &[&str]) -> Box<[[u8;5]]> {
    let mut transformed_locks = Vec::new();

    for lock in locks {
        let mut transformed_lock = [0; 5];
        for line in lock.lines().skip(1) {
            for (char_num, c) in line.trim().chars().enumerate() {
                transformed_lock[char_num] += match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("Unexpected character in lock: {}", c),
                };
            }
        }
        transformed_locks.push(transformed_lock);
    }

    transformed_locks.into_boxed_slice()
}

fn transform_keys(keys: &[&str]) -> Box<[[u8;5]]> {
    let mut transformed_keys = Vec::new();

    for key in keys {
        let mut transformed_key = [0; 5];
        for line in key.lines().rev().skip(1) {
            for (char_num, c) in line.trim().chars().enumerate() {
                transformed_key[char_num] += match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("Unexpected character in key: {}", c),
                };
            }
        }
        transformed_keys.push(transformed_key);
    }

    transformed_keys.into_boxed_slice()
}

fn solve1(locks: &[&str], keys: &[&str]) -> u64 {
    let mut solution = 0;

    let lock_nums = transform_locks(locks);
    let key_nums = transform_keys(keys);

    for lock in &lock_nums {
        for key in &key_nums {
            let mut key_fits = true;

            for i in 0..lock.len() {
                if lock[i] + key[i] > 5 {
                    key_fits = false;
                    break;
                }
            }

            if key_fits {
                solution += 1;
            }
        }
    }

    solution
}

fn main() {
    let input = include_str!("../input.txt");

    let (locks, keys) = read_locks_keys(input);

    let solution1 = solve1(&locks, &keys);

    println!("day 25:");
    println!("  - part 1: {}", solution1); // 3107
}
