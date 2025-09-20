enum Action {
    TurnOn,
    Toggle,
    TurnOff,
}

struct Instruction {
    action: Action,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut lights = [[0u8; 1000]; 1000];

    let input = std::fs::read_to_string("input.txt").unwrap();

    for inst in input.lines().map(parse_instruction) {
        match inst.action {
            Action::TurnOn => {
                for x in inst.x1..=inst.x2 {
                    for y in inst.y1..=inst.y2 {
                        lights[y][x] += 1;
                    }
                }
            }
            Action::Toggle => {
                for x in inst.x1..=inst.x2 {
                    for y in inst.y1..=inst.y2 {
                        lights[y][x] += 2;
                    }
                }
            }
            Action::TurnOff => {
                for x in inst.x1..=inst.x2 {
                    for y in inst.y1..=inst.y2 {
                        lights[y][x] = lights[y][x].saturating_sub(1);
                    }
                }
            }
        }
    }

    let count: u32 = lights.iter().flatten().map(|&x| x as u32).sum();

    println!("{count}");
}

fn parse_instruction(line: &str) -> Instruction {
    let mut iter = line.split_ascii_whitespace();

    let action = match iter.next() {
        Some("turn") => match iter.next() {
            Some("on") => Action::TurnOn,
            Some("off") => Action::TurnOff,
            _ => panic!(),
        },
        Some("toggle") => Action::Toggle,
        _ => panic!(),
    };

    let mut coord1 = iter.next().unwrap().split(',');
    let x1: usize = coord1.next().unwrap().parse().unwrap();
    let y1: usize = coord1.next().unwrap().parse().unwrap();

    iter.next();

    let mut coord2 = iter.next().unwrap().split(',');
    let x2: usize = coord2.next().unwrap().parse().unwrap();
    let y2: usize = coord2.next().unwrap().parse().unwrap();

    Instruction {
        action,
        x1,
        y1,
        x2,
        y2,
    }
}
