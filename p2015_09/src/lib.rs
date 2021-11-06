use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

pub fn part_a(input: &str) -> usize {
    let mut distances = BTreeMap::new();
    let mut journeys = BinaryHeap::new();
    let mut all_cities = BTreeSet::new();

    for line in input.trim().split('\n') {
        let parts: Vec<&str> = line.split(' ').collect();
        let from = parts[0];
        let to = parts[2];
        let dis: usize = parts[4].parse().unwrap();

        let mut leg = BTreeSet::new();
        leg.insert(from);
        leg.insert(to);
        distances.insert(leg.clone(), dis);

        all_cities.insert(from);
        all_cities.insert(to);

        journeys.push((Reverse(dis), leg.clone(), from));
        journeys.push((Reverse(dis), leg, to));
    }

    while let Some((dis, cities, from)) = journeys.pop() {
        let mut d = all_cities.difference(&cities).peekable();
        if d.peek().is_none() {
            return dis.0;
        }

        for other_city in d {
            let mut leg = BTreeSet::new();
            leg.insert(from);
            leg.insert(other_city);

            let dis = Reverse(dis.0 + distances.get(&leg).unwrap());
            let mut cities = cities.clone();
            cities.insert(other_city);
            journeys.push((dis, cities, other_city));
        }
    }

    panic!();
}

pub fn part_b(input: &str) -> usize {
    let mut distances = BTreeMap::new();
    let mut journeys = BinaryHeap::new();
    let mut all_cities = BTreeSet::new();

    for line in input.trim().split('\n') {
        let parts: Vec<&str> = line.split(' ').collect();
        let from = parts[0];
        let to = parts[2];
        let dis: usize = parts[4].parse().unwrap();

        let mut leg = BTreeSet::new();
        leg.insert(from);
        leg.insert(to);
        distances.insert(leg.clone(), dis);

        all_cities.insert(from);
        all_cities.insert(to);

        journeys.push((dis, leg.clone(), from));
        journeys.push((dis, leg, to));
    }

    let mut worst = 0;

    while let Some((dis, cities, from)) = journeys.pop() {
        let mut d = all_cities.difference(&cities).peekable();
        if d.peek().is_none() {
            worst = worst.max(dis);
        }

        for other_city in d {
            let mut leg = BTreeSet::new();
            leg.insert(from);
            leg.insert(other_city);

            let dis = dis + distances.get(&leg).unwrap();
            let mut cities = cities.clone();
            cities.insert(other_city);
            journeys.push((dis, cities, other_city));
        }
    }

    worst
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 251);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 898);
    }
}
