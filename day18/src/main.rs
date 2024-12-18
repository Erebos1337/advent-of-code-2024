use std::u32;

use min_max_heap::MinMaxHeap;

fn read_positions(input: &str) -> Box<[(usize, usize)]> {
    input
        .lines()
        .flat_map(|s| s.trim().split_once(','))
        .map(|(x, y)| ((x.parse().unwrap()), (y.parse().unwrap())))
        .collect::<Vec<(usize, usize)>>()
        .into_boxed_slice()
}

fn dijkstra(grid: &[Box<[bool]>], start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut visited =
        vec![vec![u32::MAX; grid.len()].into_boxed_slice(); grid.len()].into_boxed_slice();

    let mut queue: MinMaxHeap<(u32, (usize, usize))> = MinMaxHeap::new();
    queue.push((0, start));

    while !queue.is_empty() {
        let (cost, (x, y)) = queue.pop_min().unwrap();
        if cost >= visited[y][x] {
            continue;
        }
        visited[y][x] = cost;

        if (x, y) == end {
            return cost;
        }

        // right
        if x + 1 < grid.len() && grid[y][x + 1] {
            queue.push((cost + 1, (x + 1, y)));
        }
        // down
        if y + 1 < grid.len() && grid[y + 1][x] {
            queue.push((cost + 1, (x, y + 1)));
        }
        // left
        if x > 0 && grid[y][x - 1] {
            queue.push((cost + 1, (x - 1, y)));
        }
        // up
        if y > 0 && grid[y - 1][x] {
            queue.push((cost + 1, (x, y - 1)));
        }
    }
    u32::MAX
}

fn solve1(positions: &[(usize, usize)]) -> u32 {
    let size = 71;
    let mut grid = vec![vec![true; size].into_boxed_slice(); size].into_boxed_slice();
    
    let num_bytes = 1024;
    for (x, y) in positions.iter().take(num_bytes) {
        grid[*y][*x] = false;
    }

    dijkstra(&grid, (0, 0), (size - 1, size - 1))
}

fn solve2(positions: &[(usize, usize)]) -> (usize,usize) {
    let size = 71;
    let mut grid = vec![vec![true; size].into_boxed_slice(); size].into_boxed_slice();
    
    // save bytes known from part 1
    let save_bytes = 1024;
    for (x, y) in positions.iter().take(save_bytes) {
        grid[*y][*x] = false;
    }

    for (x, y) in positions.iter().skip(save_bytes) {
        grid[*y][*x] = false;
        if dijkstra(&grid, (0, 0), (size - 1, size - 1)) == u32::MAX {
            return (*x, *y);
        }
    }

    (0,0)
}

fn main() {
    let input = include_str!("../input.txt");

    let positions = read_positions(input);

    let solution1 = solve1(&positions);
    let solution2 = solve2(&positions);

    println!("day 18");
    println!("  - part 1: {}", solution1); // 372
    println!("  - part 2: {:?}", solution2); // 25,6
}
