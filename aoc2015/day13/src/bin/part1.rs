use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let words: Vec<Vec<&str>> = input
        .split_terminator(".\n")
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect();

    let table = construct_table(words);

    let optimal = max_happiness(table);

    println!("{optimal}");
}

fn construct_table(words: Vec<Vec<&str>>) -> Vec<Vec<i32>> {
    #[allow(clippy::manual_div_ceil)]
    let n = (1 + usize::isqrt(1 + 4 * words.len())) / 2; // get n from n(n - 1) = #lines

    let mut ids = HashMap::new();
    for i in (0..words.len()).step_by(n - 1) {
        ids.insert(words[i][0], ids.len());
    }

    let mut table = vec![vec![0; n]; n];
    for record in words {
        let to = ids[record[0]];
        let sign = match record[2] {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!(),
        };
        let value: i32 = record[3].parse().unwrap();
        let from = ids[record[10]];

        table[from][to] = sign * value;
    }

    table
}

// circular so can fix the first person
fn max_happiness(table: Vec<Vec<i32>>) -> i32 {
    let n = table.len();
    let mut used = vec![false; n];
    used[0] = true;
    let mut arrangement = vec![0];
    let mut record = i32::MIN;

    max_happiness_inner(&table, &mut used, &mut arrangement, &mut record);
    record
}

fn max_happiness_inner(
    table: &[Vec<i32>],
    used: &mut [bool],
    arrangement: &mut Vec<usize>,
    record: &mut i32,
) {
    if arrangement.len() == table.len() {
        *record = (*record).max(calculate_happiness(table, arrangement));
        return;
    }

    for i in 0..table.len() {
        if !used[i] {
            used[i] = true;
            arrangement.push(i);
            max_happiness_inner(table, used, arrangement, record);
            arrangement.pop();
            used[i] = false;
        }
    }
}

fn calculate_happiness(table: &[Vec<i32>], arrangement: &[usize]) -> i32 {
    let mut happiness = 0;

    for ids in arrangement.windows(2) {
        let &[id1, id2] = ids else { unreachable!() };
        happiness += table[id1][id2];
        happiness += table[id2][id1];
    }

    let (first, last) = (0, *arrangement.last().unwrap());
    happiness += table[first][last];
    happiness += table[last][first];

    happiness
}
