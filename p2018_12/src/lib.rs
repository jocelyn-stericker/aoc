use std::collections::HashMap;
use std::collections::HashSet;

#[inline]
fn pattern_matches(p: &str, state: &HashSet<i64>, idx: i64) -> bool {
    for (i, c) in p.chars().enumerate() {
        if state.contains(&(idx - 2 + (i as i64))) != (c == '#') {
            return false;
        }
    }

    true
}

fn sig(state: &HashSet<i64>, offset: i64) -> i64 {
    state.iter().fold(0, |m, i| m + i - offset)
}

fn normalized(state: &HashSet<i64>, offset: i64) -> HashSet<i64> {
    let mut n = HashSet::new();
    for v in state {
        n.insert(v - offset);
    }

    n
}

pub fn solve(input: &str, generations: usize, allow_jump: bool) -> i64 {
    let mut lines = input.split("\n").filter(|&line| line != "\n");
    let init_state_line = lines.next().unwrap()[15..].to_owned();

    let mut state: HashSet<i64> = HashSet::new();
    let mut bound_left: i64 = -5;
    let mut bound_right: i64 = init_state_line.len() as i64 + 5;

    for (i, c) in init_state_line.chars().enumerate() {
        match c {
            '#' => {
                state.insert(i as i64);
            }
            '.' => {
                // pass
            }
            _ => {
                panic!("Invalid char in initial state.");
            }
        }
    }

    lines.next(); // empty line

    let mut patterns_dead = Vec::new();
    let mut patterns_alive = Vec::new();

    let mut sigs: HashMap<i64, Vec<(HashSet<i64>, usize, i64)>> = HashMap::new();

    for l in lines {
        let pattern = l[0..=4].to_owned();
        let alive_after = &l[9..=9] == "#";
        if alive_after {
            if &pattern[2..=2] == "#" {
                patterns_alive.push(pattern);
            } else {
                patterns_dead.push(pattern);
            }
        }
    }

    let mut generation = 0;

    loop {
        let mut next_state = HashSet::new();
        let mut new_bound_left: i64 = *state.iter().next().unwrap();
        let mut new_bound_right: i64 = new_bound_left;
        for x in bound_left..=bound_right {
            let patterns = {
                if state.contains(&x) {
                    &patterns_alive
                } else {
                    &patterns_dead
                }
            };
            for p in patterns {
                if pattern_matches(&p, &state, x) {
                    new_bound_left = std::cmp::min(x - 5, new_bound_left);
                    new_bound_right = std::cmp::max(x + 5, new_bound_left);
                    next_state.insert(x);
                }
            }
        }

        bound_left = new_bound_left;
        bound_right = new_bound_right;
        std::mem::swap(&mut state, &mut next_state);

        if allow_jump {
            let norm_state = normalized(&state, bound_left);
            let s = sig(&state, bound_left);
            let potential_matches = sigs.entry(s).or_default();
            if let Some(m) = potential_matches.iter().find(|m| m.0 == norm_state) {
                let generations_per_jump = generation - m.1;
                let jumps = (generations - generation - 1) / generations_per_jump;
                let ff_generations = jumps * generations_per_jump;
                let offset_per_jump = bound_left - m.2;
                let ff_offset = (jumps as i64) * offset_per_jump;
                println!(
                    "Fast-forwarding {} generations by jumping {}",
                    ff_generations, ff_offset
                );

                generation += ff_generations;
                state = normalized(&state, -ff_offset);
            } else {
                potential_matches.push((norm_state, generation, bound_left));
            }
        }
        generation += 1;

        if generation >= generations {
            break;
        }
    }

    sig(&state, 0)
}

#[test]
fn test_sample() {
    assert_eq!(solve(include_str!("sample.txt"), 20, true), 325);
}

#[test]
fn test_part_a() {
    assert_eq!(solve(include_str!("input.txt"), 20, true), 2917);
}

#[test]
fn test_jump() {
    assert_eq!(solve(include_str!("input.txt"), 1000, false), 65956);
    assert_eq!(solve(include_str!("input.txt"), 1000, true), 65956);
}

#[test]
fn test_part_b() {
    assert_eq!(
        solve(include_str!("input.txt"), 50000000000, true),
        3250000000956
    );
}
