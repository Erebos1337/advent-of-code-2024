use std::vec;

use utils::inputs::read_lines;

fn collect_num_neighbors_with_value(
    input: &Box<[Box<[char]>]>,
    (x, y): (usize, usize),
    value: char,
) -> u32 {
    let width = input.len();
    let mut count: u32 = 0;
    if x > 0 && input[y][x - 1] == value {
        count += 1;
    }
    if y > 0 && input[y - 1][x] == value {
        count += 1;
    }
    if x < width - 1 && input[y][x + 1] == value {
        count += 1;
    }
    if y < width - 1 && input[y + 1][x] == value {
        count += 1;
    }
    return count;
}

fn collect_neighbors_with_value(
    input: &Box<[Box<[char]>]>,
    (x, y): (usize, usize),
    value: char,
) -> Vec<(usize, usize)> {
    let width = input.len();
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if x > 0 && input[y][x - 1] == value {
        neighbors.push((x - 1, y));
    }
    if y > 0 && input[y - 1][x] == value {
        neighbors.push((x, y - 1));
    }
    if x < width - 1 && input[y][x + 1] == value {
        neighbors.push((x + 1, y));
    }
    if y < width - 1 && input[y + 1][x] == value {
        neighbors.push((x, y + 1));
    }
    return neighbors;
}

fn solve1(input: &Box<[Box<[char]>]>) -> u32 {
    let mut fence_count: Box<[Box<[u32]>]> =
        vec![vec![4; input[0].len()].into_boxed_slice(); input.len()].into_boxed_slice();

    let mut visited: Box<[Box<[bool]>]> =
        vec![vec![false; input[0].len()].into_boxed_slice(); input.len()].into_boxed_slice();

    let mut solution: u32 = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let value = input[y][x];
            fence_count[y][x] -= collect_num_neighbors_with_value(input, (x, y), value);
        }
    }

    fn scan_region(
        input: &Box<[Box<[char]>]>,
        (start_x, start_y): (usize, usize),
        visited: &mut Box<[Box<[bool]>]>,
        fence_count: &Box<[Box<[u32]>]>,
    ) -> u32 {
        let mut stack: Vec<(usize, usize)> = Vec::from([(start_x, start_y)]);
        let mut fences: u32 = 0;
        let mut area: u32 = 0;
        let value = input[start_y][start_x];

        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();

            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;

            fences += fence_count[y][x];
            area += 1;

            for neighbor in collect_neighbors_with_value(input, (x, y), value) {
                stack.push(neighbor);
            }
        }

        return area * fences;
    }

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            solution += scan_region(input, (x, y), &mut visited, &fence_count);
        }
    }

    return solution;
}

fn main() {
    let lines: Vec<String> = read_lines("./day12/input.txt").unwrap().flatten().collect();
    let input: Box<[Box<[char]>]>;
    {
        let mut lines_vec: Vec<Box<[char]>> = Vec::new();
        for line in lines {
            lines_vec.push(line.chars().collect::<Vec<char>>().into_boxed_slice());
        }
        input = lines_vec.into_boxed_slice();
    }

    let solution1: u32 = solve1(&input);

    println!("day 10");
    println!("  - part 1: {}", solution1); // 1573474
}
