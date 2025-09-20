fn main() {
    let wrapping_paper: u32 = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(calculate_wrapping)
        .sum();

    println!("{wrapping_paper}");
}

fn calculate_wrapping(dimensions: &str) -> u32 {
    let xyz: Vec<u32> = dimensions
        .split('x')
        .map(|d| d.parse::<u32>().unwrap())
        .collect();
    let [x, y, z] = xyz[..3] else { panic!() };
    let base = 2 * (x * y + y * z + z * x);
    let smallest = (x * y).min(y * z).min(z * x);
    base + smallest
}
