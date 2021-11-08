use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

pub fn part_a(input: &str) -> usize {
    let mut replace = Vec::new();

    let mut lines = input.trim().split('\n');
    loop {
        let line = lines.next().unwrap();

        if line.is_empty() {
            break;
        }

        let mut parts = line.split(" => ");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        replace.push((from, to));
    }

    let code = lines.next().unwrap();

    let mut codes = BTreeSet::new();
    for i in 0..code.len() {
        for (from, to) in &replace {
            if code[i..].starts_with(from) {
                codes.insert(format!("{}{}{}", &code[0..i], to, &code[i + from.len()..]));
            }
        }
    }

    codes.len()
}

pub fn part_b(input: &str) -> usize {
    let mut replace = Vec::new();

    let mut lines = input.trim().split('\n');
    loop {
        let line = lines.next().unwrap();

        if line.is_empty() {
            break;
        }

        let mut parts = line.split(" => ");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        replace.push((to, from));
    }

    let goal = lines.next().unwrap();

    let mut codes = BTreeSet::new();
    codes.insert(goal.to_string());

    let mut to_visit: BinaryHeap<(Reverse<usize>, Reverse<usize>, String)> = BinaryHeap::new();
    to_visit.push((Reverse(0), Reverse(0), goal.to_string()));

    while let Some((_, steps, code)) = to_visit.pop() {
        if code == "e" {
            return steps.0;
        }

        for i in 0..code.len() {
            for (from, to) in &replace {
                if code[i..].starts_with(from) {
                    let code = format!("{}{}{}", &code[0..i], to, &code[i + from.len()..]);
                    if codes.insert(code.clone()) {
                        to_visit.push((Reverse(code.len()), Reverse(steps.0 + 1), code));
                    }
                }
            }
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 535);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 212);
    }
}
