use utils::inputs::read_lines;

fn is_file(idx: usize) -> bool {
    idx % 2 == 0
}

fn file_index(idx: usize) -> u64 {
    (idx / 2) as u64
}

fn main() {
    let lines: Vec<String> = read_lines("./day09/input.txt").unwrap().flatten().collect();
    let mut input = lines[0]
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
        .into_boxed_slice();
    println!("input length: {}", input.len());

    let mut solution: u64 = 0;

    let mut lptr: usize = 0;
    let mut rptr: usize = input.len() - 1;
    let mut memory_pos: u64 = 0;

    while lptr <= rptr {
        let length = input[lptr];
        if is_file(lptr) {
            if length == 0 {
                lptr += 1;
            } else {
                solution += memory_pos * file_index(lptr);
                memory_pos += 1;
                input[lptr] -= 1;
            }
        } else {
            if length == 0 {
                lptr += 1;
            } else {
                if input[rptr] == 0 {
                    rptr -= 2;
                } else {
                    solution += memory_pos * file_index(rptr);
                    memory_pos += 1;
                    input[lptr] -= 1;
                    input[rptr] -= 1;
                }
            }
        }
    }

    println!("day  9");
    println!("  - part 1: {}", solution); // 6356833654075
    // println!("  - part 2: {}", solution2);
}
