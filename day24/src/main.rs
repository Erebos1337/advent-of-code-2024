use std::collections::HashMap;

use regex::Regex;
use timed::timed;

fn get_z_var(i: u8) -> String {
    format!("z{:02}", i)
}

fn read_wires(wires_string: &str) -> HashMap<String, u8> {
    wires_string
        .split_terminator("\n")
        .map(|line| line.split_once(": ").unwrap())
        .map(|(key, value)| (key.to_string(), value.parse().unwrap()))
        .collect()
}

fn read_operations(wires_string: &str) -> HashMap<String, (String, String, String)> {
    let operation_regex = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
    wires_string
        .split_terminator("\n")
        .map(|line| operation_regex.captures(line).unwrap().extract())
        .map(|(_, [operand1, operation, operand2, result])| {
            (
                result.to_string(),
                (
                    operation.to_string(),
                    operand1.to_string(),
                    operand2.to_string(),
                ),
            )
        })
        .collect()
}

#[timed]
fn solve1<'a>(
    wires: &HashMap<String, u8>,
    operations: &HashMap<String, (String, String, String)>,
) -> u64 {
    let mut solution = 0;

    let mut results: HashMap<String, u8> = wires.clone();

    fn calculate<'a>(
        key: String,
        results: &mut HashMap<String, u8>,
        operations: &HashMap<String, (String, String, String)>,
    ) -> u8 {
        if let Some(value) = results.get(&key) {
            return *value;
        }

        let (operation, operand1, operand2) = operations.get(&key).unwrap();
        let operand1 = calculate(operand1.clone(), results, operations);
        let operand2 = calculate(operand2.clone(), results, operations);

        let result = match operation.as_str() {
            "AND" => operand1 & operand2,
            "OR" => operand1 | operand2,
            "XOR" => operand1 ^ operand2,
            _ => panic!("Unknown operation: {}", operation),
        };

        results.insert(key, result);
        result
    }

    for i in 0..=45u8 {
        let variable = get_z_var(i);
        let value = calculate(variable, &mut results, operations);
        solution |= (value as u64) << i;
    }

    solution
}

#[timed]
fn solve2<'a>(operations: &HashMap<String, (String, String, String)>) -> &str {
    fn get_swapped_key(original: String) -> String {
        // dkr <--> z05
        // htp <--> z15
        // hhh <--> z20
        // rhv <--> ggk
        match original.as_str() {
            "dkr" => "z05".to_string(),
            "z05" => "dkr".to_string(),
            "htp" => "z15".to_string(),
            "z15" => "htp".to_string(),
            "hhh" => "z20".to_string(),
            "z20" => "hhh".to_string(),
            "rhv" => "ggk".to_string(),
            "ggk" => "rhv".to_string(),
            _ => original,
        }
    }

    fn calculate_key(
        key: String,
        x: u64,
        y: u64,
        operations: &HashMap<String, (String, String, String)>,
    ) -> u8 {
        if key.starts_with("x") {
            let i = key[1..].parse::<u8>().unwrap();
            return ((x >> i) & 1) as u8;
        } else if key.starts_with("y") {
            let i = key[1..].parse::<u8>().unwrap();
            return ((y >> i) & 1) as u8;
        }
        let (operation, operand1, operand2) = operations.get(&key).unwrap();
        let operand1 = calculate_key(get_swapped_key(operand1.clone()), x, y, operations);
        let operand2 = calculate_key(get_swapped_key(operand2.clone()), x, y, operations);

        let result = match operation.as_str() {
            "AND" => operand1 & operand2,
            "OR" => operand1 | operand2,
            "XOR" => operand1 ^ operand2,
            _ => panic!("Unknown operation: {}", operation),
        };

        result
    }

    // used to find next point of failure
    // found wires to swap manually from there
    for i in 0..=36u8 {
        if calculate_key(format!("z{:02}", i), 0 << i, 0 << i, operations) != 0 {
            println!("z{:02} wrong. Expected : 0, got 1.", i);
            println!("-----------------------");
        }
        if calculate_key(format!("z{:02}", i), 1 << i, 0 << i, operations) != 1 {
            println!("z{:02} wrong. Expected : 1, got 0.", i);
            println!("-----------------------");
        }
        if calculate_key(format!("z{:02}", i), 0 << i, 1 << i, operations) != 1 {
            println!("z{:02} wrong. Expected : 1, got 0.", i);
            println!("-----------------------");
        }
        if calculate_key(format!("z{:02}", i), 1 << i, 1 << i, operations) != 0 {
            println!("z{:02} wrong. Expected : 0, got 1.", i);
            println!("-----------------------");
        }
    }

    "dkr,ggk,hhh,htp,rhv,z05,z15,z20"
}

fn main() {
    let input = include_str!("../input.txt");

    let (wires_string, operations_string) = input.trim().split_once("\n\n").unwrap();
    let wires = read_wires(wires_string);
    let operations = read_operations(operations_string);

    let solution1 = solve1(&wires, &operations);
    let solution2 = solve2(&operations);

    println!("day 24:");
    println!("  - part 1: {}", solution1); // 53755311654662
    println!("  - part 2: {}", solution2); // dkr,ggk,hhh,htp,rhv,z05,z15,z20
}
