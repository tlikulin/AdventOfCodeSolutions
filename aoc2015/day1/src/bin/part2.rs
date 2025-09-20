fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut floor = 0;

    for (ind, c) in input.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor < 0 {
            println!("{}", ind + 1);
            break;
        }
    }
}
