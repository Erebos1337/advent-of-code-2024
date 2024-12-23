fn read_secrets(input: &str) -> Box<[i64]> {
    input.lines().map(|v| v.trim().parse().unwrap()).collect()
}

fn calc_next_secret(secret: i64) -> i64 {
    const PRUNE: i64 = (1 << 24) - 1;
    let mut next_secret = ((secret << 6) ^ secret) & PRUNE;
    next_secret = ((next_secret >> 5) ^ next_secret) & PRUNE;
    next_secret = ((next_secret << 11) ^ next_secret) & PRUNE;
    next_secret
}

fn calc_nth_secret(secret: i64, n: usize) -> i64 {
    let next_secret = calc_next_secret(secret);
    if n == 1 {
        next_secret
    } else {
        calc_nth_secret(next_secret, n - 1)
    }
}

fn calc_next_diff_hash(index: usize, diff: i64) -> usize {
    assert!(diff >= -9 && diff <= 9);
    (index >> 5) | ((diff + 9 & 0b11111) as usize) << 15
}

fn calc_max_index() -> usize {
    let mut index: usize = 0;
    for _ in 0..4 {
        index = calc_next_diff_hash(index, 9);
    }
    index
}

fn solve1(secrets: &[i64]) -> i64 {
    let mut sum = 0;
    for secret in secrets {
        sum += calc_nth_secret(*secret, 2000);
    }
    sum
}

fn solve2(secrets: &[i64]) -> u16 {
    let mut sequence_sums = vec![0u16; calc_max_index() + 1].into_boxed_slice();
    let mut found = vec![0u16; calc_max_index() + 1].into_boxed_slice();
    for (idx_secret, secret) in secrets.iter().enumerate() {
        let mut diff_hash = 0;
        let mut curr_secret = *secret;
        let mut curr_bananas = curr_secret % 10;

        for i in 0..2000 {
            let next_secret = calc_next_secret(curr_secret);
            let next_bananas = next_secret % 10;
            let diff = next_bananas - curr_bananas;
            diff_hash = calc_next_diff_hash(diff_hash, diff);

            if i > 2 && found[diff_hash] <= idx_secret as u16 {
                found[diff_hash] = (idx_secret + 1) as u16;
                sequence_sums[diff_hash] += next_bananas as u16;
            }

            curr_secret = next_secret;
            curr_bananas = next_bananas;
        }
    }

    let mut max_sum = 0;
    for sum in sequence_sums {
        if sum > max_sum {
            max_sum = sum;
        }
    }
    max_sum
}

fn main() {
    let input = include_str!("../input.txt");
    let secrets = read_secrets(input);

    let solution1 = solve1(&secrets);
    let solution2 = solve2(&secrets);

    println!("day 22");
    println!("  - part 1: {}", solution1); // 15335183969
    println!("  - part 2: {}", solution2); // 1696
}
