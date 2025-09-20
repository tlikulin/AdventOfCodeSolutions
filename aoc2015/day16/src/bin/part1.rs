use std::{num::ParseIntError, str::FromStr};

#[derive(Default, Debug)]
struct Sample {
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

impl FromStr for Sample {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s
            .split_ascii_whitespace()
            .map(|t| t.trim_end_matches([',', ':']))
            .collect();
        let compounds = [
            (tokens[2], tokens[3]),
            (tokens[4], tokens[5]),
            (tokens[6], tokens[7]),
        ];
        let mut sample = Sample::default();

        for (name, value) in compounds {
            let value = value.parse()?;
            match name {
                "children" => sample.children = Some(value),
                "cats" => sample.cats = Some(value),
                "samoyeds" => sample.samoyeds = Some(value),
                "pomeranians" => sample.pomeranians = Some(value),
                "akitas" => sample.akitas = Some(value),
                "vizslas" => sample.vizslas = Some(value),
                "goldfish" => sample.goldfish = Some(value),
                "trees" => sample.trees = Some(value),
                "cars" => sample.cars = Some(value),
                "perfumes" => sample.perfumes = Some(value),
                _ => panic!("Unknown compound found"),
            }
        }

        Ok(sample)
    }
}

impl Sample {
    fn matches(&self, sample: &Self) -> bool {
        sample
            .children
            .is_none_or(|_| sample.children == self.children)
            && sample.cats.is_none_or(|_| sample.cats == self.cats)
            && sample
                .samoyeds
                .is_none_or(|_| sample.samoyeds == self.samoyeds)
            && sample
                .pomeranians
                .is_none_or(|_| sample.pomeranians == self.pomeranians)
            && sample.akitas.is_none_or(|_| sample.akitas == self.akitas)
            && sample
                .vizslas
                .is_none_or(|_| sample.vizslas == self.vizslas)
            && sample
                .goldfish
                .is_none_or(|_| sample.goldfish == self.goldfish)
            && sample.trees.is_none_or(|_| sample.trees == self.trees)
            && sample.cars.is_none_or(|_| sample.cars == self.cars)
            && sample
                .perfumes
                .is_none_or(|_| sample.perfumes == self.perfumes)
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let gift = Sample {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let count = input
        .lines()
        .flat_map(FromStr::from_str)
        .position(|s| gift.matches(&s))
        .unwrap()
        + 1;

    println!("{count}");
}
