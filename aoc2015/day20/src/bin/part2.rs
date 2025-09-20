#[allow(clippy::needless_range_loop)]
fn main() {
    let threshold = 33_100_000 / 11;
    const LIMIT: usize = 1_000_000;

    let mut houses = [0; LIMIT];

    for elf in 1..LIMIT {
        for i in (elf..=(50 * elf).min(LIMIT - 1)).step_by(elf) {
            houses[i] += elf;
        }

        if houses[elf] >= threshold {
            println!("{elf} with score {}", houses[elf] * 11);
            break;
        }
    }
}
