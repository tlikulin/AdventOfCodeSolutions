use std::{cmp::Reverse, collections::HashMap, str::FromStr};

const TOTAL_VOLUME: i16 = 150;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut containers: Vec<i16> = input.lines().flat_map(FromStr::from_str).collect();
    containers.sort_by_key(|&x| Reverse(x));

    let combinations = number_of_combinations(&containers);
    println!("{combinations}");
}

fn number_of_combinations(containers: &[i16]) -> i16 {
    let mut map = HashMap::new();
    recursion(containers, 0, TOTAL_VOLUME, 0, &mut map);

    let min_containers = map.keys().min().unwrap();
    map[min_containers]
}

fn recursion(
    containers: &[i16],
    ind: usize,
    volume_left: i16,
    used: i16,
    map: &mut HashMap<i16, i16>,
) {
    if volume_left < 0 {
        return;
    }
    if volume_left == 0 {
        map.entry(used).and_modify(|e| *e += 1).or_insert(1);
        return;
    }

    for i in ind..containers.len() {
        let cont = containers[i];
        recursion(containers, i + 1, volume_left - cont, used + 1, map);
    }
}
