use std::collections::HashMap;

fn read_connections(input: &str) -> Box<[(&str, &str)]> {
    input
        .lines()
        .map(|line| line.trim().split_once('-').unwrap())
        .collect()
}

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

fn solve2(connections: &[(&str, &str)]) -> String {
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

    fn find_clique<'a>(
        adjacency_map: &'a HashMap<&'a str, Box<[&'a str]>>,
        start: &'a str,
        members: Vec<&'a str>,
        offset: usize,
    ) -> Vec<&'a str> {
        let adjacents = &adjacency_map[start];
        if offset == adjacents.len() {
            return Vec::new();
        }

        let mut longest_clique_length = members.len();
        let mut longest_clique: Vec<&str> = Vec::new();

        'outer: for i in offset..adjacents.len() {
            let candidate = adjacents[i];
            if candidate <= start || (members.len() > 0 && candidate <= members.last().unwrap()) {
                continue;
            }
            for member in &members {
                if adjacency_map[member].binary_search(&candidate).is_err() {
                    continue 'outer;
                }
            }
            let mut new_members = members.clone();
            new_members.push(candidate);

            let longest_sub_clique = find_clique(adjacency_map, start, new_members.clone(), i + 1);

            let mut new_clique = [new_members.clone(), longest_sub_clique].concat();
            new_clique.sort();
            new_clique.dedup();
            if new_clique.len() > longest_clique_length {
                longest_clique_length = new_clique.len();
                longest_clique = new_clique;
            }
        }

        longest_clique
    }

    let mut longest_clique_length = 0;
    let mut longest_clique: Vec<&str> = Vec::new();
    let mut longest_clique_start: &str = "";
    for (start, adjacents) in &adjacency_map {
        let clique = find_clique(
            &adjacency_map,
            start,
            vec![],
            adjacents.binary_search(start).unwrap_err(),
        );
        if clique.len() > longest_clique_length {
            longest_clique_length = clique.len();
            longest_clique = clique;
            longest_clique_start = start;
        }
    }

    let solution = format!("{},{}", longest_clique_start, longest_clique.join(","));

    solution
}

fn main() {
    let input = include_str!("../input.txt");
    let connections = read_connections(input);

    let solution1 = solve1(&connections);
    let solution2 = solve2(&connections);

    println!("day 23");
    println!("  - part 1: {}", solution1); // 1269
    println!("  - part 2: {}", solution2); // ad,jw,kt,kz,mt,nc,nr,sb,so,tg,vs,wh,yh
}
