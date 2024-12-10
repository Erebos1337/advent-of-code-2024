use std::collections::HashMap;

use utils::inputs::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("./day08/input.txt").unwrap().flatten().collect();

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
    for (_, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;
            }
        }
    }
}
