use utils::inputs::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("./day10/input.txt").unwrap().flatten().collect();
    let input: Box<[Box<[u32]>]>;
    {
        let mut lines_vec: Vec<Box<[u32]>> = Vec::new();
        for line in lines {
            lines_vec.push(
                line.chars()
                    .flat_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>()
                    .into_boxed_slice(),
            );
        }
        input = lines_vec.into_boxed_slice();
    }

    let solution1: u32 = solve1(&input);
    let solution2: u32 = solve2(&input);

    println!("day 10");
    println!("  - part 1: {}", solution1); // 514
    println!("  - part 2: {}", solution2); //
}

fn collect_neighbors_with_value(
    input: &Box<[Box<[u32]>]>,
    (x, y): (usize, usize),
    height: u32,
) -> Vec<(usize, usize, u32)> {
    let width = input.len();
    let mut neighbors: Vec<(usize, usize, u32)> = Vec::new();
    if x > 0 && input[y][x - 1] == height {
        neighbors.push((x - 1, y, height));
    }
    if y > 0 && input[y - 1][x] == height {
        neighbors.push((x, y - 1, height));
    }
    if x < width - 1 && input[y][x + 1] == height {
        neighbors.push((x + 1, y, height));
    }
    if y < width - 1 && input[y + 1][x] == height {
        neighbors.push((x, y + 1, height));
    }
    return neighbors;
}

fn reset_visited(visited: &mut Box<[u64]>) {
    for y in 0..visited.len() {
        visited[y] = 0;
    }
}

fn set_visited(visited: &mut Box<[u64]>, y: usize, x: usize) {
    visited[y] |= 1 << x;
}

fn is_visited(visited: &Box<[u64]>, y: usize, x: usize) -> bool {
    return visited[y] & (1 << x) != 0;
}

fn find_trails(input: &Box<[Box<[u32]>]>, visited: &mut Box<[u64]>, pos: (usize, usize)) -> u32 {
    reset_visited(visited);
    let mut num_trails: u32 = 0;

    let mut stack: Vec<(usize, usize, u32)> = Vec::from([(pos.0, pos.1, 0)]);

    while !stack.is_empty() {
        let (x, y, height) = stack.pop().unwrap();

        if is_visited(visited, y, x) {
            continue;
        }
        set_visited(visited, y, x);

        if height == 9 {
            num_trails += 1;
            continue;
        }

        let neighbors = collect_neighbors_with_value(input, (x, y), height + 1);
        for neighbor in neighbors {
            stack.push(neighbor);
        }
    }

    return num_trails;
}

fn solve1(input: &Box<[Box<[u32]>]>) -> u32 {
    let width = input[0].len();

    let mut solution: u32 = 0;

    let mut visited: Box<[u64]> = vec![0; width].into_boxed_slice();

    // for each 0, do dfs to find number of trails
    for y in 0..width {
        for x in 0..width {
            if input[y][x] == 0 {
                solution += find_trails(input, &mut visited, (x, y));
            }
        }
    }

    return solution;
}

fn solve2(input: &Box<[Box<[u32]>]>) -> u32 {
    let width = input[0].len();

    let mut solution: u32 = 0;

    // init trail paths
    let mut trail_paths: Box<[Box<[u32]>]> = input.clone();
    for y in 0..width {
        for x in 0..width {
            if input[y][x] == 9 {
                trail_paths[y][x] = 1;
            } else {
                trail_paths[y][x] = 0;
            }
        }
    }

    // from top to bottom save number of trails from that position
    for height in (1..=9).rev() {
        for y in 0..width {
            for x in 0..width {
                if input[y][x] == height {
                    let num_trails = trail_paths[y][x];
                    let neighbors = collect_neighbors_with_value(input, (x, y), height - 1);
                    for (n_x, n_y, _) in neighbors {
                        trail_paths[n_y][n_x] += num_trails;
                    }
                }
            }
        }
    }

    // count number of trails starting at 0
    for y in 0..width {
        for x in 0..width {
            if input[y][x] == 0 {
                solution += trail_paths[y][x];
            }
        }
    }

    return solution;
}
