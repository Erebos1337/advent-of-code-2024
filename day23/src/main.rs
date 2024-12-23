use std::collections::HashMap;

use timed::timed;

fn read_connections(input: &str) -> Box<[(&str, &str)]> {
    input
        .lines()
        .map(|line| line.trim().split_once('-').unwrap())
        .collect()
}

#[timed]
fn solve1(connections: &[(&str, &str)]) -> usize {
    let adjacency_map: HashMap<&str, Box<[&str]>>;
    {
        let mut adjacents_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for (a, b) in connections {
            adjacents_map.entry(a).or_default().push(b);
            adjacents_map.entry(b).or_default().push(a);
        }
        adjacency_map = adjacents_map
            .into_iter()
            .map(|(k, v)| {
                let mut sorted_box = v.into_boxed_slice();
                sorted_box.sort();
                (k, sorted_box)
            })
            .collect();
    }

    let mut num_triangles = 0;
    for (start, adjacents) in &adjacency_map {
        for i in adjacents.binary_search(start).unwrap_err()..adjacents.len() {
            let mid = adjacents[i];
            if mid <= start {
                continue;
            }
            for j in i + 1..adjacents.len() {
                let end = adjacents[j];
                if adjacency_map[mid].binary_search(&end).is_ok() {
                    if start.starts_with('t') || mid.starts_with('t') || end.starts_with('t') {
                        num_triangles += 1;
                    }
                }
            }
        }
    }

    num_triangles
}

fn main() {
    let input = include_str!("../input.txt");
    let connections = read_connections(input);

    let solution1 = solve1(&connections);

    println!("day 22");
    println!("  - part 1: {}", solution1); // 1269
}
