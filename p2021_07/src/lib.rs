pub fn part_a(input: &str) -> i64 {
    let crabs: Vec<i64> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut best = i64::MAX;

    for meeting_point in min..=max {
        let mut score = 0;
        for crab in &crabs {
            score += (crab - meeting_point).abs();
        }

        best = best.min(score);
    }

    best
}

pub fn part_b(input: &str) -> i64 {
    let crabs: Vec<i64> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut best = i64::MAX;

    for meeting_point in min..=max {
        let mut score = 0;
        for crab in &crabs {
            let delta = (crab - meeting_point).abs();
            score += delta * (delta + 1) / 2;
        }

        best = best.min(score);
    }

    best
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 325528);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 85015836);
    }
}
