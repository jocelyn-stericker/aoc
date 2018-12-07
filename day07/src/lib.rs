use regex::Regex;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

struct State {
    deps: HashMap<char, HashSet<char>>, // .0 depends on items in .1
    deps_inv: HashMap<char, HashSet<char>>, // .0 is a dependency of items in .1
    zero_deps: BinaryHeap<(i32, char)>, // items without dependencies
}

impl State {
    fn new(input: &str) -> State {
        let line_re = Regex::new(r"^Step (?P<A>[A-Z]) .* before step (?P<B>[A-Z])").unwrap();

        let mut deps: HashMap<char, HashSet<char>> = HashMap::new();
        let mut deps_inv: HashMap<char, HashSet<char>> = HashMap::new();
        let mut high_pri: HashSet<char> = HashSet::new();
        for l in input.split('\n').filter(|line| line != &"") {
            let l = line_re.captures(l).expect("Invalid line");
            let (a, b) = (l.name("A").unwrap().as_str(), l.name("B").unwrap().as_str());
            let (a, b) = (a.chars().next().unwrap(), b.chars().next().unwrap());
            deps.entry(a).or_insert(HashSet::new()).insert(b);
            deps_inv.entry(b).or_insert(HashSet::new()).insert(a);
            high_pri.insert(a);
        }

        let zero_deps: BinaryHeap<(i32, char)> = high_pri
            .iter()
            .collect::<HashSet<&char>>()
            .difference(&deps_inv.keys().collect())
            .map(|s| (-(**s as i32), **s)) // heaps are max heaps, so we put a negative number in front
            .collect();

        State {
            deps,
            deps_inv,
            zero_deps,
        }
    }

    fn process(&mut self, d: char) {
        for freed in self.deps.remove(&d).unwrap_or(HashSet::new()) {
            let freed_deps = self.deps_inv.get_mut(&freed).unwrap();
            freed_deps.remove(&d);
            if freed_deps.is_empty() {
                self.zero_deps.push((-(freed as i32), freed));
                self.deps_inv.remove(&freed);
            }
        }
    }
}

pub fn part_a(input: &str) -> String {
    let mut state = State::new(input);

    let mut solution: Vec<char> = vec![];

    while !state.zero_deps.is_empty() {
        let d = state.zero_deps.pop().unwrap();
        solution.push(d.1);
        state.process(d.1);
    }

    solution.iter().collect()
}

pub fn part_b(input: &str, delay: u32, num_workers: u32) -> u32 {
    let mut state = State::new(input);
    let mut workers: Vec<(u32, Option<char>)> = vec![];
    for _ in 0..num_workers {
        workers.push((0, None))
    }

    let mut time = 0;

    while !state.zero_deps.is_empty() || workers.iter().any(|w| w.0 > 0) {
        time += 1;

        // See if any workers finished
        for worker in &mut workers {
            match worker.0 {
                0 => {}
                1 => {
                    worker.0 = 0;
                    let d = worker.1.take().unwrap();
                    state.process(d);
                }
                _ => {
                    worker.0 -= 1;
                }
            }
        }

        // If any workers are free, give them work.
        for worker in &mut workers {
            if !state.zero_deps.is_empty() {
                if worker.0 == 0 {
                    let d = state.zero_deps.pop().unwrap();
                    worker.0 = delay + (d.1 as u32) - 65 + 1;
                    worker.1 = Some(d.1);
                }
            }
        }
    }

    time - 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a(include_str!("sample.txt")), "CABDFE");
    }

    #[test]
    fn part_a() {
        assert_eq!(
            super::part_a(include_str!("input.txt")),
            "LFMNJRTQVZCHIABKPXYEUGWDSO"
        );
    }

    #[test]
    fn ascii_assumptions() {
        assert_eq!(('A' as i32) - 65, 0)
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b(include_str!("sample.txt"), 0, 2), 15);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt"), 60, 5), 1180);
    }
}
