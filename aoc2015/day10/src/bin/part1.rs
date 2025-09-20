fn main() {
    let mut sequence = vec![3, 1, 1, 3, 3, 2, 2, 1, 1, 3];
    for _ in 0..40 {
        sequence = next_sequence(sequence);
    }
    //print_sequnce(&sequence);
    println!("Length: {}", sequence.len());
}

fn next_sequence(sequence: Vec<u8>) -> Vec<u8> {
    let mut count: u8 = 1;
    let mut digit: u8 = sequence[0];
    let mut output = Vec::new();

    for dig in sequence.into_iter().skip(1) {
        if dig == digit {
            count += 1;
        } else {
            debug_assert!(count < 10);
            output.push(count);
            output.push(digit);
            digit = dig;
            count = 1;
        }
    }

    debug_assert!(count < 10);
    output.push(count);
    output.push(digit);

    output
}
