use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

pub fn solve(input: &str, groups: i64) -> i64 {
    let mut packages = BTreeSet::new();
    let mut combinations = BTreeSet::new();
    let mut to_visit = BinaryHeap::new();

    for line in input.trim().split('\n') {
        packages.insert(line.parse().unwrap());
    }

    let sum: i64 = packages.iter().sum();
    assert!(sum % groups == 0);
    let group_size = sum / groups;
    to_visit.push((Reverse(0), Reverse(1), 0, BTreeSet::new()));

    eprintln!("{} {}", sum, group_size);

    while let Some((num, prod, partial, included)) = to_visit.pop() {
        if partial == group_size {
            eprintln!("{:?}", &included);
            return prod.0;
        }
        for next in packages.difference(&included) {
            if partial + *next <= group_size {
                let mut included = included.clone();
                included.insert(*next);
                if !combinations.contains(&included) {
                    combinations.insert(included.clone());
                    to_visit.push((
                        Reverse(included.len()),
                        Reverse(included.iter().product::<i64>()),
                        partial + *next,
                        included,
                    ));
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a() {
        // 99706987 is too low
        assert_eq!(super::solve(include_str!("input.txt"), 3), 11266889531);
    }

    #[test]
    fn part_b() {
        // 99706987 is too low
        assert_eq!(super::solve(include_str!("input.txt"), 4), 77387711);
    }
}
