use std::collections::HashMap;

fn read_towels(input: &str) -> Box<[&str]> {
    input.lines().next().unwrap().trim().split(", ").collect()
}

fn read_designs(input: &str) -> Box<[&str]> {
    input.lines().skip(2).map(|line| line.trim()).collect()
}

fn solve1<'a>(towels: &[&str], designs: &[&str]) -> u32 {
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

    let mut count = 0;
    let mut memo: HashMap<&str, bool> = HashMap::new();
    
    for design in designs {
        if dfs(towels, design, &mut memo) {
            count += 1;
        }
    }

    count
}

fn solve2<'a>(towels: &[&str], designs: &[&str]) -> u64 {
    fn dfs_count<'a>(towels: &[&str], design: &'a str, memo: &mut HashMap<&'a str, u64>) -> u64 {
        if design.is_empty() {
            return 1;
        }
    
        if memo.contains_key(design) {
            return memo[design];
        }
    
        let mut count = 0;
        for towel in towels {
            if towel.len() > design.len() {
                continue;
            }
    
            if design.starts_with(towel) {
                let rest_design = &design[towel.len()..];
                let result = dfs_count(towels, &rest_design, memo);
                count += result;
            }
        }
        memo.insert(design, count);
    
        count
    }

    let mut count = 0;
    let mut memo: HashMap<&str, u64> = HashMap::new();

    for design in designs {
        let result = dfs_count(towels, design, &mut memo);
        count += result;
    }

    count
}

fn main() {
    let input = include_str!("../input.txt");

    let towels = read_towels(&input);
    let designs = read_designs(&input);

    let solution1 = solve1(&towels, &designs);
    let solution2 = solve2(&towels, &designs);

    println!("day 19");
    println!("  - part 1: {}", solution1); // 336
    println!("  - part 2: {}", solution2); // 758890600222015
}
