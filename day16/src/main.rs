use std::str::Lines;

use utils::grid::print_char_grid;

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

fn read_map(lines: &mut Lines) -> (Box<[Box<[char]>]>, usize, usize) {
    let mut grid_lines: Vec<Box<[char]>> = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        grid_lines.push(line.trim().chars().collect::<Box<[char]>>());
    }
    let grid = grid_lines.into_boxed_slice();
    let width = grid[0].len();
    let height = grid.len();

    (grid, width, height)
}

fn find_start_end(
    grid: &[Box<[char]>],
    width: usize,
    height: usize,
) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'S' {
                start = (x, y);
            } else if grid[y][x] == 'E' {
                end = (x, y);
            }
        }
    }
    (start, end)
}

fn solve1(
    grid: &[Box<[char]>],
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> u32 {
    let mut visited: Box<[Box<[[u32; 4]]>]> =
        vec![vec![[u32::MAX, u32::MAX, u32::MAX, u32::MAX]; width].into_boxed_slice(); height].into_boxed_slice();
    let mut queue: Vec<(u32, (usize, usize), usize)> = Vec::new();

    let mut min_cost = u32::MAX;
    queue.push((0, start, RIGHT));

    while !queue.is_empty() {
        let (cost, (x, y), direction) = queue.pop().unwrap();
        if (x, y) == end {
            if cost < min_cost {
                min_cost = cost;
            }
            continue;
        }
        if cost >= visited[y][x][direction] {
            continue;
        }
        visited[y][x][direction] = cost;
        if cost >= min_cost {
            continue;
        }

        // go straight
        let (next_x, next_y) = match direction {
            RIGHT => (x + 1, y),
            DOWN => (x, y + 1),
            LEFT => (x - 1, y),
            UP => (x, y - 1),
            _ => panic!("Invalid direction"),
        };
        if grid[next_y][next_x] != '#' {
            queue.push((cost + 1, (next_x, next_y), direction));
        }

        // turn clockwise
        let cw_direction = match direction {
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            UP => RIGHT,
            _ => panic!("Invalid direction"),
        };
        queue.push((cost + 1000, (x, y), cw_direction));

        // turn counterclockwise
        let ccw_direction = match direction {
            RIGHT => UP,
            DOWN => RIGHT,
            LEFT => DOWN,
            UP => LEFT,
            _ => panic!("Invalid direction"),
        };
        queue.push((cost + 1000, (x, y), ccw_direction));
    }

    return min_cost;
}

fn main() {
    let mut input = include_str!("../input.txt").lines();

    let (grid, width, height) = read_map(&mut input);

    print_char_grid(&grid);
    let (start, end) = find_start_end(&grid, width, height);

    let solution1 = solve1(&grid, width, height, start, end);
    println!("day 16");
    println!("  - part 1: {}", solution1); // 122492
}
