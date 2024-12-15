use regex::Regex;

static PRINT_EASTER_EGG: bool = false;

fn solve1(robots: &Vec<(i64, i64, i64, i64)>, width: i64, height: i64) -> u32 {
    let mid_x = width / 2;
    let mid_y = height / 2;
    let time = 100;
    let mut quadrant1: u32 = 0;
    let mut quadrant2: u32 = 0;
    let mut quadrant3: u32 = 0;
    let mut quadrant4: u32 = 0;
    for (x, y, dx, dy) in robots {
        let final_x = (x + dx * time).rem_euclid(width);
        let final_y = (y + dy * time).rem_euclid(height);
        if final_x < mid_x && final_y < mid_y {
            quadrant1 += 1;
        } else if final_x > mid_x && final_y < mid_y {
            quadrant2 += 1;
        } else if final_x < mid_x && final_y > mid_y {
            quadrant3 += 1;
        } else if final_x > mid_x && final_y > mid_y {
            quadrant4 += 1;
        }
    }
    let solution = quadrant1 * quadrant2 * quadrant3 * quadrant4;
    return solution;
}

fn print_grid(width: i64, height: i64, time: i64, robots: &Vec<(i64, i64, i64, i64)>) {
    let mut grid =
        vec![vec![' '; width as usize].into_boxed_slice(); height as usize].into_boxed_slice();

    for (x, y, dx, dy) in robots {
        let final_x = (x + dx * time).rem_euclid(width);
        let final_y = (y + dy * time).rem_euclid(height);
        grid[final_y as usize][final_x as usize] = '#';
    }

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}

fn solve2(robots: &Vec<(i64, i64, i64, i64)>, width: i64, height: i64) -> u32 {
    let mut grid: Box<[Box<[i64]>]> =
        vec![vec![-1; width as usize].into_boxed_slice(); height as usize].into_boxed_slice();
    let mut easter_egg_time = -1;
    'timeloop: for time in 0..1000000 {
        for (x, y, dx, dy) in robots {
            let final_x = (x + dx * time).rem_euclid(width);
            let final_y = (y + dy * time).rem_euclid(height);
            if grid[final_y as usize][final_x as usize] == time {
                continue 'timeloop;
            }
            grid[final_y as usize][final_x as usize] = time;
        }
        easter_egg_time = time;

        break;
    }
    if PRINT_EASTER_EGG {
        print_grid(width, height, easter_egg_time, robots);
    }
    return easter_egg_time as u32;
}

fn main() {
    let input = include_str!("../input.txt");
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let lines: Vec<&str> = input.lines().map(|s| s.trim()).collect();
    let mut robots: Vec<(i64, i64, i64, i64)> = Vec::new();

    for line in lines {
        let (_, [str_x, str_y, str_dx, str_dy]) = regex.captures(line).unwrap().extract();
        let x: i64 = str_x.parse().unwrap();
        let y: i64 = str_y.parse().unwrap();
        let dx: i64 = str_dx.parse().unwrap();
        let dy: i64 = str_dy.parse().unwrap();
        robots.push((x, y, dx, dy));
    }

    let width = 101;
    let height = 103;

    let solution1 = solve1(&robots, width, height);
    let solution2 = solve2(&robots, width, height);

    println!("day 14");
    println!("  - part 1: {}", solution1); // 208437768
    println!("  - part 2: {}", solution2); // 7492
}
