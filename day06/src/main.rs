use std::time::Instant;

use utils::inputs::read_lines;

const DIR_UP: u8 = 1;
const DIR_RIGHT: u8 = 2;
const DIR_DOWN: u8 = 4;
const DIR_LEFT: u8 = 8;

fn main() {
    let now = Instant::now();
    let mut result2: u32 = 0;

    let lines: Vec<String> = read_lines("./day06/input.txt").unwrap().flatten().collect();

    let width = lines.len();
    let char_vec: Vec<char> = vec!['.'; width * width];
    let mut char_array: Box<[char]> = char_vec.into_boxed_slice();

    let mut start_pos: (usize, usize) = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        let offset: usize = width * y;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                char_array[offset + x] = c;
                if c == '^' {
                    start_pos = (x, y);
                }
            }
        }
    }

    fn stays_in_grid((x, y): (usize, usize), dir: u8, length: usize) -> bool {
        match dir {
            DIR_UP => y > 0,
            DIR_RIGHT => x < length - 1,
            DIR_DOWN => y < length - 1,
            DIR_LEFT => x > 0,
            _ => panic!("Invalid direction"),
        }
    }

    fn go_in_direction((x, y): (usize, usize), dir: u8) -> (usize, usize) {
        match dir {
            DIR_UP => (x, y - 1),
            DIR_RIGHT => (x + 1, y),
            DIR_DOWN => (x, y + 1),
            DIR_LEFT => (x - 1, y),
            _ => panic!("Invalid direction"),
        }
    }

    fn rotate90(dir: u8) -> u8 {
        if dir == DIR_LEFT {
            DIR_UP
        } else {
            dir << 1
        }
    }

    fn trace_path(
        steps: &mut u64,
        start_pos: (usize, usize),
        start_dir: u8,
        char_array: &mut Box<[char]>,
        width: usize,
    ) -> u32 {
        let mut path_length: u32 = 1;
        char_array[calc_array_pos(start_pos.1, start_pos.0, width)] = 'X';

        let mut curr_pos = start_pos;
        let mut curr_dir = start_dir;

        while stays_in_grid(curr_pos, curr_dir, char_array.len()) {
            *steps += 1;
            let (x, y) = go_in_direction(curr_pos, curr_dir);
            if char_array[calc_array_pos(y, x, width)] == '#' {
                curr_dir = rotate90(curr_dir);
            } else {
                curr_pos = (x, y);
                let array_index = calc_array_pos(curr_pos.1, curr_pos.0, width);
                if char_array[array_index] != 'X' {
                    char_array[array_index] = 'X';
                    path_length += 1;
                }
            }
        }
        return path_length;
    }

    fn calc_array_pos(y: usize, x: usize, width: usize) -> usize {
        width * y + x
    }

    fn simulate_run(
        steps: &mut u64,
        start_pos: (usize, usize),
        start_dir: u8,
        char_array: &Box<[char]>,
        visited: &mut Box<[u8]>,
        obstacle_pos: (usize, usize),
        width: usize,
    ) -> bool {
        let mut curr_pos = start_pos;
        let mut curr_dir = start_dir;

        for i in 0..visited.len() {
            visited[i] = 0;
        }
        visited[calc_array_pos(curr_pos.1, curr_pos.0, width)] |= curr_dir;

        while stays_in_grid(curr_pos, curr_dir, width) {
            *steps += 1;
            let (x, y) = go_in_direction(curr_pos, curr_dir);
            if char_array[calc_array_pos(y, x, width)] == '#' || (x, y) == obstacle_pos {
                curr_dir = rotate90(curr_dir);
            } else {
                curr_pos = (x, y);
            }
            let array_index = calc_array_pos(curr_pos.1, curr_pos.0, width);
            if visited[array_index] & curr_dir != 0 {
                return true;
            }
            visited[array_index] |= curr_dir;
        }
        return false;
    }

    #[allow(dead_code)]
    fn simulate_run_limit(
        steps: &mut u64,
        start_pos: (usize, usize),
        start_dir: u8,
        char_array: &Box<[char]>,
        limit: u32,
        obstacle_pos: (usize, usize),
        width: usize,
    ) -> bool {
        let mut curr_pos = start_pos;
        let mut curr_dir = start_dir;
        let mut curr_steps = 0;

        while stays_in_grid(curr_pos, curr_dir, width) {
            *steps += 1;
            curr_steps += 1;
            let (x, y) = go_in_direction(curr_pos, curr_dir);
            if char_array[y*width+x] == '#' || (x, y) == obstacle_pos {
                curr_dir = rotate90(curr_dir);
            } else {
                curr_pos = (x, y);
            }
            if curr_steps >= limit {
                return true;
            }
            curr_steps += 1;
        }
        return false;
    }

    let mut steps: u64 = 0;
    let start_dir = DIR_UP;
    let result = trace_path(&mut steps, start_pos, start_dir, &mut char_array, width);

    char_array[calc_array_pos(start_pos.1, start_pos.0, width)] = '^';

    let visited_vec: Vec<u8> = vec![0; width * width];
    let mut visited: Box<[u8]> = visited_vec.into_boxed_slice();

    for y in 0..width {
        let offset = width * y;
        for x in 0..width {
            if char_array[offset + x] == 'X' {
                let has_loop = simulate_run(
                    &mut steps,
                    start_pos,
                    start_dir,
                    &char_array,
                    &mut visited,
                    (x, y),
                    width,
                );
                if has_loop {
                    result2 += 1;
                }
            }
        }
    }

    // let limit = (width * width) as u32;

    // for y in 0..width {
    //     for x in 0..width {
    //         if char_array[y*width + x] == 'X' {
    //             let has_loop = simulate_run_limit(
    //                 &mut steps,
    //                 start_pos,
    //                 start_dir,
    //                 &char_array,
    //                 limit,
    //                 (x, y),
    //                 width,
    //             );
    //             if has_loop {
    //                 result2 += 1;
    //             }
    //         }
    //     }
    // }

    println!("day  6");
    println!("  - part 1: {}", result); // 4602
    println!("  - part 2: {}", result2); // 1703

    let elapsed = now.elapsed();
    println!("runtime: {:.2?}", elapsed);
    println!("steps: {}", steps);

    /*
       | visited lookup  |   trace lookup          | runtime  | num steps | notes
       |-----------------|-------------------------|----------|-----------|--------------------------------------------------------------------
       | hashset         |   hashset               | 118.27s  | 78627888  |
       | vector          |   hashset               |  78.22s  | 78627888  |
       | vector          |   hashset (if visited)  |  62.11s  | 78627888  | only check trace in hashset if position was visited
       | vector          |   hashset (if visited)  |  58.42s  | 74239956  | excluded starting pos + obstacles pos
       | vector          |   hashset (if visited)  |  11.65s  | 14355222  | pre-calc path, only place obstacles on original path
       | no lookup       |   vector (recycle)      |  10.84s  | 14355222  | skip visited lookup, only do trace lookup using nested vector
       | no lookup       |   vector (recycle)      |  10.29s  | 14355222  | pass mutable vector to function to avoid allocation
       | no lookup       |   vector (recycle)      |  10.18s  | 14355222  | use int as direction, calc next pos via switch case
       | no lookup       |   flat vector (recycle) |   4.12s  | 14355222  | use flat vector (bool, size: length * length * 4)
       | no lookup       |   flat vector (recycle) |   1.50s  | 14355222  | use flat vector (u8, size: length * length), bit mask for direction
       | no lookup       |   flat vector (recycle) |   1.45s  | 14355222  | direction as bit mask
       | no lookup       |   flat vector (recycle) |   1.25s  | 14355222  | input as 2d array (hard coded size)
       | no lookup       |   flat vector (recycle) |   1.23s  | 14355222  | use const values for length and length where possible
       | no lookup       |   flat vector (recycle) |   1.23s  | 14355222  | input as flat array (hard coded size)
       | no lookup       |   flat vector (recycle) |   1.21s  | 14355222  | input as boxed slice (dynamic size)
       | no lookup       |   boxed slice (recycle) |   0.60s  | 14355222  | trace lookup using boxed slice
       | no lookup       |   no lookup             |   0.64s  | 23420927  | detect loop by counting steps and setting limit
       | no lookup       |   no lookup             |   0.27s  | 23420927  | use boxed slice for char array
    */
}
