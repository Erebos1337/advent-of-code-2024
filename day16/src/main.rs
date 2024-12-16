use std::{
    collections::HashSet,
    str::Lines,
    time::{Duration, Instant},
};

use min_max_heap::MinMaxHeap;

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
) -> (u32, Duration) {
    let now = Instant::now();
    let mut visited: Box<[Box<[[u32; 4]]>]> =
        vec![vec![[u32::MAX, u32::MAX, u32::MAX, u32::MAX]; width].into_boxed_slice(); height]
            .into_boxed_slice();
    let mut queue: MinMaxHeap<(u32, (usize, usize), usize)> = MinMaxHeap::new();

    let mut min_cost = u32::MAX;
    queue.push((0, start, RIGHT));

    while !queue.is_empty() {
        let (cost, (x, y), direction) = queue.pop_min().unwrap();
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
    let elapsed = now.elapsed();
    (min_cost, elapsed)
}

fn solve2(
    grid: &[Box<[char]>],
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> (u32, Duration) {
    let now = Instant::now();
    let mut visited_forwards_right: Box<[Box<[[u32; 4]]>]> =
        vec![vec![[u32::MAX, u32::MAX, u32::MAX, u32::MAX]; width].into_boxed_slice(); height]
            .into_boxed_slice();
    let mut visited_forwards_up: Box<[Box<[[u32; 4]]>]> =
        vec![vec![[u32::MAX, u32::MAX, u32::MAX, u32::MAX]; width].into_boxed_slice(); height]
            .into_boxed_slice();
    let mut visited_backwards_left: Box<[Box<[[u32; 4]]>]> =
        vec![vec![[u32::MAX, u32::MAX, u32::MAX, u32::MAX]; width].into_boxed_slice(); height]
            .into_boxed_slice();
    let mut visited_backwards_down: Box<[Box<[[u32; 4]]>]> =
        vec![vec![[u32::MAX, u32::MAX, u32::MAX, u32::MAX]; width].into_boxed_slice(); height]
            .into_boxed_slice();

    fn dijkstra(
        grid: &[Box<[char]>],
        visited: &mut Box<[Box<[[u32; 4]]>]>,
        start: (usize, usize),
        start_dir: usize,
        end: (usize, usize),
        end_dir: usize,
    ) -> u32 {
        let mut queue: MinMaxHeap<(u32, (usize, usize), usize)> = MinMaxHeap::new();
        queue.push((0, start, start_dir));

        let mut min_cost = u32::MAX;
        while !queue.is_empty() {
            let (cost, (x, y), direction) = queue.pop_min().unwrap();
            if cost >= visited[y][x][direction] {
                continue;
            }
            visited[y][x][direction] = cost;

            if (x, y) == end && direction == end_dir {
                if cost < min_cost {
                    min_cost = cost;
                }
                continue;
            }
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
        min_cost
    }

    fn find_nodes_on_shortest_path(
        forwards: Box<[Box<[[u32; 4]]>]>,
        backwards: Box<[Box<[[u32; 4]]>]>,
        nodes: &mut HashSet<(usize, usize)>,
        length: u32,
    ) {
        for y in 1..forwards.len() {
            for x in 1..forwards[y].len() {
                for dir in 0..4 {
                    let r_dir = (dir + 2) % 4;
                    let f_value = forwards[y][x][dir];
                    let r_value = backwards[y][x][r_dir];
                    if f_value != u32::MAX && r_value != u32::MAX && f_value + r_value == length {
                        nodes.insert((x, y));
                    }
                }
            }
        }
    }

    let min_cost_f_right = dijkstra(&grid, &mut visited_forwards_right, start, RIGHT, end, RIGHT);
    let min_cost_f_up = dijkstra(&grid, &mut visited_forwards_up, start, RIGHT, end, UP);

    let min_cost = min_cost_f_right.min(min_cost_f_up);

    let mut nodes_shortest_path: HashSet<(usize, usize)> = HashSet::new();

    if min_cost_f_right == min_cost {
        let min_cost_r = dijkstra(&grid, &mut visited_backwards_left, end, LEFT, start, LEFT);
        assert_eq!(min_cost, min_cost_r);

        find_nodes_on_shortest_path(
            visited_forwards_right,
            visited_backwards_left,
            &mut nodes_shortest_path,
            min_cost,
        );
    }
    if min_cost_f_up == min_cost {
        let min_cost_r = dijkstra(&grid, &mut visited_backwards_down, end, DOWN, start, LEFT);
        assert_eq!(min_cost, min_cost_r);

        find_nodes_on_shortest_path(
            visited_forwards_up,
            visited_backwards_down,
            &mut nodes_shortest_path,
            min_cost,
        );
    }

    let num_nodes = nodes_shortest_path.len() as u32;

    let elapsed = now.elapsed();
    (num_nodes, elapsed)
}

fn main() {
    let mut input = include_str!("../input.txt").lines();

    let (grid, width, height) = read_map(&mut input);

    let (start, end) = find_start_end(&grid, width, height);

    let (solution1, _) = solve1(&grid, width, height, start, end);
    let (solution2, _) = solve2(&grid, width, height, start, end);

    println!("day 16");
    println!("  - part 1: {}", solution1); // 122492
    println!("  - part 2: {}", solution2); // 520
}
