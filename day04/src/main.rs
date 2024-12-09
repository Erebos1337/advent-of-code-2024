use regex::Regex;
use utils::inputs::read_lines;

pub fn main() {
    let mut result: u32 = 0;
    let mut result2: u32 = 0;

    let regex_fwd = Regex::new(r"XMAS").unwrap();
    let regex_bwd = Regex::new(r"SAMX").unwrap();

    let lines = read_lines("./day04/input.txt").unwrap().flatten();

    let mut char_array: Vec<Vec<char>> = Vec::new();
    let mut lines_vec: Vec<String> = Vec::new();

    for line in lines {
        // add horizontal lines to list of lines
        lines_vec.push(line.clone());

        // split line into characters
        let line_vec: Vec<char> = line.chars().collect();
        // add characters line to 2d grid of characters
        char_array.push(line_vec);
    }

    let last_x = char_array[0].len()-1;
    let last_y = char_array.len()-1;

    // capture vertical lines
    for x in 0..=last_x {
        let mut line = String::new();
        for y in 0..=last_y {
            line.push(char_array[y][x]);
        }
        lines_vec.push(line);
    }

    // capture diagonal lines from top left to bottom right
    for x in 0..=last_x {
        let mut line = String::new();
        for y in 0..=last_x - x {
            line.push(char_array[y][x+y]);
        }
        lines_vec.push(line);
    }
    for y in 1..=last_y {
        let mut line = String::new();
        for x in 0..=last_y - y {
            line.push(char_array[y+x][x]);
        }
        lines_vec.push(line);
    }

    // capture diagonal lines from bottom left to top right
    for x in 0..=last_x {
        let mut line = String::new();
        for y in 0..=last_x - x {
            line.push(char_array[last_y-y][x+y]);
        }
        lines_vec.push(line);
    }
    for y in 1..=last_y {
        let mut line = String::new();
        for x in 0..=last_y - y {
            line.push(char_array[last_x - y - x][x]);
        }
        lines_vec.push(line);
    }

    for line in lines_vec {
        result += regex_fwd.captures_iter(&line).count() as u32;
        result += regex_bwd.captures_iter(&line).count() as u32;
    }

    for x in 1..=last_x-1 {
        for y in 1..=last_y-1 {
            // find mid of x-mas
            if char_array[y][x] == 'A' {
                // check if top left to bottom right is mas or sam
                if (char_array[y-1][x-1] == 'M' && char_array[y+1][x+1] == 'S')
                || (char_array[y-1][x-1] == 'S' && char_array[y+1][x+1] == 'M') {
                    // check if bottom left to top right is mas or sam
                    if (char_array[y+1][x-1] == 'M' && char_array[y-1][x+1] == 'S')
                    || (char_array[y+1][x-1] == 'S' && char_array[y-1][x+1] == 'M') {
                        result2 += 1;
                    }
                }

            }
        }
    }

    println!("day  4");
    println!("  - part 1: {}", result); // 2593
    println!("  - part 2: {}", result2); // 1950
}
