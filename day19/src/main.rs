use std::vec;

use regex::Regex;
use utils::inputs::read_lines;

fn main() {
    let lines = read_lines("./day19/test.txt").unwrap();
    let mut scanners: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut scanner_distances: Vec<Vec<Vec<u64>>> = Vec::new();
    let coordinate_regex = Regex::new(r"^(-?\d+),(-?\d+),(-?\d+)$").unwrap();

    fn calculate_distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> u64 {
        p1.0.abs_diff(p2.0).pow(2) + p1.1.abs_diff(p2.1).pow(2) + p1.2.abs_diff(p2.2).pow(2)
    }

    fn count_matches(vec1: &Vec<u64>, vec2: &Vec<u64>) -> u64 {
        let mut ptr1: usize = 0;
        let mut ptr2: usize = 0;
        let mut matches = 0;

        while ptr1 < vec1.len() && ptr2 < vec2.len() {
            if vec1[ptr1] == vec2[ptr2] {
                matches += 1;
                ptr1 += 1;
                ptr2 += 1;
            } else if vec1[ptr1] < vec2[ptr2] {
                ptr1 += 1;
            } else {
                ptr2 += 1;
            }
        }

        matches
    }

    fn find_common_nodes(
        scanner1: &Vec<Vec<u64>>,
        scanner2: &Vec<Vec<u64>>,
    ) -> (u64, Vec<(usize, usize)>) {
        let mut max_matches = 0;
        let mut max_match_indices: Vec<(usize, usize)> = vec![];
        for i in 0..scanner1.len() {
            for j in 0..scanner2.len() {
                let matches = count_matches(&scanner1[i], &scanner2[j]);
                if matches > max_matches {
                    max_matches = matches;
                    max_match_indices = vec![(i, j)];
                } else if matches == max_matches {
                    max_match_indices.push((i, j));
                }
            }
        }
        return (max_matches, max_match_indices);
    }

    for line in lines.flatten() {
        if line.starts_with("---") {
            // create new scanner
            scanners.push(Vec::new());
            scanner_distances.push(Vec::new());
        } else if line.len() == 0 {
            let curr_scanner_distances = scanner_distances.last_mut().unwrap();
            for distances in curr_scanner_distances {
                distances.sort();
            }
        } else {
            let (_, [x, y, z]) = coordinate_regex.captures(&line).unwrap().extract();
            // parse the coordinates
            let coordinates = (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
            // initialize distance vector for new coordinate
            let curr_scanner = scanners.last_mut().unwrap();
            let curr_scanner_distances = scanner_distances.last_mut().unwrap();
            let mut curr_distances = Vec::new();
            // calculate distance to all other coordinates and add to both distance vectors

            for i in 0..curr_scanner.len() {
                let distance = calculate_distance(curr_scanner[i], coordinates);
                curr_distances.push(distance);
                curr_scanner_distances[i].push(distance);
            }
            // add new coordinate to scanner
            curr_scanner.push(coordinates);
            curr_scanner_distances.push(curr_distances);
        }
    }

    let curr_scanner_distances = scanner_distances.last_mut().unwrap();
    for distances in curr_scanner_distances {
        distances.sort();
    }

    let scanner1: &Vec<Vec<u64>> = &scanner_distances[0];
    let scanner2: &Vec<Vec<u64>> = &scanner_distances[1];

    let (max_matches, max_match_indices) = find_common_nodes(scanner1, scanner2);

    println!("max_matches: {}", max_matches);
    let mut matches_coordinates: Vec<((i64, i64, i64), (i64, i64, i64))> = max_match_indices
        .iter()
        .map(|(i, j)| (scanners[0][*i], scanners[1][*j]))
        .collect();
    matches_coordinates.sort_by(|a, b| a.cmp(b));
    // max_match_indices.sort();
    // for matched in &matches_coordinates {
    //     println!("{:?} {:?}", matched.0, matched.1);
    // }

    let ((a0, b0, c0), (x0, y0, z0)) = matches_coordinates[0];
    let mut level1: Vec<((i64, i64, i64), (i64, i64, i64))> = matches_coordinates[0..4].to_vec();
    for i in 1..4 {
        let ((a1, b1, c1), (x1, y1, z1)) = level1[i];
        level1[i] = (
            (a0 - a1, b0 - b1, c0 - c1),
            (x0 - x1, y0 - y1, z0 - z1),
        );
    }
    let ((a1, b1, c1), (x1, y1, z1)) = level1[1];
    let mut level2: Vec<((i64, i64, i64), (i64, i64, i64))> = level1.to_vec();
    for i in 2..4 {
        let ((a2, b2, c2), (x2, y2, z2)) = level2[i];
        level2[i] = (
            (a2*a1 - a1*a2, a2*b1 - a1*b2, a2*c1 - a1*c2),
            (a2*x1 - a1*x2, a2*y1 - a1*y2, a2*z1 - a1*z2),
        );
    }
    let ((a2, b2, c2), (x2, y2, z2)) = level2[2];
    let mut level3: Vec<((i64, i64, i64), (i64, i64, i64))> = level2.to_vec();
    for i in 3..4 {
        let ((a3, b3, c3), (x3, y3, z3)) = level3[i];
        level3[i] = (
            (b3*a2 - b2*a3, b3*b2 - b2*b3, b3*c2 - b2*c3),
            (b3*x2 - b2*x3, b3*y2 - b2*y3, b3*z2 - b2*z3),
        );
    }
    // for row in &matches_coordinates {
    //     println!("{:?}", row);
    // }
    // println!();
    // for row in level1 {
    //     println!("{:?}", row);
    // }
    // println!();
    // for row in level2 {
    //     println!("{:?}", row);
    // }
    // println!();
    for row in &level3 {
        println!("{:?}", row);
    }
    let z3 = &level3[3].0.2;
    let z = (level3[3].1.0 / z3, level3[3].1.1 / z3, level3[3].1.2 / z3);
    println!("z: {:?}", z);
}
