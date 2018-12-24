use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug)]
enum Type {
    Rocky,
    Wet,
    Narow,
}

impl Type {
    fn risk_level(&self) -> u64 {
        match self {
            Type::Rocky => 0,
            Type::Wet => 1,
            Type::Narow => 2,
        }
    }

    fn from_spec(erosion_level: u64) -> Type {
        match erosion_level % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narow,
            _ => panic!(),
        }
    }

    fn tools(&self) -> HashSet<Tool> {
        let mut s = HashSet::new();
        match self {
            Type::Rocky => {
                s.insert(Tool::Torch);
                s.insert(Tool::ClimbingGear);
            }
            Type::Wet => {
                s.insert(Tool::ClimbingGear);
                s.insert(Tool::Neither);
            }
            Type::Narow => {
                s.insert(Tool::Torch);
                s.insert(Tool::Neither);
            }
        }

        s
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    fn alt(&self, t: &Type) -> Option<Tool> {
        let mut s = HashSet::new();
        s.insert(*self);
        t.tools().difference(&s).next().cloned()
    }
}

fn gen_map(depth: u64, target: (u64, u64), margin: u64) -> HashMap<(u64, u64), Type> {
    let mut erosion_levels: HashMap<(u64, u64), u64> = HashMap::new();
    for y in 0..=(target.1 + margin) {
        for x in 0..=(target.0 + margin) {
            let geologic_index = if x == 0 && y == 0 {
                0
            } else if x == target.0 && y == target.1 {
                0
            } else if y == 0 {
                (x * 16807)
            } else if x == 0 {
                (y * 48271)
            } else {
                erosion_levels
                    .get(&(x - 1, y))
                    .unwrap()
                    .checked_mul(*erosion_levels.get(&(x, y - 1)).unwrap())
                    .expect("Overflow")
            };
            let erosion_level = (geologic_index + depth) % 20183;

            erosion_levels.insert((x, y), erosion_level);
        }
    }

    erosion_levels
        .iter()
        .map(|(p, v)| (*p, Type::from_spec(*v)))
        .collect()
}

pub fn part_a(depth: u64, target: (u64, u64)) -> u64 {
    gen_map(depth, target, 0)
        .values()
        .fold(0, |m, v| m + v.risk_level())
}

pub fn part_b(depth: u64, target: (u64, u64)) -> u64 {
    let map = gen_map(depth, (target.0, target.1), 1000);

    let mut scores: HashMap<(u64, u64, Tool), i64> = HashMap::new();
    let mut queue: BinaryHeap<(i64, (u64, u64), Tool)> = BinaryHeap::new(); // max pq
    queue.push((0, (0, 0), Tool::Torch));
    while let Some((score, pos, tool)) = queue.pop() {
        let score = if pos == target && tool == Tool::ClimbingGear {
            score - 7
        } else {
            score
        };

        if scores.get(&(pos.0, pos.1, tool)).unwrap_or(&-99999999) >= &score {
            continue;
        }

        let c_type = map.get(&pos).unwrap();
        queue.push(((score - 7), pos, tool.alt(&c_type).unwrap()));

        if pos == target && tool == Tool::Torch {
            return -score as u64;
        }

        scores.insert((pos.0, pos.1, tool), score);

        // Ugh.
        if pos.0 > 0 {
            let next_p = (pos.0 - 1, pos.1);
            let t = map.get(&next_p).unwrap();
            let tools = t.tools();
            if tools.contains(&tool) {
                queue.push((score - 1, next_p, tool));
            }
        }
        if pos.1 > 0 {
            let next_p = (pos.0, pos.1 - 1);
            let t = map.get(&next_p).unwrap();
            let tools = t.tools();
            if tools.contains(&tool) {
                queue.push((score - 1, next_p, tool));
            }
        }
        if map.get(&(pos.0 + 1, pos.1)).is_some() {
            let next_p = (pos.0 + 1, pos.1);
            let t = map.get(&next_p).unwrap();
            let tools = t.tools();
            if tools.contains(&tool) {
                queue.push((score - 1, next_p, tool));
            }
        }

        if map.get(&(pos.0, pos.1 + 1)).is_some() {
            let next_p = (pos.0, pos.1 + 1);
            let t = map.get(&next_p).unwrap();
            let tools = t.tools();
            if tools.contains(&tool) {
                queue.push((score - 1, next_p, tool));
            }
        }
    }

    unreachable!();
}

#[test]
fn test_sample() {
    assert_eq!(part_a(510, (10, 10)), 114);
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(5616, (10, 785)), 8681);
}

#[test]
fn test_sample_b() {
    assert_eq!(part_b(510, (10, 10)), 45);
}

#[test]
fn test_part_b() {
    assert_eq!(part_b(5616, (10, 785)), 1070);
}
