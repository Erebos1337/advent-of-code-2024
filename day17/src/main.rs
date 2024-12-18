use std::{
    collections::HashSet,
    str::Lines,
    time::{Duration, Instant},
    u64,
};

use regex::Regex;

fn read_registers(lines: &mut Lines) -> [u64; 3] {
    let regex = Regex::new(r"Register [ABC]: (\d+)").unwrap();
    let (_, [register_a]) = regex.captures(lines.next().unwrap()).unwrap().extract();
    let (_, [register_b]) = regex.captures(lines.next().unwrap()).unwrap().extract();
    let (_, [register_c]) = regex.captures(lines.next().unwrap()).unwrap().extract();
    lines.next(); // empty line
    return [
        register_a.parse().unwrap(),
        register_b.parse().unwrap(),
        register_c.parse().unwrap(),
    ];
}

fn read_instructions(lines: &mut Lines) -> Box<[u8]> {
    lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn combo(&registers: &[u64; 3], operand: u8) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => 0,
    }
}

fn run_program(registers: &[u64; 3], instructions: &Box<[u8]>) -> Box<[u8]> {
    // Combo operands 0 through 3 represent literal values 0 through 3.
    // Combo operand 4 represents the value of register A.
    // Combo operand 5 represents the value of register B.
    // Combo operand 6 represents the value of register C.
    // Combo operand 7 is reserved and will not appear in valid programs.
    let mut registers = *registers;
    let mut output: Vec<u8> = Vec::new();
    let mut ptr = 0usize;
    while ptr < instructions.len() {
        match instructions[ptr] {
            // The adv instruction (opcode 0) performs division.
            // The numerator is the value in the A register.
            // The denominator is found by raising 2 to the power of the instruction's combo operand.
            // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
            // The result of the division operation is truncated to an integer and then written to the A register.
            0 => {
                registers[0] = registers[0] >> combo(&&registers, instructions[ptr + 1]);
                ptr += 2;
            }
            // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
            1 => {
                registers[1] ^= instructions[ptr + 1] as u64;
                ptr += 2;
            }
            // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
            2 => {
                registers[1] = combo(&&registers, instructions[ptr + 1]) % 8;
                ptr += 2;
            }
            // The jnz instruction (opcode 3) does nothing if the A register is 0.
            // However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
            // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
            3 => {
                if registers[0] != 0 {
                    ptr = instructions[ptr + 1] as usize;
                } else {
                    ptr += 2;
                }
            }
            // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B.
            // (For legacy reasons, this instruction reads an operand but ignores it.)
            4 => {
                registers[1] ^= registers[2];
                ptr += 2;
            }
            // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
            // (If a program outputs multiple values, they are separated by commas.)
            5 => {
                output.push((combo(&&registers, instructions[ptr + 1]) % 8) as u8);
                ptr += 2;
            }
            // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
            // (The numerator is still read from the A register.)
            6 => {
                registers[1] = registers[0] >> combo(&&registers, instructions[ptr + 1]);
                ptr += 2;
            }
            // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
            // (The numerator is still read from the A register.)
            7 => {
                registers[2] = registers[0] >> combo(&&registers, instructions[ptr + 1]);
                ptr += 2;
            }
            _ => {
                ptr += 2;
            }
        }
    }
    output.into_boxed_slice()
}

fn solve1(registers: &[u64; 3], instructions: &Box<[u8]>) -> (String, Duration) {
    let start = Instant::now();

    let output = run_program(registers, &instructions);

    let solution: String = output
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
    (solution, start.elapsed())
}

fn solve2(registers: &[u64; 3], instructions: &Box<[u8]>) -> (u64, Duration) {
    let start = Instant::now();
    let mut suffixes: HashSet<u64> = HashSet::from([0]);
    for i in 0..16 {
        let shift = i * 3;
        let match_length = i + 1;
        let mut new_suffixes: HashSet<u64> = HashSet::new();
        for suffix in &suffixes {
            for j in 0..1 << 18 {
                let new_num = suffix + (j << shift);
                let mut new_registers = *registers;
                new_registers[0] = new_num;
                let output = run_program(&new_registers, instructions);
                if output.len() >= match_length
                    && output.len() <= instructions.len()
                    && output[0..match_length] == instructions[0..match_length]
                {
                    new_suffixes.insert(new_num & ((1 << match_length * 3) - 1));
                }
            }
        }
        suffixes = new_suffixes;
    }
    let mut min = u64::MAX;
    for suffix in suffixes {
        if suffix < min {
            min = suffix;
        }
    }

    (min, start.elapsed())
}

fn main() {
    let mut input = include_str!("../input.txt").lines();

    let registers: [u64; 3] = read_registers(&mut input);
    let instructions: Box<[u8]> = read_instructions(&mut input);

    let (solution1, _) = solve1(&registers, &instructions);
    let (solution2, _) = solve2(&registers, &instructions);

    println!("day 17");
    println!("  - part 1: {}", solution1); // 2,7,4,7,2,1,7,5,1
    println!("  - part 2: {}", solution2); // 37221274271220
}
