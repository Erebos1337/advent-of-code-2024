use utils::inputs::read_lines;

fn solve(curr_value: u64, goal: u64, operands: &[u64], index: usize) -> bool {
    if index == operands.len() {
        return curr_value == goal;
    }
    if curr_value > goal {
        return false;
    }
    return solve(curr_value + operands[index], goal, operands, index + 1)
        || solve(curr_value * operands[index], goal, operands, index + 1);
}

fn concat_nums(num1: u64, num2: u64) -> u64 {
    let mut num = num1;
    let num2_len = num2.to_string().len();
    for _ in 0..num2_len {
        num *= 10;
    }
    return num + num2;
}

fn solve2(curr_value: u64, goal: u64, operands: &[u64], index: usize) -> bool {
    if index == operands.len() {
        return curr_value == goal;
    }
    if curr_value > goal {
        return false;
    }
    return solve2(curr_value + operands[index], goal, operands, index + 1)
        || solve2(curr_value * operands[index], goal, operands, index + 1)
        || solve2(
            concat_nums(curr_value, operands[index]),
            goal,
            operands,
            index + 1,
        );
}

fn main() {
    let lines: Vec<String> = read_lines("./day07/input.txt").unwrap().flatten().collect();

    let mut solution: u64 = 0;
    let mut solution2: u64 = 0;
    for line in lines {
        let (result_string, operands_string) = line.split_once(": ").unwrap();
        let result: u64 = result_string.parse().unwrap();
        let operands: Box<[u64]> = operands_string
            .split_whitespace()
            .flat_map(|operand_string| operand_string.parse::<u64>())
            .collect::<Vec<u64>>()
            .into_boxed_slice();
        if solve(0, result, &operands, 0) {
            solution += result;
        }
        if solve2(0, result, &operands, 0) {
            solution2 += result;
        }
    }

    println!("day  7");
    println!("  - part 1: {}", solution);  //   42283209483350
    println!("  - part 2: {}", solution2); // 1026766857276279
}
