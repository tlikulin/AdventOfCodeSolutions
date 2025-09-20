fn main() {
    let mut input = std::fs::read_to_string("input.txt").unwrap();
    remove_red(&mut input);
    let sum: i32 = input
        .split(&['[', ']', '{', '}', ':', ','])
        .flat_map(|token| token.parse::<i32>())
        .sum();

    println!("{sum}");
}

fn remove_red(json: &mut String) {
    while let Some(red) = json.find(r#":"red""#) {
        let bytes = json.as_bytes();

        let mut left = red;
        let mut to_match = 0;
        loop {
            match bytes[left] {
                b'}' => to_match += 1,
                b'{' if to_match == 0 => break,
                b'{' => to_match -= 1,
                _ => (),
            }
            left -= 1;
        }

        let mut right = red;
        let mut to_match = 0;
        loop {
            match bytes[right] {
                b'{' => to_match += 1,
                b'}' if to_match == 0 => break,
                b'}' => to_match -= 1,
                _ => (),
            }
            right += 1;
        }

        // println!("Wiped: {}", &json[left..=right]);
        json.replace_range(left..=right, "0");
    }
}
