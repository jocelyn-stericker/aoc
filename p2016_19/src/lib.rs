use std::collections::BTreeMap;

pub fn part_a(input: &str) -> usize {
    let elves: usize = input.trim().parse().unwrap();
    let mut left = BTreeMap::new();

    for i in 1..elves {
        left.insert(i, i + 1);
    }
    left.insert(elves, 1);

    let mut i = 1;
    loop {
        if left.len() == 1 {
            return *left.values().next().unwrap();
        }
        let to_remove = left[&i];
        let left_left = left[&to_remove];

        left.insert(i, left_left);

        left.remove(&to_remove);

        i = left_left;
    }
}

pub fn part_b(input: &str) -> i64 {
    let elves: i64 = input.trim().parse().unwrap();
    let mut left = BTreeMap::new();
    let mut right = BTreeMap::new();

    for i in 1..elves {
        left.insert(i, i + 1);
        right.insert(i + 1, i);
    }
    left.insert(elves, 1);
    right.insert(1, elves);

    let mut across = 1;
    let mut across_count = (left.len() / 2) as i64;
    for _ in 0..across_count {
        across = left[&across];
    }

    let mut i = 1;
    loop {
        if left.len() == 1 {
            return *left.values().next().unwrap();
        }

        let skips: i64 = (left.len() / 2) as i64;

        match skips - across_count {
            0 => {}
            1 => {
                across = left[&across];
            }
            -1 => {
                across = right[&across];
            }
            _ => {
                panic!()
            }
        }
        across_count = skips;

        let before = right[&across];
        let after = left[&across];

        left.insert(before, after);
        right.insert(after, before);

        left.remove(&across);
        right.remove(&across);

        across = after;
        across_count -= 1;

        i = left[&i];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("5\n"), 3);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1841611);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("5\n"), 2);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1423634);
    }
}
