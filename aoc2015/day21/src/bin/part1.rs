struct Item {
    cost: u16,
    damage: u16,
    armor: u16,
}

#[derive(Debug)]
struct Combatant {
    health: u16,
    damage: u16,
    armor: u16,
}

fn main() {
    let boss = parse_boss();

    let weapons = vec![
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armor = vec![
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
        // possible no armor
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
    ];

    let rings = vec![
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
        // possible no ring 1
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        // possible no ring 2
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
    ];

    let mut optimal = u16::MAX;

    for weapon in &weapons {
        for armorpiece in &armor {
            for (ring1, ring2) in choose_two(&rings) {
                let player = Combatant {
                    health: 100,
                    damage: weapon.damage + ring1.damage + ring2.damage,
                    armor: armorpiece.armor + ring1.armor + ring2.armor,
                };
                let cost = weapon.cost + armorpiece.cost + ring1.cost + ring2.cost;
                if cost < optimal && can_win(&boss, &player) {
                    optimal = cost;
                }
            }
        }
    }

    println!("{optimal}");
}

fn parse_boss() -> Combatant {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (mut health, mut damage, mut armor) = (0, 0, 0);

    for line in input.lines() {
        let (stat, value) = line.split_once(": ").unwrap();
        let value = value.parse().unwrap();

        match stat {
            "Hit Points" => health = value,
            "Damage" => damage = value,
            "Armor" => armor = value,
            _ => panic!("Unknown stat"),
        }
    }

    Combatant {
        health,
        damage,
        armor,
    }
}

fn can_win(boss: &Combatant, player: &Combatant) -> bool {
    let boss_damage = if boss.damage <= player.armor {
        1
    } else {
        boss.damage - player.armor
    };
    let player_damage = if player.damage <= boss.armor {
        1
    } else {
        player.damage - boss.armor
    };

    let boss_attacks = player.health.div_ceil(boss_damage);
    let player_attacks = boss.health.div_ceil(player_damage);

    boss_attacks >= player_attacks
}

fn choose_two(items: &[Item]) -> Vec<(&Item, &Item)> {
    let mut vec = Vec::new();
    let len = items.len();

    for i in 0..len {
        for j in i + 1..len {
            vec.push((&items[i], &items[j]));
        }
    }

    vec
}
