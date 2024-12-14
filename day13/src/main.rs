use regex::Regex;

fn solve_equation(xa: &i64, ya: &i64, xb: &i64, yb: &i64, xp: &i64, yp: &i64) -> (i64, i64) {
    // let b = (ya * xp - xa * yp) / (ya * xb - xa * yb);
    // let a = (xp - xb * b) / xa;

    let b_nom = ya * xp - xa * yp;
    let b_denom = ya * xb - xa * yb;
    if b_nom % b_denom != 0 {
        // if not divisible, skip
        return (i64::MIN, i64::MIN);
    }
    let b = b_nom / b_denom;

    let a_nom = xp - xb * b;
    if a_nom % xa != 0 {
        // if not divisible, skip
        return (i64::MIN, i64::MIN);
    }
    let a = a_nom / xa;

    return (a, b);
}

fn solve1(prize_machines: &Vec<(i64, i64, i64, i64, i64, i64)>) -> i64 {
    let mut solution: i64 = 0;

    for (xa, ya, xb, yb, xp, yp) in prize_machines {
        let (a, b) = solve_equation(xa, ya, xb, yb, xp, yp);
        if a > 0 && b > 0 {
            solution += 3 * a + 1 * b;
        }
    }

    return solution;
}

fn solve2(prize_machines: &Vec<(i64, i64, i64, i64, i64, i64)>) -> i64 {
    let mut solution: i64 = 0;

    for (xa, ya, xb, yb, xp, yp) in prize_machines {
        let xp = 10000000000000 + xp;
        let yp = 10000000000000 + yp;
        let (a, b) = solve_equation(xa, ya, xb, yb, &xp, &yp);
        if a > 0 && b > 0 {
            solution += 3 * a + 1 * b;
        }
    }

    return solution;
}

fn main() {
    let input = include_str!("../input.txt");
    let regex_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_p = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let mut prize_machines: Vec<(i64, i64, i64, i64, i64, i64)> = Vec::new();

    let mut line_idx: usize = 0;
    while line_idx < lines.len() {
        let line = lines[line_idx].trim();
        let (_, [str_xa, str_ya]) = regex_a.captures(line).unwrap().extract();
        let xa: i64 = str_xa.parse().unwrap();
        let ya: i64 = str_ya.parse().unwrap();
        line_idx += 1;

        let line = lines[line_idx].trim();
        let (_, [str_xb, str_yb]) = regex_b.captures(line).unwrap().extract();
        let xb: i64 = str_xb.parse().unwrap();
        let yb: i64 = str_yb.parse().unwrap();
        line_idx += 1;

        let line = lines[line_idx].trim();
        let (_, [str_xp, str_yp]) = regex_p.captures(line).unwrap().extract();
        let xp: i64 = str_xp.parse().unwrap();
        let yp: i64 = str_yp.parse().unwrap();
        line_idx += 2;

        prize_machines.push((xa, ya, xb, yb, xp, yp));
    }

    println!("day 13");
    println!("  - part 1: {}", solve1(&prize_machines)); // 35255
    println!("  - part 2: {}", solve2(&prize_machines)); // 87582154060429
}
