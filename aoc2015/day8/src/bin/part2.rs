fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let diff: u32 = input.lines().map(str::as_bytes).map(calc_other_diff).sum();

    println!("{diff}");
}

fn calc_other_diff(string: &[u8]) -> u32 {
    let n = string.len();
    let mut diff = 4; // new outer quotes + escaping old outer quotes

    for b in &string[1..n - 1] {
        if b == &b'\\' || b == &b'"' {
            diff += 1;
        }
    }

    diff
}
