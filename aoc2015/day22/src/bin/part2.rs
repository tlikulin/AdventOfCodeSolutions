#[derive(Clone, Copy)]
struct Boss {
    health: u16,
    damage: u16,
    poisoning: Option<Effect>,
}

impl Boss {
    fn take_damage(&mut self, damage: u16) -> bool {
        if self.health <= damage {
            true
        } else {
            self.health -= damage;
            false
        }
    }

    fn poison(&mut self) -> bool {
        if let Some(Effect { duration, strength }) = self.poisoning {
            if self.take_damage(strength) {
                return true;
            }

            self.poisoning = if duration == 1 {
                None
            } else {
                Some(Effect {
                    duration: duration - 1,
                    strength,
                })
            }
        }

        false
    }
}

#[derive(Clone, Copy)]
struct Player {
    health: u16,
    mana: u16,
    shielding: Option<Effect>,
    recharging: Option<Effect>,
}

impl Player {
    fn take_damage(&mut self, damage: u16) -> bool {
        let armor = match self.shielding {
            Some(Effect {
                duration: _,
                strength,
            }) => strength,
            None => 0,
        };

        let damage = if damage <= armor { 1 } else { damage - armor };

        if self.health <= damage {
            true
        } else {
            self.health -= damage;
            false
        }
    }

    fn can_cast(&self, spell: &Spell, boss: &Boss) -> bool {
        match *spell {
            Spell::MagicMissile { cost, damage: _ } | Spell::Drain { cost, damage: _ } => {
                self.mana >= cost
            }
            Spell::Shield { cost, effect: _ } => {
                self.mana >= cost
                    && (self.shielding.is_none() || self.shielding.unwrap().duration == 1)
            }
            Spell::Poison { cost, effect: _ } => {
                self.mana >= cost
                    && (boss.poisoning.is_none() || boss.poisoning.unwrap().duration == 1)
            }
            Spell::Recharge { cost, effect: _ } => {
                self.mana >= cost
                    && (self.recharging.is_none() || self.recharging.unwrap().duration == 1)
            }
        }
    }

    fn recharge_and_shield(&mut self) {
        if let Some(Effect { duration, strength }) = self.recharging {
            self.mana += strength;
            self.recharging = if duration == 1 {
                None
            } else {
                Some(Effect {
                    duration: duration - 1,
                    strength,
                })
            };
        }

        if let Some(Effect { duration, strength }) = self.shielding {
            self.shielding = if duration == 1 {
                None
            } else {
                Some(Effect {
                    duration: duration - 1,
                    strength,
                })
            };
        }
    }
}

enum Spell {
    MagicMissile { cost: u16, damage: u16 },
    Drain { cost: u16, damage: u16 },
    Shield { cost: u16, effect: Effect },
    Poison { cost: u16, effect: Effect },
    Recharge { cost: u16, effect: Effect },
}

impl Spell {
    fn cost(&self) -> u16 {
        match *self {
            Spell::MagicMissile { cost, damage: _ }
            | Spell::Drain { cost, damage: _ }
            | Spell::Shield { cost, effect: _ }
            | Spell::Poison { cost, effect: _ }
            | Spell::Recharge { cost, effect: _ } => cost,
        }
    }
}

#[derive(Clone, Copy)]
struct Effect {
    duration: u8,
    strength: u16,
}

fn main() {
    let boss = parse_boss();
    let player = Player {
        health: 50,
        mana: 500,
        shielding: None,
        recharging: None,
    };

    let spells = [
        Spell::MagicMissile {
            cost: 53,
            damage: 4,
        },
        Spell::Drain {
            cost: 73,
            damage: 2,
        },
        Spell::Shield {
            cost: 113,
            effect: Effect {
                duration: 6,
                strength: 7,
            },
        },
        Spell::Poison {
            cost: 173,
            effect: Effect {
                duration: 6,
                strength: 3,
            },
        },
        Spell::Recharge {
            cost: 229,
            effect: Effect {
                duration: 5,
                strength: 101,
            },
        },
    ];

    let best = start_fight(player, boss, &spells);
    println!("{best}");
}

fn parse_boss() -> Boss {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (mut health, mut damage) = (0, 0);

    for line in input.lines() {
        let (stat, value) = line.split_once(": ").unwrap();
        let value = value.parse().unwrap();

        match stat {
            "Hit Points" => health = value,
            "Damage" => damage = value,
            _ => panic!("Unknown stat"),
        }
    }

    Boss {
        health,
        damage,
        poisoning: None,
    }
}

fn start_fight(player: Player, boss: Boss, spells: &[Spell]) -> u16 {
    let mut record = u16::MAX;

    for spell in spells {
        fight(player, boss, spells, &mut record, spell, 0);
    }

    record
}

fn fight(
    mut player: Player,
    mut boss: Boss,
    spells: &[Spell],
    record: &mut u16,
    to_cast: &Spell,
    mut mana_spent: u16,
) {
    if mana_spent > *record {
        return;
    }

    // PLAYER TURN
    if player.take_damage(1) {
        return;
    }
    if boss.poison() {
        *record = mana_spent.min(*record);
        return;
    }
    player.recharge_and_shield();

    mana_spent += to_cast.cost();
    player.mana -= to_cast.cost();
    match *to_cast {
        Spell::MagicMissile { cost: _, damage } => {
            if boss.take_damage(damage) {
                *record = mana_spent.min(*record);
                return;
            }
        }
        Spell::Drain { cost: _, damage } => {
            if boss.take_damage(damage) {
                *record = mana_spent.min(*record);
                return;
            }
            player.health += damage;
        }
        Spell::Shield { cost: _, effect } => player.shielding = Some(effect),
        Spell::Poison { cost: _, effect } => boss.poisoning = Some(effect),
        Spell::Recharge { cost: _, effect } => player.recharging = Some(effect),
    }

    // BOSS TURN
    if boss.poison() {
        *record = mana_spent.min(*record);
        return;
    }
    player.recharge_and_shield();

    if player.take_damage(boss.damage) {
        return;
    }

    for spell in spells {
        if player.can_cast(spell, &boss) {
            fight(player, boss, spells, record, spell, mana_spent);
        }
    }
}
