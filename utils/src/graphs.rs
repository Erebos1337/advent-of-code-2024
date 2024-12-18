use min_max_heap::MinMaxHeap;

use crate::grid::make_grid;

pub fn dijkstra_bool(grid: &[Box<[bool]>], start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut visited = make_grid(grid.len(), grid.len(), u32::MAX);

    let mut queue: MinMaxHeap<(u32, (usize, usize))> = MinMaxHeap::new();
    queue.push((0, start));

    while !queue.is_empty() {
        let (cost, (x, y)) = queue.pop_min().unwrap();
        
        if (x, y) == end {
            return cost;
        }

        if cost >= visited[y][x] {
            continue;
        }
        visited[y][x] = cost;


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