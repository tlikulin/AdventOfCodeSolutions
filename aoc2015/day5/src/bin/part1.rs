fn main() {
    let count = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter(is_nice)
        .count();

    println!("{count}");
}

fn is_nice(line: &&str) -> bool {
    condition1(line) && condition2(line) && condition3(line)
}

fn condition1(line: &&str) -> bool {
    line.chars().filter(|&c| "aeiou".contains(c)).count() >= 3
}

fn condition2(line: &&str) -> bool {
    line.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn condition3(line: &&str) -> bool {
    line.as_bytes()
        .windows(2)
        .all(|w| ![&b"ab"[..], &b"cd"[..], &b"pq"[..], &b"xy"[..]].contains(&w))
}
