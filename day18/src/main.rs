use utils::{graphs::dijkstra_bool, grid::make_grid};

fn read_positions(input: &str) -> Box<[(usize, usize)]> {
    input
        .lines()
        .flat_map(|s| s.trim().split_once(','))
        .map(|(x, y)| ((x.parse().unwrap()), (y.parse().unwrap())))
        .collect()
}

fn solve1(positions: &[(usize, usize)]) -> u32 {
    let size = 71;
    let mut grid = make_grid(size, size, true);

    let num_bytes = 1024;
    for (x, y) in positions.iter().take(num_bytes) {
        grid[*y][*x] = false;
    }

    dijkstra_bool(&grid, (0, 0), (size - 1, size - 1))
}

fn solve2(positions: &[(usize, usize)]) -> (usize, usize) {
    let size = 71;
    let mut grid = make_grid(size, size, true);

    // save bytes known from part 1
    let save_bytes = 1024;
    for (x, y) in positions.iter().take(save_bytes) {
        grid[*y][*x] = false;
    }

    for (x, y) in positions.iter().skip(save_bytes) {
        grid[*y][*x] = false;
        if dijkstra_bool(&grid, (0, 0), (size - 1, size - 1)) == u32::MAX {
            return (*x, *y);
        }
    }

    (0, 0)
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
