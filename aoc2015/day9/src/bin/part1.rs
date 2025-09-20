use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = get_map(&input);
    let distances = get_distances(&input, &map);

    let shortest = shortest_path(&distances);
    println!("{shortest}");
}

fn get_map(input: &str) -> HashMap<&str, usize> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut it = line.split_ascii_whitespace();
        let location1 = it.next().unwrap();
        it.next();
        let location2 = it.next().unwrap();

        if !map.contains_key(&location1) {
            map.insert(location1, map.len());
        }
        if !map.contains_key(&location2) {
            map.insert(location2, map.len());
        }
    }

    map
}

fn get_distances(input: &str, map: &HashMap<&str, usize>) -> Vec<Vec<u16>> {
    let len = map.len();
    let mut distances = vec![vec![0u16; len]; len];

    for line in input.lines() {
        let mut it = line.split_ascii_whitespace();
        let from = it.next().unwrap();
        it.next();
        let to = it.next().unwrap();
        it.next();
        let dist: u16 = it.next().unwrap().parse().unwrap();

        let from_ind = map[&from];
        let to_ind = map[&to];

        distances[from_ind][to_ind] = dist;
        distances[to_ind][from_ind] = dist;
    }

    distances
}

// trying all possible n! paths
fn shortest_path(distances: &Vec<Vec<u16>>) -> u16 {
    let mut used = vec![false; distances.len()];
    let mut record = u16::MAX;

    for i in 0..used.len() {
        used[i] = true;
        shortest_path_inner(distances, i, &mut used, 0, &mut record);
        used[i] = false;
    }
    record
}

fn shortest_path_inner(
    distances: &Vec<Vec<u16>>,
    last_visited: usize,
    used: &mut Vec<bool>,
    current: u16,
    record: &mut u16,
) {
    if used.iter().all(|&i| i) {
        *record = current.min(*record);
        return;
    }

    for i in 0..used.len() {
        if !used[i] {
            used[i] = true;
            shortest_path_inner(
                distances,
                i,
                used,
                current + distances[last_visited][i],
                record,
            );
            used[i] = false;
        }
    }
}
