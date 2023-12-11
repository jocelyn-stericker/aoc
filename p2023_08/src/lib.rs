use std::collections::HashMap;

pub fn part_a(input: &str) -> usize {
    let mut lines = input.trim().split('\n');
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();

    lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let (from, to) = line.split_once(" = ").unwrap();
        let (left, right) = to
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();
        map.insert(from, (left, right));
    }

    let mut location = "AAA";
    for i in 0usize.. {
        if location == "ZZZ" {
            return i;
        }

        location = match instructions[i % instructions.len()] {
            'L' => map[location].0,
            'R' => map[location].1,
            _ => unreachable!(),
        }
    }
    unreachable!();
}

pub fn part_b(input: &str) -> usize {
    let mut lines = input.trim().split('\n');
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();

    lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let (from, to) = line.split_once(" = ").unwrap();
        let (left, right) = to
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();
        map.insert(from, (left, right));
    }

    let mut locations = map
        .iter()
        .map(|(k, _)| (*k, *k))
        .filter(|m| m.0.ends_with('A'))
        .collect::<Vec<_>>();

    let mut loop_history: HashMap<&str, HashMap<(usize, &str), usize>> = HashMap::new();
    let mut loop_lengths: HashMap<&str, (usize, usize)> = HashMap::new();
    let mut loop_good_states: HashMap<&str, Vec<usize>> = HashMap::new();

    for i in 0usize.. {
        for (start, now) in &locations {
            if now.ends_with('Z') {
                loop_good_states.entry(start).or_default().push(i);
            }
        }

        locations = locations
            .into_iter()
            .filter_map(|(start, location)| {
                let next_location = match instructions[i % instructions.len()] {
                    'L' => map[location].0,
                    'R' => map[location].1,
                    _ => unreachable!(),
                };
                if next_location == start {
                    loop_lengths.insert(start, (0, i));
                    return None;
                }
                let loop_history = loop_history.entry(start).or_default();
                if let Some(offset) = loop_history.get(&(i % instructions.len(), next_location)) {
                    loop_lengths.insert(start, (*offset, i));
                    None
                } else {
                    loop_history.insert((i % instructions.len(), next_location), i);
                    Some((start, next_location))
                }
            })
            .collect();

        if locations.is_empty() {
            break;
        }
    }

    #[derive(Debug)]
    struct Sys {
        i: usize,
        good: Vec<usize>,
        reset_at: usize,
        reset_to: usize,
    }
    impl Sys {
        fn next_step(&self) -> usize {
            self.good
                .iter()
                .filter_map(|i| if *i > self.i { Some(*i - self.i) } else { None })
                .min()
                .unwrap_or(self.reset_at - self.i)
                .min(self.reset_at - self.i)
        }

        fn increment(&mut self, i: usize) {
            self.i += i;
        }

        fn roll(&mut self) {
            if self.i == self.reset_at {
                self.i = self.reset_to;
            }
        }

        fn is_good(&self) -> bool {
            self.good.contains(&self.i)
        }
    }

    let mut systems = loop_lengths
        .iter()
        .map(|(start, (reset_to, reset_at))| {
            let good = loop_good_states[start].clone();
            Sys {
                i: 0,
                good,
                reset_at: *reset_at,
                reset_to: *reset_to,
            }
        })
        .collect::<Vec<_>>();

    let mut answer = 0;
    loop {
        let increment = systems.iter().map(|s| s.next_step()).min().unwrap();
        answer += increment;
        for sys in &mut systems {
            sys.increment(increment);
        }
        if systems.iter().all(|s| s.is_good()) {
            return answer;
        }
        for sys in &mut systems {
            sys.roll();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)\n"
            ),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_a(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)\n"
            ),
            6
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 24253);
    }

    #[test]
    fn example3() {
        assert_eq!(
            super::part_b(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 12357789728873);
    }
}
