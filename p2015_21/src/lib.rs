#[derive(Debug, Clone)]
struct Item {
    name: &'static str,
    cost: i64,
    damage: i64,
    armor: i64,
}

#[derive(Debug, Clone)]
struct Player {
    hp: i64,
    damage: i64,
    armor: i64,
}

fn wins(mut player: Player, mut boss: Player) -> bool {
    loop {
        boss.hp -= (player.damage - boss.armor).max(1);
        if boss.hp <= 0 {
            return true;
        }

        player.hp -= (boss.damage - player.armor).max(1);
        if player.hp <= 0 {
            return false;
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut weapons = Vec::new();
    let mut armor = Vec::new();
    let mut rings = Vec::new();
    weapons.push(Item {
        name: "Dagger",
        cost: 8,
        damage: 4,
        armor: 0,
    });
    weapons.push(Item {
        name: "Shortsword",
        cost: 10,
        damage: 5,
        armor: 0,
    });
    weapons.push(Item {
        name: "Warhammer",
        cost: 25,
        damage: 6,
        armor: 0,
    });
    weapons.push(Item {
        name: "Longsword",
        cost: 40,
        damage: 7,
        armor: 0,
    });
    weapons.push(Item {
        name: "Greataxe",
        cost: 74,
        damage: 8,
        armor: 0,
    });
    armor.push(Item {
        name: "Leather",
        cost: 13,
        damage: 0,
        armor: 1,
    });
    armor.push(Item {
        name: "Chainmail",
        cost: 31,
        damage: 0,
        armor: 2,
    });
    armor.push(Item {
        name: "Splintmail",
        cost: 53,
        damage: 0,
        armor: 3,
    });
    armor.push(Item {
        name: "Bandedmail",
        cost: 75,
        damage: 0,
        armor: 4,
    });
    armor.push(Item {
        name: "Platemail",
        cost: 102,
        damage: 0,
        armor: 5,
    });
    armor.push(Item {
        name: "None",
        cost: 0,
        damage: 0,
        armor: 0,
    });
    rings.push(Item {
        name: "Damage +1",
        cost: 25,
        damage: 1,
        armor: 0,
    });
    rings.push(Item {
        name: "Damage +2",
        cost: 50,
        damage: 2,
        armor: 0,
    });
    rings.push(Item {
        name: "Damage +3",
        cost: 100,
        damage: 3,
        armor: 0,
    });
    rings.push(Item {
        name: "Defense +1",
        cost: 20,
        damage: 0,
        armor: 1,
    });
    rings.push(Item {
        name: "Defense +2",
        cost: 40,
        damage: 0,
        armor: 2,
    });
    rings.push(Item {
        name: "Defense +3",
        cost: 80,
        damage: 0,
        armor: 3,
    });
    rings.push(Item {
        name: "None 0",
        cost: 0,
        damage: 0,
        armor: 0,
    });
    rings.push(Item {
        name: "None 1",
        cost: 0,
        damage: 0,
        armor: 0,
    });

    let mut lines = input.trim().split('\n');
    let boss = {
        let hp: i64 = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let damage: i64 = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let armor: i64 = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        Player { hp, damage, armor }
    };

    let mut best = i64::MAX;

    for weapon in &weapons {
        for armor in &armor {
            for (i, ring_1) in rings.iter().enumerate() {
                for ring_2 in &rings[i + 1..] {
                    let player = Player {
                        hp: 100,
                        damage: weapon.damage + armor.damage + ring_1.damage + ring_2.damage,
                        armor: weapon.armor + armor.armor + ring_1.armor + ring_2.armor,
                    };

                    if wins(player, boss.clone()) {
                        best = best.min(weapon.cost + armor.cost + ring_1.cost + ring_2.cost);
                    }
                }
            }
        }
    }

    best
}

pub fn part_b(input: &str) -> i64 {
    let mut weapons = Vec::new();
    let mut armor = Vec::new();
    let mut rings = Vec::new();
    weapons.push(Item {
        name: "Dagger",
        cost: 8,
        damage: 4,
        armor: 0,
    });
    weapons.push(Item {
        name: "Shortsword",
        cost: 10,
        damage: 5,
        armor: 0,
    });
    weapons.push(Item {
        name: "Warhammer",
        cost: 25,
        damage: 6,
        armor: 0,
    });
    weapons.push(Item {
        name: "Longsword",
        cost: 40,
        damage: 7,
        armor: 0,
    });
    weapons.push(Item {
        name: "Greataxe",
        cost: 74,
        damage: 8,
        armor: 0,
    });
    armor.push(Item {
        name: "Leather",
        cost: 13,
        damage: 0,
        armor: 1,
    });
    armor.push(Item {
        name: "Chainmail",
        cost: 31,
        damage: 0,
        armor: 2,
    });
    armor.push(Item {
        name: "Splintmail",
        cost: 53,
        damage: 0,
        armor: 3,
    });
    armor.push(Item {
        name: "Bandedmail",
        cost: 75,
        damage: 0,
        armor: 4,
    });
    armor.push(Item {
        name: "Platemail",
        cost: 102,
        damage: 0,
        armor: 5,
    });
    armor.push(Item {
        name: "None",
        cost: 0,
        damage: 0,
        armor: 0,
    });
    rings.push(Item {
        name: "Damage +1",
        cost: 25,
        damage: 1,
        armor: 0,
    });
    rings.push(Item {
        name: "Damage +2",
        cost: 50,
        damage: 2,
        armor: 0,
    });
    rings.push(Item {
        name: "Damage +3",
        cost: 100,
        damage: 3,
        armor: 0,
    });
    rings.push(Item {
        name: "Defense +1",
        cost: 20,
        damage: 0,
        armor: 1,
    });
    rings.push(Item {
        name: "Defense +2",
        cost: 40,
        damage: 0,
        armor: 2,
    });
    rings.push(Item {
        name: "Defense +3",
        cost: 80,
        damage: 0,
        armor: 3,
    });
    rings.push(Item {
        name: "None 0",
        cost: 0,
        damage: 0,
        armor: 0,
    });
    rings.push(Item {
        name: "None 1",
        cost: 0,
        damage: 0,
        armor: 0,
    });

    let mut lines = input.trim().split('\n');
    let boss = {
        let hp: i64 = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let damage: i64 = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let armor: i64 = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        Player { hp, damage, armor }
    };

    let mut best = i64::MIN;

    for weapon in &weapons {
        for armor in &armor {
            for (i, ring_1) in rings.iter().enumerate() {
                for ring_2 in &rings[i + 1..] {
                    let player = Player {
                        hp: 100,
                        damage: weapon.damage + armor.damage + ring_1.damage + ring_2.damage,
                        armor: weapon.armor + armor.armor + ring_1.armor + ring_2.armor,
                    };

                    if !wins(player, boss.clone()) {
                        best = best.max(weapon.cost + armor.cost + ring_1.cost + ring_2.cost);
                    }
                }
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 91);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 158);
    }
}
