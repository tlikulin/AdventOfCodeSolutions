use std::{num::ParseIntError, str::FromStr};

const TOTAL_TIME: u32 = 2503;

enum ReindeerState {
    Flying(u32),
    Resting(u32),
}

struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
    distance: u32,
    state: ReindeerState,
    score: u32,
}

impl FromStr for Reindeer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        let speed = words[3].parse()?;
        let fly_time = words[6].parse()?;
        let rest_time = words[13].parse()?;
        Ok(Reindeer {
            speed,
            fly_time,
            rest_time,
            distance: 0,
            state: ReindeerState::Flying(fly_time),
            score: 0,
        })
    }
}

impl Reindeer {
    fn next_second(&mut self) {
        use ReindeerState as S;

        if let ReindeerState::Flying(_) = self.state {
            self.distance += self.speed;
        }

        self.state = match self.state {
            S::Flying(1) => S::Resting(self.rest_time),
            S::Flying(t) => S::Flying(t - 1),
            S::Resting(1) => S::Flying(self.fly_time),
            S::Resting(t) => S::Resting(t - 1),
        };
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut deer: Vec<Reindeer> = input
        .lines()
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .collect();

    for _ in 0..TOTAL_TIME {
        for reindeer in &mut deer {
            reindeer.next_second();
        }

        let max_distance = deer.iter().map(|d| d.distance).max().unwrap();

        for reindeer in deer.iter_mut().filter(|d| d.distance == max_distance) {
            reindeer.score += 1;
        }
    }

    let best = deer.iter().map(|d| d.score).max().unwrap();

    println!("{best}");
}
