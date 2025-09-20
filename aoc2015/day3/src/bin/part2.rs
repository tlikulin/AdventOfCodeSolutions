use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut set = HashSet::<(i32, i32)>::new();
    let mut coords = [(0, 0); 2];
    let mut turn = 0;
    set.insert((0, 0));

    for c in input.chars() {
        match c {
            '^' => coords[turn].1 += 1,
            'v' => coords[turn].1 -= 1,
            '>' => coords[turn].0 += 1,
            '<' => coords[turn].0 -= 1,
            _ => (),
        }
        set.insert(coords[turn]);
        turn = 1 - turn;
    }

    println!("{}", set.len());
}
