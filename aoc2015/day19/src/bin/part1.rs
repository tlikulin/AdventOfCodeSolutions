use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Replacement(String, String);

impl FromStr for Replacement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" => ");
        Ok(Replacement(
            iter.next().ok_or(())?.to_string(),
            iter.next().ok_or(())?.to_string(),
        ))
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut lines = input.lines();
    let mut replacements = Vec::new();

    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        replacements.push(Replacement::from_str(line).unwrap());
    }

    let base = lines.next().unwrap();

    let mut seen = HashSet::new();

    for Replacement(from, to) in replacements {
        let matches: Vec<usize> = base.match_indices(&from).map(|(ind, _)| ind).collect();
        let len = from.len();

        for at in matches {
            let new = format!("{}{to}{}", &base[..at], &base[at + len..]);
            seen.insert(new);
        }
    }

    println!("{}", seen.len());
}
