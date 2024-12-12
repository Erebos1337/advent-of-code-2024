use std::{u16, vec};

use utils::inputs::read_lines;

fn is_file(idx: usize) -> bool {
    idx % 2 == 0
}

fn file_index(idx: usize) -> u64 {
    (idx / 2) as u64
}

fn solve1(mut input: Box<[u32]>) -> u64 {
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

    return solution;
}

fn solve2(input: Box<[u32]>) -> u64 {
    let mut solution: u64 = 0;

    let last_file_index = input.len() / 2;

    let mut memory_length: usize = 0;
    for i in 0..input.len() {
        memory_length += input[i] as usize;
    }
    let mut memory: Box<[u16]> = vec![u16::MAX; memory_length].into_boxed_slice();
    let mut files_reverse: Box<[(u16, usize, u32)]> =
        vec![(0, 0, 0); last_file_index + 1].into_boxed_slice();

    {
        let mut mem_ptr: usize = 0;
        let mut file_idx: u16 = 0;
        for i in 0..input.len() {
            let length = input[i];
            if is_file(i) {
                files_reverse[last_file_index - file_idx as usize] = (file_idx, mem_ptr, length);
                for _ in 0..length {
                    memory[mem_ptr] = file_idx;
                    mem_ptr += 1;
                }
                file_idx += 1;
            } else {
                mem_ptr += length as usize;
            }
        }
    }

    for (file_idx, mem_pos, file_length) in files_reverse {
        let mut mem_ptr: usize = 0;
        'try_move_file: while mem_ptr + (file_length as usize) <= mem_pos {
            if memory[mem_ptr] == u16::MAX {
                let mut space_length = 0;
                let mut space_ptr = mem_ptr;
                // find out space length
                while memory[space_ptr] == u16::MAX {
                    space_length += 1;
                    space_ptr += 1;
                }
                if space_length >= file_length {
                    // found space for file
                    for i in 0..file_length as usize {
                        // insert file into empty memory
                        memory[mem_ptr + i] = file_idx;
                        // delete file from original position
                        memory[mem_pos + i] = u16::MAX;
                    }
                    break 'try_move_file;
                } else {
                    // if file does not fit, continue finding next space
                    mem_ptr += space_length as usize;
                }
            } else {
                // if memory occupied, skip to next memory slot
                mem_ptr += 1;
            }
        }
    }

    for i in 0..memory.len() {
        if memory[i] != u16::MAX {
            solution += memory[i] as u64 * i as u64;
        }
    }

    return solution;
}

fn main() {
    let lines: Vec<String> = read_lines("./day09/input.txt").unwrap().flatten().collect();
    let input = lines[0]
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
        .into_boxed_slice();
    let input2 = input.clone();

    let solution1: u64 = solve1(input);
    let solution2: u64 = solve2(input2);

    println!("day  9");
    println!("  - part 1: {}", solution1); // 6356833654075
    println!("  - part 2: {}", solution2); // 6389911791746
}
