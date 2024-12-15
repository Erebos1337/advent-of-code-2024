pub fn print_char_grid(grid: &Box<[Box<[char]>]>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", &grid[y][x]);
        }
        println!();
    }
}

pub fn print_char_grid_mark(grid: &Box<[Box<[char]>]>, (mx, my): (usize, usize)) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if x == mx && y == my {
                print!("@");
            } else {
                print!("{}", &grid[y][x]);
            }
        }
        println!();
    }
}
