fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let diff: u32 = input.lines().map(str::as_bytes).map(calc_diff).sum();

    println!("{diff}");
}

fn calc_diff(string: &[u8]) -> u32 {
    let n = string.len();
    let mut i = 1;
    let mut diff = 2; // outer quotes

    while i < n - 1 {
        if string[i] != b'\\' {
            i += 1;
            continue;
        }

        match string[i + 1] {
            b'\\' | b'"' => (i, diff) = (i + 2, diff + 1),
            b'x' => (i, diff) = (i + 4, diff + 3),
            _ => unreachable!(),
        }
    }

    diff
}
