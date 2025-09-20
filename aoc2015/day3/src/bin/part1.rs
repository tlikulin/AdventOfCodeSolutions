use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut set = HashSet::<(i32, i32)>::new();
    let (mut x, mut y) = (0, 0);
    set.insert((x, y));

    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }
        set.insert((x, y));
    }

    println!("{}", set.len());
}
