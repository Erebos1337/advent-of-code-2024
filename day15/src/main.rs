use std::str::Lines;

fn read_map(lines: &mut Lines) -> (Box<[Box<[char]>]>, usize, usize) {
    let mut grid_lines: Vec<Box<[char]>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        grid_lines.push(line.trim().chars().collect::<Box<[char]>>());
    }
    let grid = grid_lines.into_boxed_slice();
    let width = grid[0].len();
    let height = grid.len();

    return (grid, width, height);
}

fn read_movements(lines: &mut Lines) -> Box<[char]> {
    let mut movements_vec: Vec<char> = Vec::new();
    while let Some(line) = lines.next() {
        movements_vec.extend(line.trim().chars());
    }
    return movements_vec.into_boxed_slice();
}

fn find_robot(grid: &mut Box<[Box<[char]>]>, width: usize, height: usize) -> (usize, usize) {
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if grid[y][x] == '@' {
                grid[y][x] = '.';
                return (x, y);
            }
        }
    }
    panic!("Robot not found");
}

fn move_right(grid: &mut Box<[Box<[char]>]>, x: usize, y: usize) -> (usize, usize) {
    let new_x = x + 1;
    let mut should_move = false;

    if grid[y][new_x] == '.' {
        should_move = true;
    } else if grid[y][new_x] == 'O' {
        let mut free_x = new_x + 1;
        while grid[y][free_x] == 'O' {
            free_x += 1;
        }
        if grid[y][free_x] == '.' {
            grid[y][free_x] = 'O';
            grid[y][new_x] = '.';

            should_move = true;
        }
    }

    if should_move {
        return (new_x, y);
    } else {
        return (x, y);
    }
}

fn move_left(grid: &mut Box<[Box<[char]>]>, x: usize, y: usize) -> (usize, usize) {
    let new_x = x - 1;
    let mut should_move = false;

    if grid[y][new_x] == '.' {
        should_move = true;
    } else if grid[y][new_x] == 'O' {
        let mut free_x = new_x - 1;
        while grid[y][free_x] == 'O' {
            free_x -= 1;
        }
        if grid[y][free_x] == '.' {
            grid[y][free_x] = 'O';
            grid[y][new_x] = '.';

            should_move = true;
        }
    }

    if should_move {
        return (new_x, y);
    } else {
        return (x, y);
    }
}

fn move_down(grid: &mut Box<[Box<[char]>]>, x: usize, y: usize) -> (usize, usize) {
    let new_y: usize = y + 1;
    let mut should_move = false;

    if grid[new_y][x] == '.' {
        should_move = true;
    } else if grid[new_y][x] == 'O' {
        let mut free_y = new_y + 1;
        while grid[free_y][x] == 'O' {
            free_y += 1;
        }
        if grid[free_y][x] == '.' {
            grid[free_y][x] = 'O';
            grid[new_y][x] = '.';

            should_move = true;
        }
    }

    if should_move {
        return (x, new_y);
    } else {
        return (x, y);
    }
}

fn move_up(grid: &mut Box<[Box<[char]>]>, x: usize, y: usize) -> (usize, usize) {
    let new_y: usize = y - 1;
    let mut should_move = false;

    if grid[new_y][x] == '.' {
        should_move = true;
    } else if grid[new_y][x] == 'O' {
        let mut free_y = new_y - 1;
        while grid[free_y][x] == 'O' {
            free_y -= 1;
        }
        if grid[free_y][x] == '.' {
            grid[free_y][x] = 'O';
            grid[new_y][x] = '.';

            should_move = true;
        }
    }

    if should_move {
        return (x, new_y);
    } else {
        return (x, y);
    }
}

fn solve1(
    original_grid: &Box<[Box<[char]>]>,
    movements: &Box<[char]>,
    width: usize,
    height: usize,
) -> usize {
    let mut grid = original_grid.clone();
    let (mut x, mut y) = find_robot(&mut grid, width, height);

    for movement in movements {
        match movement {
            '>' => {
                (x, y) = move_right(&mut grid, x, y);
            }
            '<' => {
                (x, y) = move_left(&mut grid, x, y);
            }
            'v' => {
                (x, y) = move_down(&mut grid, x, y);
            }
            '^' => {
                (x, y) = move_up(&mut grid, x, y);
            }
            _ => panic!("Invalid movement"),
        }
    }

    let mut solution1 = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if grid[y][x] == 'O' {
                solution1 += 100 * y + x;
            }
        }
    }

    return solution1;
}

fn main() {
    let mut input = include_str!("../input.txt").lines();

    let (grid, width, height) = read_map(&mut input);
    let movements: Box<[char]> = read_movements(&mut input);

    println!("day 15");
    println!("  - part 1: {}", solve1(&grid, &movements, width, height)); // 1527563
}
