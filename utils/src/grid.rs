pub fn print_char_grid(grid: &[Box<[char]>]) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", &grid[y][x]);
        }
        println!();
    }
}

pub fn print_char_grid_mark(grid: &[Box<[char]>], (mx, my): (usize, usize)) {
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

pub fn make_grid<T: Clone>(width: usize, height: usize, default: T) -> Box<[Box<[T]>]> {
    vec![vec![default; width].into_boxed_slice(); height].into_boxed_slice()
}