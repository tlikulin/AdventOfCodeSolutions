use std::iter::once;

const N: usize = 100;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut lights = init_lights(input);
    let mut neighbors = vec![vec![0u8; N + 2]; N + 2];

    for _ in 0..N {
        update_neighbors(&mut neighbors, &lights);
        update_lights(&mut lights, &neighbors);
    }

    let count = lights.iter().flatten().filter(|&&l| l).count();
    println!("{count}");
}

fn init_lights(input: String) -> Vec<Vec<bool>> {
    let mut lights = Vec::with_capacity(N + 2);
    lights.push(vec![false; 102]);
    for line in input.lines().map(str::as_bytes) {
        let row: Vec<bool> = once(b'.')
            .chain(line.iter().copied())
            .chain(once(b'.'))
            .map(|b| b == b'#')
            .collect();
        lights.push(row);
    }
    lights.push(vec![false; N + 2]);

    lights[1][1] = true;
    lights[1][N] = true;
    lights[N][1] = true;
    lights[N][N] = true;

    lights
}

fn update_neighbors(neighbors: &mut [Vec<u8>], lights: &[Vec<bool>]) {
    for y in 1..N + 1 {
        for x in 1..N + 1 {
            let mut new = 0;

            if lights[y - 1][x - 1] {
                new += 1;
            }
            if lights[y - 1][x] {
                new += 1;
            }
            if lights[y - 1][x + 1] {
                new += 1;
            }
            if lights[y][x + 1] {
                new += 1;
            }
            if lights[y + 1][x + 1] {
                new += 1;
            }
            if lights[y + 1][x] {
                new += 1;
            }
            if lights[y + 1][x - 1] {
                new += 1;
            }
            if lights[y][x - 1] {
                new += 1;
            }

            neighbors[y][x] = new;
        }
    }
}

fn update_lights(lights: &mut [Vec<bool>], neighbors: &[Vec<u8>]) {
    for y in 1..N + 1 {
        for x in 1..N + 1 {
            if matches!((y, x), (1, 1) | (1, N) | (N, 1) | (N, N)) {
                continue;
            }

            match (lights[y][x], neighbors[y][x]) {
                (true, 2..=3) => (),
                (true, _) => lights[y][x] = false,
                (false, 3) => lights[y][x] = true,
                (false, _) => (),
            }
        }
    }
}
