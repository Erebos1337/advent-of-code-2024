use std::vec;

use utils::inputs::read_lines;

fn collect_num_neighbors_with_value(
    input: &Box<[Box<[char]>]>,
    (x, y): (usize, usize),
    value: char,
) -> u32 {
    let width = input[0].len();
    let height = input.len();
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
    if y < height - 1 && input[y + 1][x] == value {
        count += 1;
    }
    return count;
}

fn collect_neighbors_with_value(
    input: &Box<[Box<[char]>]>,
    (x, y): (usize, usize),
    value: char,
) -> Vec<(usize, usize)> {
    let height = input.len();
    let width = input[0].len();
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
    if y < height - 1 && input[y + 1][x] == value {
        neighbors.push((x, y + 1));
    }
    return neighbors;
}

fn collect_neighbors_and_fences(
    input: &Box<[Box<[char]>]>,
    (x, y): (usize, usize),
    value: char,
) -> (Vec<(usize, usize)>, Vec<(usize, usize, usize, usize)>) {
    let height = input.len();
    let width = input[0].len();
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let mut fences: Vec<(usize, usize, usize, usize)> = Vec::new();
    if x > 0 && input[y][x - 1] == value {
        neighbors.push((x - 1, y));
    } else {
        fences.push((x, y, x, y + 1));
    }
    if y > 0 && input[y - 1][x] == value {
        neighbors.push((x, y - 1));
    } else {
        fences.push((x, y, x + 1, y));
    }
    if x < width - 1 && input[y][x + 1] == value {
        neighbors.push((x + 1, y));
    } else {
        fences.push((x + 1, y, x + 1, y + 1));
    }
    if y < height - 1 && input[y + 1][x] == value {
        neighbors.push((x, y + 1));
    } else {
        fences.push((x, y + 1, x + 1, y + 1));
    }
    return (neighbors, fences);
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

fn solve2(input: &Box<[Box<[char]>]>) -> u32 {
    let mut visited: Box<[Box<[bool]>]> =
        vec![vec![false; input[0].len()].into_boxed_slice(); input.len()].into_boxed_slice();

    let mut solution: u32 = 0;

    fn scan_region(
        input: &Box<[Box<[char]>]>,
        (start_x, start_y): (usize, usize),
        visited: &mut Box<[Box<[bool]>]>,
    ) -> u32 {
        let mut stack: Vec<(usize, usize)> = Vec::from([(start_x, start_y)]);
        let mut fences: Vec<(usize, usize, usize, usize)> = Vec::new();
        let mut area: u32 = 0;
        let value = input[start_y][start_x];

        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();

            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;

            area += 1;

            let (neighbors, new_fences) = collect_neighbors_and_fences(input, (x, y), value);

            fences.extend_from_slice(&new_fences);
            for neighbor in neighbors {
                stack.push(neighbor);
            }
        }
        let num_sides: u32 = count_fence_sides(input, fences);
        return area * num_sides;
    }

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            solution += scan_region(input, (x, y), &mut visited);
        }
    }

    return solution;
}

fn remove_adjacent_fences(
    input: &Box<[Box<[char]>]>,
    fences: &mut Vec<(usize, usize, usize, usize)>,
    (start_x, start_y, end_x, end_y): (usize, usize, usize, usize),
    is_vertical: bool,
    forwards: bool,
) {
    let height = input.len();
    let width = input[0].len();
    let fence_to_find: (usize, usize, usize, usize);
    if is_vertical {
        if forwards {
            if end_y >= height {
                return;
            }
            if start_x > 0 && start_x < width {
                // check for corner
                if input[start_y][start_x] == input[start_y + 1][start_x - 1]
                    || input[start_y][start_x - 1] == input[start_y + 1][start_x]
                {
                    return;
                }
            }
            fence_to_find = (end_x, end_y, end_x, end_y + 1);
        } else {
            if start_y < 1 {
                return;
            }
            if start_x > 0 && start_x < width {
                // check for corner
                if input[start_y][start_x] == input[start_y - 1][start_x - 1]
                    || input[start_y][start_x - 1] == input[start_y - 1][start_x]
                {
                    return;
                }
            }
            fence_to_find = (start_x, start_y - 1, start_x, start_y);
        }
    } else {
        if forwards {
            if end_x >= width {
                return;
            }
            if start_y > 0 && start_y < height {
                // check for corner
                if input[start_y][start_x] == input[start_y - 1][start_x + 1]
                    || input[start_y][start_x + 1] == input[start_y - 1][start_x]
                {
                    return;
                }
            }
            fence_to_find = (end_x, end_y, end_x + 1, end_y);
        } else {
            if start_x < 1 {
                return;
            }
            if start_y > 0 && start_y < height {
                // check for corner
                if input[start_y][start_x] == input[start_y - 1][start_x - 1]
                    || input[start_y][start_x - 1] == input[start_y - 1][start_x]
                {
                    return;
                }
            }
            fence_to_find = (start_x - 1, start_y, start_x, start_y);
        }
    }
    let index = fences.iter().position(|fence| *fence == fence_to_find);
    if let Some(index) = index {
        let new_fence = fences.swap_remove(index);
        remove_adjacent_fences(input, fences, new_fence, is_vertical, forwards);
    }
}

fn count_fence_sides(
    input: &Box<[Box<[char]>]>,
    mut fences: Vec<(usize, usize, usize, usize)>,
) -> u32 {
    let mut sides: u32 = 0;

    while !fences.is_empty() {
        sides += 1;

        let fence = fences.pop().unwrap();
        let is_vertical = fence.0 == fence.2;
        remove_adjacent_fences(input, &mut fences, fence, is_vertical, true);
        remove_adjacent_fences(input, &mut fences, fence, is_vertical, false);
    }
    return sides;
}

fn main() {
    let width: usize = 140;
    let height: usize = 140;
    let lines: Vec<String> = read_lines("./day12/input.txt")
        .unwrap()
        .flatten()
        .take(height)
        .collect();
    let input: Box<[Box<[char]>]>;
    {
        let mut lines_vec: Vec<Box<[char]>> = Vec::new();
        for line in lines {
            lines_vec.push(
                line.chars()
                    .take(width)
                    .collect::<Vec<char>>()
                    .into_boxed_slice(),
            );
        }
        input = lines_vec.into_boxed_slice();
    }

    let solution1: u32 = solve1(&input);
    let solution2: u32 = solve2(&input);

    println!("day 12");
    println!("  - part 1: {}", solution1); // 1573474
    println!("  - part 2:  {}", solution2); //  966476
}
