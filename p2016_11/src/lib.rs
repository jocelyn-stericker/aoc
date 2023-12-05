use std::collections::{BTreeSet, BinaryHeap};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
enum Thing {
    Generator(u8),
    Microchip(u8),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
struct State {
    floors: Vec<BTreeSet<Thing>>,
    elevator: usize,
}

fn floor_is_ok(things: &BTreeSet<Thing>) -> bool {
    let mut explody = false;
    let mut has_gen = false;

    for thing in things {
        if let Thing::Microchip(g) = thing {
            if !things.contains(&Thing::Generator(g.to_owned())) {
                explody = true;
            }
        }
        if matches!(thing, Thing::Generator(_)) {
            has_gen = true;
        }
    }

    !explody || !has_gen
}

impl State {
    fn is_ok(&self) -> bool {
        for things in &self.floors {
            if !floor_is_ok(things) {
                return false;
            }
        }

        true
    }
    fn is_soln(&self) -> bool {
        self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }
    fn next(&self) -> Vec<State> {
        let mut next_states = vec![];

        let i = self.elevator;
        let level = &self.floors[i];
        for (j, item) in level.iter().enumerate() {
            {
                let mut new_items_on_level = level.clone();
                new_items_on_level.remove(item);
                let items = vec![item.clone()];

                for other_lvl in &[(i as i64) - 1, (i as i64) + 1] {
                    if *other_lvl < 0 {
                        continue;
                    }

                    if let Some(mut new_items_on_other_level) =
                        self.floors.get(*other_lvl as usize).cloned()
                    {
                        for item in &items {
                            new_items_on_other_level.insert(item.clone());
                        }
                        let mut next_state = self.clone();
                        next_state.floors[*other_lvl as usize] = new_items_on_other_level;
                        next_state.floors[i] = new_items_on_level.clone();
                        next_state.elevator = *other_lvl as usize;
                        if next_state.is_ok() {
                            next_states.push(next_state);
                        }
                    }
                }
            }

            for item_2 in level.iter().skip(j + 1) {
                let items = vec![item.clone(), item_2.clone()];

                // TODO: decopy
                for other_lvl in &[(i as i64) - 1, (i as i64) + 1] {
                    if *other_lvl < 0 {
                        continue;
                    }

                    if let Some(mut new_items_on_other_level) =
                        self.floors.get(*other_lvl as usize).cloned()
                    {
                        let mut new_items_on_level = level.clone();
                        for item in &items {
                            new_items_on_level.remove(item);
                            new_items_on_other_level.insert(item.clone());
                        }
                        let mut next_state = self.clone();
                        next_state.floors[*other_lvl as usize] = new_items_on_other_level;
                        next_state.floors[i] = new_items_on_level.clone();
                        next_state.elevator = *other_lvl as usize;
                        if next_state.is_ok() {
                            next_states.push(next_state);
                        }
                    }
                }
            }
        }

        next_states
    }
}

pub fn solve(input: &str, evil: bool) -> i64 {
    let mut floors = Vec::new();
    for line in input.trim().split('\n') {
        let mut floor = BTreeSet::new();
        for item in line.trim().split(' ') {
            let item: Vec<_> = item.split('_').collect();
            floor.insert(match item[1] {
                "generator" => Thing::Generator(item[0].parse().unwrap()),
                "microchip" => Thing::Microchip(item[0].parse().unwrap()),
                _ => panic!(),
            });
        }
        floors.push(floor);
    }
    floors.push(BTreeSet::new());
    if evil {
        floors[0].insert(Thing::Generator(5));
        floors[0].insert(Thing::Microchip(5));
        floors[0].insert(Thing::Generator(6));
        floors[0].insert(Thing::Microchip(6));
    }
    assert!(floors.len() == 4);

    let mut visited = BTreeSet::new();
    visited.insert(State {
        elevator: 0,
        floors: floors.clone(),
    });

    let mut q = BinaryHeap::new();
    assert!(State {
        elevator: 0,
        floors: floors.clone()
    }
    .is_ok());
    q.push((
        0,
        State {
            floors,
            elevator: 0,
        },
    ));

    let mut best_step = 0;
    while let Some((steps, state)) = q.pop() {
        if steps < best_step {
            eprintln!("{:?}", steps);
            best_step = steps;
        }
        if state.is_soln() {
            return -steps;
        }
        for next_state in state.next().into_iter() {
            if !visited.contains(&next_state) {
                visited.insert(next_state.clone());
                q.push((steps - 1, next_state));
            }
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::solve("0_microchip 1_microchip\n0_generator\n1_generator\n", false),
            11
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::solve(include_str!("input.txt"), false), 33); //1:00:48
    }

    // #[test]
    // fn part_b() {
    //     // not 53
    //     assert_eq!(super::solve(include_str!("input.txt"), true), 33); //1:24:24
    // }
}
