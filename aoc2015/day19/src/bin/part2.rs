// Rn and Ar are always added for free and don't have own replacements, so they are ignored.
// Each replacement adds `1 + count(Y)` more non-special elements
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let normalised_molecule = input
        .lines()
        .last()
        .unwrap()
        .replace("Rn", "")
        .replace("Ar", "")
        .replace("Y", "|");

    let token_count = normalised_molecule
        .chars()
        .filter(char::is_ascii_uppercase)
        .count();
    let y_count = normalised_molecule.chars().filter(|&c| c == '|').count();

    let steps = token_count - y_count - 1;
    println!("{steps}");
}
