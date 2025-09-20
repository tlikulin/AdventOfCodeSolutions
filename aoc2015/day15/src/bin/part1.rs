use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
}

impl FromStr for Ingredient {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s
            .split_ascii_whitespace()
            .map(|w| w.trim_end_matches(','))
            .collect();

        let capacity = words[2].parse()?;
        let durability = words[4].parse()?;
        let flavor = words[6].parse()?;
        let texture = words[8].parse()?;

        Ok(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
        })
    }
}

impl Ingredient {
    fn get_score(&self) -> i32 {
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }

    fn makeable(&self, best: &Self, left: i32) -> bool {
        (self.capacity + best.capacity * left) > 0
            && (self.durability + best.durability * left) > 0
            && (self.flavor + best.flavor * left) > 0
            && (self.texture + best.texture * left) > 0
    }

    fn add(&mut self, other: &Self) {
        self.capacity += other.capacity;
        self.durability += other.durability;
        self.flavor += other.flavor;
        self.texture += other.texture;
    }

    fn sub(&mut self, other: &Self) {
        self.capacity -= other.capacity;
        self.durability -= other.durability;
        self.flavor -= other.flavor;
        self.texture -= other.texture;
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let ingredients: Vec<Ingredient> = input.lines().flat_map(FromStr::from_str).collect();
    let best = best_stats(&ingredients);

    let score = highest_score(&ingredients, &best);
    println!("{score}");
}

fn best_stats(ingredients: &[Ingredient]) -> Ingredient {
    let capacity = ingredients.iter().map(|ing| ing.capacity).max().unwrap();
    let durability = ingredients.iter().map(|ing| ing.durability).max().unwrap();
    let flavor = ingredients.iter().map(|ing| ing.flavor).max().unwrap();
    let texture = ingredients.iter().map(|ing| ing.texture).max().unwrap();

    Ingredient {
        capacity,
        durability,
        flavor,
        texture,
    }
}

fn highest_score(ingredients: &[Ingredient], best: &Ingredient) -> i32 {
    let mut record = 0;
    let mut current = Ingredient {
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
    };

    recursion(ingredients, best, &mut record, &mut current, 100, 0);
    record
}

fn recursion(
    ingredients: &[Ingredient],
    best: &Ingredient,
    record: &mut i32,
    current: &mut Ingredient,
    left: i32,
    at: usize,
) {
    if !current.makeable(best, left) {
        return;
    }

    if left == 0 {
        *record = (*record).max(current.get_score());
        return;
    }

    for i in at..ingredients.len() {
        let ing = &ingredients[i];
        current.add(ing);
        recursion(ingredients, best, record, current, left - 1, i);
        current.sub(ing);
    }
}
