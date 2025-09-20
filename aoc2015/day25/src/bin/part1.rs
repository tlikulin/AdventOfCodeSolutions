fn main() {
    let first = 20_151_125usize;
    let multiplier = 252_533;
    let modulo = 33_554_393;

    let (x, y) = parse_coords();
    let diagonal = x + y - 1; // diagonal number (starts from 1)
    let prev_trianle = diagonal * (diagonal - 1) / 2; // all prev diagonals summed as a triangle
    let extra = diagonal - y + 1; // progress in the current diagonal
    let steps = prev_trianle + extra - 1; // -1 since first number is known and not calculated

    let mut current = first;
    for _ in 0..steps {
        current = (current * multiplier) % modulo;
    }

    println!("{current}");
}

fn parse_coords() -> (u32, u32) {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (y_str, x_str) = input
        .trim()
        .trim_start_matches(
            "To continue, please consult the code grid in the manual.  Enter the code at row ",
        )
        .trim_end_matches('.')
        .split_once(", column ")
        .unwrap();
    (x_str.parse().unwrap(), y_str.parse().unwrap())
}
