fn main() {
    let ribbon: u32 = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(calculate_ribbon)
        .sum();

    println!("{ribbon}");
}

fn calculate_ribbon(dimensions: &str) -> u32 {
    let xyz: Vec<u32> = dimensions
        .split('x')
        .map(|d| d.parse::<u32>().unwrap())
        .collect();
    let [x, y, z] = xyz[..3] else { panic!() };
    let smallest_perimeter = 2 * (x + y).min(y + z).min(z + x);
    let extra = x * y * z;
    smallest_perimeter + extra
}
