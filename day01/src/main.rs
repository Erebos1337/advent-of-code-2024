use utils::{conversion::to_int, inputs::read_lines};


pub fn main() {
    let mut delta: u32 = 0;
    let mut locations1: Vec<u32> = Vec::new();
    let mut locations2: Vec<u32> = Vec::new();

    let lines = read_lines("./day01/input.txt").unwrap();

    for line in lines.flatten() {
        let pair = line.split_once("   ").unwrap();
        locations1.push(to_int(pair.0));
        locations2.push(to_int(pair.1));
    }

    locations1.sort();
    locations2.sort();

    for i in 0..locations1.len() {
        if locations1[i] > locations2[i] {
            delta += locations1[i] - locations2[i];
        } else {
            delta += locations2[i] - locations1[i];
        }
    }

    let mut similarity: u32 = 0;
    let mut ptr1: usize = 0;
    let mut ptr2: usize = 0;

    let mut curr_val: u32;
    let mut curr_count1: u32;
    let mut curr_count2: u32;

    while ptr1 < locations1.len() && ptr2 < locations2.len() {
        if locations1[ptr1] == locations2[ptr2] {
            // initialize new similarity check
            curr_val = locations1[ptr1];
            curr_count1 = 1;
            curr_count2 = 1;

            // count number of times the current value appears in the first list
            ptr1 += 1;
            while ptr1 < locations1.len() && locations1[ptr1] == curr_val {
                curr_count1 += 1;
                ptr1 += 1;
            }

            // count number of times the current value appears in the second list
            ptr2 += 1;
            while ptr2 < locations2.len() && locations2[ptr2] == curr_val {
                curr_count2 += 1;
                ptr2 += 1;
            }

            similarity += curr_val * curr_count1 * curr_count2;
        } else if locations1[ptr1] < locations2[ptr2] {
            ptr1 += 1;
        } else {
            ptr2 += 1;
        }
    }

    println!("day  1");
    println!("  - part 1: {}", delta); // 1319616
    println!("  - part 2: {}", similarity); // 27267728
}
