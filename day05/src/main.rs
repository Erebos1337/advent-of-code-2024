use utils::{conversion::to_int, inputs::read_lines};
use std::collections::HashMap;

fn check_correct_order(pages: &Vec<u32>, dependencies_map: &HashMap<u32, Vec<u32>>) -> bool {
    let mut is_valid_update = true;
    for i in 0..pages.len() - 1 {
        for j in i + 1..pages.len() {
            if dependencies_map.contains_key(&pages[j])
            && dependencies_map.get(&pages[j]).unwrap().contains(&pages[i]) {
                is_valid_update = false;
                break;
            }
        }
        if !is_valid_update {
            break;
        }
    }
    is_valid_update
}

fn swap_entries (pages: &mut Vec<u32>, i: usize, j: usize) {
    let temp = pages[i];
    pages[i] = pages[j];
    pages[j] = temp;
}

fn fix_pages(pages: &mut Vec<u32>, dependencies_map: &HashMap<u32, Vec<u32>>) {

    while !check_correct_order(pages, dependencies_map) {
        for i in 0..pages.len() - 1 {
            let first_page = pages[i];
            for j in i + 1..pages.len() {
                let second_page = pages[j];
                if dependencies_map.contains_key(&second_page)
                && dependencies_map.get(&second_page).unwrap().contains(&first_page) {
                    swap_entries(pages, i, j);
                }
            }
        }
    }
}

fn main() {
    let lines = read_lines("./day05/input.txt").unwrap();

    let mut read_dependencies = true;
    let mut dependencies_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut result: u32 = 0; // 143
    let mut result2: u32 = 0; // 123

    for line in lines.flatten() {
        if line.len() == 0 {
            read_dependencies = false;
            continue;
        }

        if read_dependencies {
            let pair_string = line.split_once("|").unwrap();
            let pair = (to_int(pair_string.0), to_int(pair_string.1));
            if !dependencies_map.contains_key(&pair.0) {
                dependencies_map.insert(pair.0, vec![pair.1]);
            } else {
                let dependencies = dependencies_map.get_mut(&pair.0).unwrap();
                dependencies.push(pair.1);
            }
        }

        if !read_dependencies {
            let mut page_numbers = line.split_terminator(",").map(|x| to_int(x)).collect::<Vec<u32>>();
            
            if check_correct_order(&page_numbers, &dependencies_map) {
                result += &page_numbers[page_numbers.len() / 2];
            } else {
                fix_pages(&mut page_numbers, &dependencies_map);
                result2 += page_numbers[page_numbers.len() / 2];
            }
        }
    }

    println!("day  5");
    println!("  - part 1: {}", result); // 4569
    println!("  - part 2: {}", result2); // 6456
}
