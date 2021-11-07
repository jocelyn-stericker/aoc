use std::collections::{HashMap, HashSet};

fn best<'a>(
    remaining: &HashSet<&'a str>,
    first: &'a str,
    last: &'a str,
    deltas: &HashMap<(&'a str, &'a str), i64>,
) -> i64 {
    let mut best_so_far = i64::MIN;

    for next in remaining.iter() {
        let mut remaining = remaining.clone();
        remaining.remove(next);
        if !remaining.is_empty() {
            best_so_far = best_so_far.max(
                best(&remaining, first, next, deltas)
                    + deltas.get(&(last, next)).unwrap()
                    + deltas.get(&(next, last)).unwrap(),
            );
        } else {
            best_so_far = best_so_far.max(
                deltas.get(&(last, next)).unwrap()
                    + deltas.get(&(next, last)).unwrap()
                    + deltas.get(&(first, next)).unwrap()
                    + deltas.get(&(next, first)).unwrap(),
            );
        }
    }

    best_so_far
}

pub fn part_a(input: &str) -> i64 {
    let mut deltas = HashMap::new();
    let mut all = HashSet::new();

    for line in input.trim().split('\n') {
        let parts: Vec<_> = line.split(' ').collect();
        let one = parts[0];
        let two = parts[10];
        let mut diff: i64 = parts[3].parse().unwrap();
        match parts[2] {
            "lose" => {
                diff *= -1;
            }
            "gain" => {}
            _ => {
                panic!();
            }
        }

        deltas.insert((one, two), diff);
        all.insert(one);
        all.insert(two);
    }

    let mut best_so_far = i64::MIN;
    for first in all.iter() {
        let mut remaining = all.clone();
        remaining.remove(first);
        best_so_far = best_so_far.max(best(&remaining, first, first, &deltas));
    }

    best_so_far
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 618);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_a(include_str!("input_b.txt")), 601);
    }
}
