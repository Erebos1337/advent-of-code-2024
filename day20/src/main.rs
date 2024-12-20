use std::collections::VecDeque;

use utils::grid::make_grid;

fn prime_distance(grid: &[Box<[char]>], start: (usize, usize)) -> Box<[Box<[u32]>]> {
    let mut visited: Box<[Box<[u32]>]> = make_grid(grid[0].len(), grid.len(), u32::MAX);

    let mut queue: VecDeque<(u32, (usize, usize))> = VecDeque::new();
    queue.push_back((0, start));

    while !queue.is_empty() {
        let (distance, (x, y)) = queue.pop_front().unwrap();
        if visited[y][x] <= distance {
            continue;
        }
        visited[y][x] = distance;

        if x > 0 && grid[y][x - 1] != '#' {
            queue.push_back((distance + 1, (x - 1, y)));
        }
        if x + 1 < grid[0].len() && grid[y][x + 1] != '#' {
            queue.push_back((distance + 1, (x + 1, y)));
        }
        if y > 0 && grid[y - 1][x] != '#' {
            queue.push_back((distance + 1, (x, y - 1)));
        }
        if y + 1 < grid.len() && grid[y + 1][x] != '#' {
            queue.push_back((distance + 1, (x, y + 1)));
        }
    }

    return visited;
}

fn find_start_end(grid: &[Box<[char]>]) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (x, y);
            } else if cell == 'E' {
                end = (x, y);
            }
        }
    }

    (start, end)
}

fn solve1(grid: &[Box<[char]>], start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut count_shortcuts = 0u32;

    let distances_forwards = prime_distance(&grid, start);
    let distances_backwards = prime_distance(&grid, end);
    let shortest_path = distances_forwards[end.1][end.0];

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x] != '#' {
                continue;
            }

            // horizontal
            if grid[y][x - 1] != '#' && grid[y][x + 1] != '#' {
                {
                    // l -> r
                    let start = distances_forwards[y][x - 1];
                    let end = distances_backwards[y][x + 1];
                    if start + end + 2 <= shortest_path - 100 {
                        count_shortcuts += 1;
                    }
                }
                {
                    // r -> l
                    let start = distances_forwards[y][x + 1];
                    let end = distances_backwards[y][x - 1];
                    if start + end + 2 <= shortest_path - 100 {
                        count_shortcuts += 1;
                    }
                }
            }
            // vertical
            if grid[y - 1][x] != '#' && grid[y + 1][x] != '#' {
                {
                    // u -> d
                    let start = distances_forwards[y - 1][x];
                    let end = distances_backwards[y + 1][x];
                    if start + end + 2 <= shortest_path - 100 {
                        count_shortcuts += 1;
                    }
                }
                {
                    // d -> u
                    let start = distances_forwards[y + 1][x];
                    let end = distances_backwards[y - 1][x];
                    if start + end + 2 <= shortest_path - 100 {
                        count_shortcuts += 1;
                    }
                }
            }
        }
    }
    count_shortcuts
}

fn solve2(grid: &[Box<[char]>], start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut count_shortcuts = 0u32;

    let distances_forwards = prime_distance(&grid, start);
    let distances_backwards = prime_distance(&grid, end);
    let shortest_path = distances_forwards[end.1][end.0];

    let height = grid.len();
    let width = grid[0].len();
    for start_y in 1..height - 1 {
        for start_x in 1..width - 1 {
            if grid[start_y][start_x] == '#' {
                continue;
            }

            let x_min = match start_x {
                x if x > 21 => x - 20,
                _ => 1,
            };
            let x_max = match start_x {
                x if x < width - 22 => x + 20,
                _ => width - 2,
            };
            let y_min = match start_y {
                y if y > 21 => y - 20,
                _ => 1,
            };
            let y_max = match start_y {
                y if y < height - 22 => y + 20,
                _ => height - 2,
            };

            for end_y in y_min..=y_max {
                for end_x in x_min..=x_max {
                    if grid[end_y][end_x] == '#' {
                        continue;
                    }

                    let shortcut_length =
                        (end_y.abs_diff(start_y) + end_x.abs_diff(start_x)) as u32;
                    if shortcut_length > 20 {
                        continue;
                    }

                    let start = distances_forwards[start_y][start_x];
                    let end = distances_backwards[end_y][end_x];
                    if start + end + shortcut_length <= shortest_path - 100 {
                        count_shortcuts += 1;
                    }
                }
            }
        }
    }
    count_shortcuts
}

fn main() {
    let input = include_str!("../input.txt");
    let grid: Box<[Box<[char]>]> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let (start, end) = find_start_end(&grid);

    let solution1 = solve1(&grid, start, end);
    let solution2 = solve2(&grid, start, end);

    println!("day 20");
    println!("  - part 1: {}", solution1); // 1499
    println!("  - part 2: {}", solution2); // 1027164
}
