use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_a(input: &str) -> i64 {
    let mut parts = input.trim().split("\n\n");

    let mut p1: VecDeque<_> = parts
        .next()
        .unwrap()
        .split('\n')
        .skip(1)
        .map(|p| p.parse::<i64>().unwrap())
        .collect();
    let mut p2: VecDeque<_> = parts
        .next()
        .unwrap()
        .split('\n')
        .skip(1)
        .map(|p| p.parse::<i64>().unwrap())
        .collect();

    while let (Some(c1), Some(c2)) = (p1.front().copied(), p2.front().copied()) {
        p1.pop_front();
        p2.pop_front();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            assert!(c1 != c2);
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    p1.into_iter()
        .rev()
        .enumerate()
        .map(|(i, s)| (i as i64 + 1) * s)
        .sum::<i64>()
        + (p2
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, s)| (i as i64 + 1) * s)
            .sum::<i64>())
}

fn play(mut p1: VecDeque<i64>, mut p2: VecDeque<i64>) -> (i64, i64) {
    let mut states = HashSet::new();
    while let (Some(c1), Some(c2)) = (p1.front().copied(), p2.front().copied()) {
        let x = (p1, p2);
        if states.contains(&x) {
            return (1, 0);
        }
        states.insert(x.clone());
        p1 = x.0;
        p2 = x.1;

        p1.pop_front();
        p2.pop_front();

        let mut winner1 = c1 > c2;

        if ((c1 as usize) <= p1.len()) && ((c2 as usize) <= p2.len()) {
            let d1 = p1.iter().take(c1 as usize).copied().collect();
            let d2 = p2.iter().take(c2 as usize).copied().collect();
            let (a, b) = play(d1, d2);

            winner1 = a > b;
        }

        if winner1 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            assert!(c1 != c2);
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    (
        p1.into_iter()
            .rev()
            .enumerate()
            .map(|(i, s)| (i as i64 + 1) * s)
            .sum::<i64>(),
        (p2.into_iter()
            .rev()
            .enumerate()
            .map(|(i, s)| (i as i64 + 1) * s)
            .sum::<i64>()),
    )
}

pub fn part_b(input: &str) -> i64 {
    let mut parts = input.trim().split("\n\n");

    let p1: VecDeque<_> = parts
        .next()
        .unwrap()
        .split('\n')
        .skip(1)
        .map(|p| p.parse::<i64>().unwrap())
        .collect();
    let p2: VecDeque<_> = parts
        .next()
        .unwrap()
        .split('\n')
        .skip(1)
        .map(|p| p.parse::<i64>().unwrap())
        .collect();

    let x = play(p1, p2);

    x.0 + x.1
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 306);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 291);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 32162);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 32534);
    }
}
