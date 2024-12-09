use regex::Regex;
use utils::{conversion::to_int, inputs::read_lines};

pub fn main() {
    let lines = read_lines("./day03/input.txt").unwrap().flatten();

    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let mut result: u32 = 0;
    let re2 = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
    let mut result2: u32 = 0;
    let mut multiplicator: u32 = 1;
    for line in lines {
        for (_, [instruction]) in re2.captures_iter(&line).map(|c| c.extract()) {
            match instruction {
                "do()" => multiplicator = 1,
                "don't()" => multiplicator = 0,
                _ => {
                    let (_, [m1, m2]) = re.captures(instruction).unwrap().extract();
                    let multi = to_int(m1) * to_int(m2);
                    result += multi;
                    result2 += multi * multiplicator;
                }
            };
        }
    }

    println!("day  3");
    println!("  - part 1: {}", result); // 164730528
    println!("  - part 2: {}", result2); // 70478672
}
