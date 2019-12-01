use regex::Regex;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum AttackerType {
    Immune,
    Infection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
    id: u64,
    team: AttackerType,
    unit_hp: u64,
    units: u64,
    attack_damage: u64,
    attack_type: String,
    initiative: u64,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

impl Group {
    fn effective_damage(&self, from: &Group) -> u64 {
        let multiplier = if self.immunities.contains(&from.attack_type) {
            0
        } else if self.weaknesses.contains(&from.attack_type) {
            2
        } else {
            1
        };

        let effective_power = from.units * from.attack_damage;

        multiplier * effective_power
    }

    fn compare(&self, from: &Group) -> (u64, u64, u64) {
        (
            self.effective_damage(from),
            self.units * self.attack_damage,
            self.initiative,
        )
    }

    fn loss(&self, from: &Group) -> u64 {
        let dmg = self.effective_damage(from);
        let units_lost = dmg / self.unit_hp;

        std::cmp::min(units_lost, self.units)
    }

    fn from_str(id: u64, team: AttackerType, line: &str) -> Group {
        let line_re = Regex::new(
            r"(?x)
            (?P<units>\d+)\sunits\seach\swith\s
            (?P<unit_hp>\d+)\shit\spoints\s
            (\((immune\sto\s(?P<immunities>[^;)]+);?\s?)?
                (weak\sto\s(?P<weaknesses>[^)]+))?\)\s)?
            with\san\sattack\sthat\sdoes\s
            (?P<attack_damage>\d+)\s
            (?P<attack_type>\S+)\sdamage\sat\sinitiative\s
            (?P<initiative>\d+)",
        )
        .unwrap();

        let line = line_re.captures(line).unwrap();

        Group {
            id,
            team,
            unit_hp: line
                .name("unit_hp")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap(),
            units: line.name("units").unwrap().as_str().parse::<u64>().unwrap(),
            attack_damage: line
                .name("attack_damage")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap(),
            attack_type: line.name("attack_type").unwrap().as_str().to_owned(),
            initiative: line
                .name("initiative")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap(),
            weaknesses: line
                .name("weaknesses")
                .map(|a| a.as_str())
                .unwrap_or(&"")
                .split(", ")
                .filter_map(|a| if a == "" { None } else { Some(a.to_owned()) })
                .collect(),
            immunities: line
                .name("immunities")
                .map(|a| a.as_str())
                .unwrap_or(&"")
                .split(", ")
                .filter_map(|a| if a == "" { None } else { Some(a.to_owned()) })
                .collect(),
        }
    }
}

fn select_targets(
    attack: &Vec<Group>,
    defend: &Vec<Group>,
    atk_type: AttackerType,
) -> Vec<(u64, AttackerType, usize, usize)> {
    let mut targets = Vec::new();
    let mut remaining_defend: Vec<Group> = defend.iter().filter(|x| x.units > 0).cloned().collect();
    let mut attack = attack.clone();
    attack.sort_by_key(|i| {
        (
            -((i.units * i.attack_damage) as i64),
            -(i.initiative as i64),
        )
    });

    for i in attack.iter() {
        if i.units == 0 {
            continue;
        }

        let best = remaining_defend.iter().max_by_key(|j| j.compare(i));

        if best.is_none() {
            break;
        }

        let best = best.unwrap();

        let best_score = best.compare(i);

        if best_score.0 == 0 {
            continue;
        }

        assert_eq!(
            remaining_defend
                .iter()
                .filter(|j| j.compare(i) == best_score)
                .count(),
            1
        );

        let defend_idx = remaining_defend.iter().position(|b| b == best).unwrap();
        let defend_id = best.id;
        remaining_defend.remove(defend_idx);
        targets.push((i.initiative, atk_type, i.id as usize, defend_id as usize));
    }

    targets
}

fn parse(input: &str) -> (Vec<Group>, Vec<Group>) {
    let mut input = input.split('\n').filter(|line| line != &"");
    assert_eq!(input.next().unwrap(), "Immune System:");
    let mut immune_system = Vec::new();
    let mut infection = Vec::new();

    let mut id = 0;
    while let Some(line) = input.next() {
        if line == "Infection:" {
            break;
        }
        immune_system.push(Group::from_str(id, AttackerType::Immune, line));
        id += 1;
    }

    let mut id = 0;
    while let Some(line) = input.next() {
        infection.push(Group::from_str(id, AttackerType::Infection, line));
        id += 1;
    }

    (infection, immune_system)
}

fn solve(infection: &Vec<Group>, immune_system: &Vec<Group>, boost: u64) -> Option<i64> {
    let mut infection = infection.clone();
    let mut immune_system = immune_system.clone();

    for i in &mut immune_system {
        i.attack_damage += boost;
    }

    while immune_system.iter().filter(|i| i.units > 0).count() > 0
        && infection.iter().filter(|i| i.units > 0).count() > 0
    {
        let mut targets = BinaryHeap::new();
        for target in select_targets(&infection, &immune_system, AttackerType::Infection) {
            targets.push(target);
        }
        for target in select_targets(&immune_system, &infection, AttackerType::Immune) {
            targets.push(target);
        }

        let mut attacked = false;

        while let Some(target) = targets.pop() {
            match target.1 {
                AttackerType::Immune => {
                    let loss = infection[target.3].loss(&immune_system[target.2]);
                    infection[target.3].units -= loss;
                    attacked = attacked || loss > 0;
                }
                AttackerType::Infection => {
                    let loss = immune_system[target.3].loss(&infection[target.2]);
                    immune_system[target.3].units -= loss;
                    attacked = attacked || loss > 0;
                }
            }
        }

        if !attacked {
            return None;
        }
    }

    Some(
        (immune_system.iter().fold(0, |m, i| m + i.units) as i64)
            - (infection.iter().fold(0, |m, i| m + i.units) as i64),
    )
}

pub fn part_a(input: &str) -> u64 {
    let (infection, immune_system) = parse(input);

    solve(&infection, &immune_system, 0).unwrap().abs() as u64
}

pub fn part_b(input: &str) -> u64 {
    let (infection, immune_system) = parse(input);

    for boost in 0.. {
        println!("Checking {}", boost);
        if let Some(res) = solve(&infection, &immune_system, boost) {
            if res > 0 {
                return res as u64;
            }
        }
    }

    unreachable!();
}

#[test]
fn test_sample_1() {
    assert_eq!(part_a(include_str!("sample.txt")), 5216);
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(include_str!("input.txt")), 25088);
}

#[test]
fn test_part_b() {
    // not 61
    assert_eq!(part_b(include_str!("input.txt")), 2002);
}
