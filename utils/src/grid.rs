pub fn print_char_grid(grid: &Box<[Box<[char]>]>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", &grid[y][x]);
        }
        println!();
    }
}