fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let sum: i32 = input
        .split(&['[', ']', '{', '}', ':', ','])
        .flat_map(|token| token.parse::<i32>())
        .sum();

    println!("{sum}");
}
