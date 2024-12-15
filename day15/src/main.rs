use std::str::Lines;

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

fn read_movements(lines: &mut Lines) -> Box<[char]> {
    let mut movements_vec: Vec<char> = Vec::new();
    for line in lines.by_ref() {
        movements_vec.extend(line.trim().chars());
    }
    movements_vec.into_boxed_slice()
}

fn find_robot(grid: &mut [Box<[char]>], width: usize, height: usize) -> (usize, usize) {
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

fn move_right(grid: &mut [Box<[char]>], x: usize, y: usize) -> (usize, usize) {
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
        (new_x, y)
    } else {
        (x, y)
    }
}

fn move_left(grid: &mut [Box<[char]>], x: usize, y: usize) -> (usize, usize) {
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
        (new_x, y)
    } else {
        (x, y)
    }
}

fn move_down(grid: &mut [Box<[char]>], x: usize, y: usize) -> (usize, usize) {
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
        (x, new_y)
    } else {
        (x, y)
    }
}

fn move_up(grid: &mut [Box<[char]>], x: usize, y: usize) -> (usize, usize) {
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
        (x, new_y)
    } else {
        (x, y)
    }
}

fn solve1(original_grid: &[Box<[char]>], movements: &[char], width: usize, height: usize) -> usize {
    let mut grid = original_grid.to_vec().clone().into_boxed_slice();
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

    let mut solution = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if grid[y][x] == 'O' {
                solution += 100 * y + x;
            }
        }
    }

    solution
}

fn move_right_wide(grid: &mut Box<[Box<[char]>]>, x: usize, y: usize) -> (usize, usize) {
    let new_x = x + 1;
    let mut should_move = false;

    if grid[y][new_x] == '.' {
        should_move = true;
    } else if grid[y][new_x] == '[' {
        let mut free_x = new_x + 2;
        while grid[y][free_x] == '[' {
            free_x += 2;
        }
        if grid[y][free_x] == '.' {
            grid[y][new_x] = '.';
            for i in (new_x + 1..free_x).step_by(2) {
                grid[y][i] = '[';
                grid[y][i + 1] = ']';
            }

            should_move = true;
        }
    }

    if should_move {
        (new_x, y)
    } else {
        (x, y)
    }
}

fn move_left_wide(grid: &mut Box<[Box<[char]>]>, x: usize, y: usize) -> (usize, usize) {
    let new_x = x - 1;
    let mut should_move = false;

    if grid[y][new_x] == '.' {
        should_move = true;
    } else if grid[y][new_x] == ']' {
        let mut free_x = new_x - 2;
        while grid[y][free_x] == ']' {
            free_x -= 2;
        }
        if grid[y][free_x] == '.' {
            grid[y][new_x] = '.';
            for i in (free_x..new_x).step_by(2) {
                grid[y][i] = '[';
                grid[y][i + 1] = ']';
            }

            should_move = true;
        }
    }

    if should_move {
        (new_x, y)
    } else {
        (x, y)
    }
}

fn move_down_wide(grid: &mut Box<[Box<[char]>]>, x: usize, y: usize) -> (usize, usize) {
    let new_y: usize = y + 1;
    let mut should_move = false;

    fn can_move_down(grid: &[Box<[char]>], x: usize, y: usize) -> bool {
        match grid[y][x] {
            '.' => true,
            '[' => can_move_down(grid, x, y + 1) && can_move_down(grid, x + 1, y + 1),
            ']' => can_move_down(grid, x, y + 1) && can_move_down(grid, x - 1, y + 1),
            _ => false,
        }
    }

    fn collect_updates(
        grid: &mut Box<[Box<[char]>]>,
        resets: &mut Vec<(usize, usize)>,
        updates: &mut Vec<(usize, usize, char)>,
        x: usize,
        y: usize,
    ) {
        let new_y = y + 1;
        let curr_char = grid[y][x];
        resets.push((x, y));
        updates.push((x, new_y, curr_char));

        let next_char = grid[new_y][x];

        if next_char != '.' {
            collect_updates(grid, resets, updates, x, new_y);
        }

        if curr_char == '[' && next_char == ']' {
            collect_updates(grid, resets, updates, x - 1, new_y);
        } else if curr_char == ']' && next_char == '[' {
            collect_updates(grid, resets, updates, x + 1, new_y);
        }
    }

    fn push_down(grid: &mut Box<[Box<[char]>]>, x: usize, y: usize) {
        let new_y = y + 1;

        let mut resets: Vec<(usize, usize)> = Vec::new();
        let mut updates: Vec<(usize, usize, char)> = Vec::new();

        collect_updates(grid, &mut resets, &mut updates, x, new_y);

        let above_char = grid[new_y][x];
        if above_char == '[' {
            collect_updates(grid, &mut resets, &mut updates, x + 1, new_y);
        } else if above_char == ']' {
            collect_updates(grid, &mut resets, &mut updates, x - 1, new_y);
        }

        for (x, y) in resets {
            grid[y][x] = '.';
        }
        for (x, y, c) in updates {
            grid[y][x] = c;
        }
    }

    if grid[new_y][x] == '.' {
        should_move = true;
    } else if grid[new_y][x] == '[' {
        if can_move_down(grid, x, new_y) && can_move_down(grid, x + 1, new_y) {
            push_down(grid, x, y);

            should_move = true;
        }
    } else if grid[new_y][x] == ']' {
        if can_move_down(grid, x, new_y) && can_move_down(grid, x - 1, new_y) {
            push_down(grid, x, y);

            should_move = true;
        }
    }

    if should_move {
        (x, new_y)
    } else {
        (x, y)
    }
}

