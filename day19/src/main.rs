use std::collections::HashMap;

fn read_towels(input: &str) -> Box<[&str]> {
    input.lines().next().unwrap().trim().split(", ").collect()
}

fn read_designs(input: &str) -> Box<[&str]> {
    input.lines().skip(2).map(|line| line.trim()).collect()
}

fn dfs<'a>(towels: &[&str], design: &'a str, memo: &mut HashMap<&'a str, bool>) -> bool {
    if design.is_empty() {
        return true;
    }

    if memo.contains_key(design) {
        return memo[design];
    }

    for towel in towels {
        if towel.len() > design.len() {
            continue;
        }

        if design.starts_with(towel) {
            let rest_design = &design[towel.len()..];
            let result = dfs(towels, &rest_design, memo);
            memo.insert(rest_design, result);

            if result {
                return true;
            }
        }
    }

    false
}

fn solve1<'a>(towels: &[&str], designs: &[&str]) -> u32 {
    let mut count = 0;

    let mut memo: HashMap<&str, bool> = HashMap::new();
    for design in designs {
        if dfs(towels, design, &mut memo) {
            count += 1;
        }
    }

    count
}

fn main() {
    let input = include_str!("../input.txt");

    let towels = read_towels(&input);
    let designs = read_designs(&input);

    let solution1: u32 = solve1(&towels, &designs);

    println!("day 19");
    println!("  - part 1: {}", solution1); // 336
}
