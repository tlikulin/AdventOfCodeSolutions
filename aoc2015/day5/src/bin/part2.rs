fn main() {
    let count = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(str::as_bytes)
        .filter(is_nice)
        .count();

    println!("{count}");
}

fn is_nice(line: &&[u8]) -> bool {
    condition1(line) && condition2(line)
}

fn condition1(line: &&[u8]) -> bool {
    let n = line.len();
    if n < 4 {
        return false;
    }

    for i in 0..n - 3 {
        for j in i + 2..n - 1 {
            if line[i] == line[j] && line[i + 1] == line[j + 1] {
                return true;
            }
        }
    }
    false
}

fn condition2(line: &&[u8]) -> bool {
    let n = line.len();
    for i in 0..n - 2 {
        if line[i] == line[i + 2] {
            return true;
        }
    }
    false
}
