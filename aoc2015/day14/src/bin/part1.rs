const TOTAL_TIME: u32 = 2503;

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

use std::{num::ParseIntError, str::FromStr};
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
        })
    }
}

impl Reindeer {
    fn calculate_distance(&self) -> u32 {
        let cycle_time = self.fly_time + self.rest_time;
        let cycle_distance = self.speed * self.fly_time;
        let cycles = TOTAL_TIME / cycle_time;

        let extra = (TOTAL_TIME % cycle_time).min(self.fly_time) * self.speed;

        cycles * cycle_distance + extra
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let deer: Vec<Reindeer> = input
        .lines()
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .collect();

    let max_distance = deer.iter().map(Reindeer::calculate_distance).max().unwrap();

    println!("{max_distance}");
}
