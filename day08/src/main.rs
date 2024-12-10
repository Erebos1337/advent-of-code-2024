use std::collections::HashMap;

use utils::inputs::read_lines;

fn calc_new_pos(x: i32, y:i32, dx:i32, dy:i32, width: i32) -> usize {
    let new_x = x + dx;
    let new_y = y + dy;
    if new_x >= 0 && new_y >= 0 && new_x < width && new_y < width {
        (new_y * width + new_x) as usize
    } else {
        usize::MAX
    }
}

fn main() {
    let lines: Vec<String> = read_lines("./input.txt").unwrap().flatten().collect();

    let width = lines.len();
    let mut char_array: Box<[char]> = vec!['.'; width * width].into_boxed_slice();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        let offset: usize = width * y;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push((x, y));
                char_array[offset + x] = c;
            }
        }
    }
    let mut antinodes: Box<[bool]> = vec![false; width * width].into_boxed_slice();
    let w: i32 = width as i32;
    for (_, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;
                let mut mul:i32 = 1;
                let mut new_pos = calc_new_pos(x1 as i32, y1 as i32, dx, dy, w);
                while new_pos != usize::MAX {
                    antinodes[new_pos] = true;
                    mul += 1;
                    new_pos = calc_new_pos(x1 as i32, y1 as i32, mul*dx, mul*dy, w);
                }
                mul = -1;
                new_pos = calc_new_pos(x2 as i32, y2 as i32, mul*dx, mul*dy, w);
                while new_pos != usize::MAX {
                    antinodes[new_pos] = true;
                    mul -= 1;
                    new_pos = calc_new_pos(x2 as i32, y2 as i32, mul*dx, mul*dy, w);
                }
            }
        }
    }
    let mut solution:u32 = 0;
    for antinode in antinodes {
        if antinode {
            solution+=1;
        }
    }
    println!("{}", solution);
}
