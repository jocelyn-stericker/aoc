use std::collections::HashMap;

pub fn count(
    p1: char,
    p2: char,
    reactions: &HashMap<(char, char), char>,
    memo: &mut HashMap<(char, char, usize), HashMap<char, usize>>,
    iterations: usize,
) -> HashMap<char, usize> {
    if iterations == 0 {
        let mut counts = HashMap::new();
        *counts.entry(p1).or_default() += 1;
        *counts.entry(p2).or_default() += 1;
        return counts;
    }

    if let Some(result) = memo.get(&(p1, p2, iterations)) {
        return result.clone();
    }

    if let Some(c) = reactions.get(&(p1, p2)) {
        let mut counts = HashMap::new();
        for (c, count) in count(p1, *c, reactions, memo, iterations - 1) {
            *counts.entry(c).or_default() += count;
        }
        for (c, count) in count(*c, p2, reactions, memo, iterations - 1) {
            *counts.entry(c).or_default() += count;
        }
        *counts.entry(*c).or_default() -= 1;
        memo.insert((p1, p2, iterations), counts.clone());
        counts
    } else {
        let mut counts = HashMap::new();
        *counts.entry(p1).or_default() += 1;
        *counts.entry(p2).or_default() += 1;
        counts
    }
}

pub fn solve(input: &str, iter: usize) -> usize {
    let mut lines = input.trim().split('\n');
    let template: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut reactions = HashMap::new();

    for line in lines {
        let (pattern, addition) = line.split_once(" -> ").unwrap();
        let mut pattern = pattern.chars();
        let p1 = pattern.next().unwrap();
        let p2 = pattern.next().unwrap();
        let addition = addition.chars().next().unwrap();
        reactions.insert((p1, p2), addition);
    }

    let mut memo: HashMap<(char, char, usize), HashMap<char, usize>> = HashMap::new();
    let mut counts: HashMap<char, usize> = HashMap::new();

    for i in 1..template.len() {
        let p1 = template[i - 1];
        let p2 = template[i];
        for (c, count) in count(p1, p2, &reactions, &mut memo, iter) {
            *counts.entry(c).or_default() += count;
        }

        if i != 1 {
            *counts.entry(p1).or_default() -= 1;
        }
    }

    let most_common = counts.values().max().unwrap();
    let least_common = counts.values().min().unwrap();
    most_common - least_common
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::solve(
                "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
                10
            ),
            1588
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::solve(include_str!("input.txt"), 10), 2851);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::solve(
                "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
                40
            ),
            2188189693529
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::solve(include_str!("input.txt"), 40), 10002813279337);
    }
}
