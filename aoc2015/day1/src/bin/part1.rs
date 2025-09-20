fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut floor = 0;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }

    println!("{floor}");
}
