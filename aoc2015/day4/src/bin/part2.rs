use md5::{Digest, Md5};

fn main() {
    let key = "yzbqklnj";
    let mut counter: u32 = 1;

    let result = loop {
        let mut hasher = Md5::new();
        let text = format!("{key}{counter}");
        hasher.update(text.as_bytes());
        let result: [u8; 16] = hasher.finalize().into();

        if let [0x00, 0x00, 0x00, ..] = result {
            break result;
        } else {
            counter += 1;
        }
    };

    print!("{counter}: ");
    print_hash(result);
}

fn print_hash(hash: [u8; 16]) {
    for b in hash {
        print!("{b:02x}");
    }
    println!();
}