fn move_up_wide(grid: &mut Box<[Box<[char]>]>, x: usize, y: usize) -> (usize, usize) {
    let new_y: usize = y - 1;
    let mut should_move = false;

    fn can_move_up(grid: &[Box<[char]>], x: usize, y: usize) -> bool {
        match grid[y][x] {
            '.' => true,
            '[' => can_move_up(grid, x, y - 1) && can_move_up(grid, x + 1, y - 1),
            ']' => can_move_up(grid, x, y - 1) && can_move_up(grid, x - 1, y - 1),
            _ => false,
        }
    }

    fn collect_updates(
        grid: &mut Box<[Box<[char]>]>,
        resets: &mut Vec<(usize, usize)>,
        updates: &mut Vec<(usize, usize, char)>,
        x: usize,
        y: usize,
    ) {
        let new_y = y - 1;
        let curr_char = grid[y][x];
        resets.push((x, y));
        updates.push((x, new_y, curr_char));

        let next_char = grid[new_y][x];

        if next_char != '.' {
            collect_updates(grid, resets, updates, x, new_y);
        }

        if curr_char == '[' && next_char == ']' {
            collect_updates(grid, resets, updates, x - 1, new_y);
        } else if curr_char == ']' && next_char == '[' {
            collect_updates(grid, resets, updates, x + 1, new_y);
        }
    }

    fn push_up(grid: &mut Box<[Box<[char]>]>, x: usize, y: usize) {
        let new_y = y - 1;

        let mut resets: Vec<(usize, usize)> = Vec::new();
        let mut updates: Vec<(usize, usize, char)> = Vec::new();

        collect_updates(grid, &mut resets, &mut updates, x, new_y);

        let above_char = grid[new_y][x];
        if above_char == '[' {
            collect_updates(grid, &mut resets, &mut updates, x + 1, new_y);
        } else if above_char == ']' {
            collect_updates(grid, &mut resets, &mut updates, x - 1, new_y);
        }

        for (x, y) in resets {
            grid[y][x] = '.';
        }
        for (x, y, c) in updates {
            grid[y][x] = c;
        }
    }

    if grid[new_y][x] == '.' {
        should_move = true;
    } else if grid[new_y][x] == '[' {
        if can_move_up(grid, x, new_y) && can_move_up(grid, x + 1, new_y) {
            push_up(grid, x, y);

            should_move = true;
        }
    } else if grid[new_y][x] == ']' {
        if can_move_up(grid, x, new_y) && can_move_up(grid, x - 1, new_y) {
            push_up(grid, x, y);

            should_move = true;
        }
    }

    if should_move {
        (x, new_y)
    } else {
        (x, y)
    }
}

fn solve2(
    original_grid: &[Box<[char]>],
    movements: &[char],
    original_width: usize,
    height: usize,
) -> usize {
    let width = original_width * 2;
    let mut grid = vec![vec!['.'; width].into_boxed_slice(); height].into_boxed_slice();

    let (mut rx, mut ry) = (0, 0);
    {
        for y in 0..height {
            for x in 0..original_width {
                let c = original_grid[y][x];
                match c {
                    '#' => {
                        grid[y][2 * x] = '#';
                        grid[y][2 * x + 1] = '#';
                    }
                    'O' => {
                        grid[y][2 * x] = '[';
                        grid[y][2 * x + 1] = ']';
                    }
                    '@' => {
                        (rx, ry) = (2 * x, y);
                        grid[y][2 * x] = '.';
                        grid[y][2 * x + 1] = '.';
                    }
                    _ => continue,
                }
            }
        }
    }

    for movement in movements {
        match movement {
            '>' => {
                (rx, ry) = move_right_wide(&mut grid, rx, ry);
            }
            '<' => {
                (rx, ry) = move_left_wide(&mut grid, rx, ry);
            }
            'v' => {
                (rx, ry) = move_down_wide(&mut grid, rx, ry);
            }
            '^' => {
                (rx, ry) = move_up_wide(&mut grid, rx, ry);
            }
            _ => panic!("Invalid movement"),
        }
    }

    let mut solution = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if grid[y][x] == '[' {
                solution += 100 * y + x;
            }
        }
    }

    solution
}

fn main() {
    let mut input = include_str!("../input.txt").lines();

    let (grid, width, height) = read_map(&mut input);
    let movements: Box<[char]> = read_movements(&mut input);

    println!("day 15");
    println!("  - part 1: {}", solve1(&grid, &movements, width, height)); // 1527563
    println!("  - part 2: {}", solve2(&grid, &movements, width, height)); // 1521635
}
