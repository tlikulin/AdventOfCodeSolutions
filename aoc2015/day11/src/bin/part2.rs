fn main() {
    let mut password = [b'a', b'b', b'x', b'w', b'x', b'b', b'x', b'h'];
    find_next_valid(&mut password);
    find_next_valid(&mut password);
    password.reverse();

    println!("{}", str::from_utf8(&password).unwrap());
}

fn find_next_valid(password: &mut [u8; 8]) {
    increment(password);
    while !is_valid(password) {
        increment(password);
    }
}

fn increment(password: &mut [u8; 8]) {
    for symbol in password {
        match *symbol {
            b'z' => *symbol = b'a',
            _ => {
                *symbol += 1;
                break;
            }
        }
    }
}

fn is_valid(password: &[u8; 8]) -> bool {
    check1(password) && check2(password) && check3(password)
}

fn check1(password: &[u8; 8]) -> bool {
    password
        .windows(3)
        .any(|wind| wind[2] + 1 == wind[1] && wind[2] + 2 == wind[0])
}

fn check2(password: &[u8; 8]) -> bool {
    ![b'i', b'o', b'l']
        .iter()
        .any(|forbidden| password.contains(forbidden))
}

fn check3(password: &[u8; 8]) -> bool {
    let mut found = false;
    let mut i = 0;
    while i < 7 {
        match (password[i] == password[i + 1], found) {
            (true, true) => return true,
            (true, false) => (i, found) = (i + 2, true),
            (false, _) => i += 1,
        }
    }

    false
}
