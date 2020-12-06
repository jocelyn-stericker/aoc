use std::collections::HashSet;

pub fn part_a(input: &str) -> usize {
    let mut sum = 0;
    for group in input.trim().split("\n\n") {
        let mut qs = HashSet::new();
        for c in group.chars() {
            if c != '\n' {
                qs.insert(c);
            }
        }
        sum += qs.len();
    }
    sum
}

pub fn part_b(input: &str) -> usize {
    let mut sum = 0;
    for group in input.trim().split("\n\n") {
        let mut all_qs: Option<HashSet<char>> = None;
        for line in group.trim().split('\n') {
            let mut qs = HashSet::new();
            for c in line.chars() {
                if c != '\n' {
                    qs.insert(c);
                }
            }
            all_qs = Some(
                all_qs
                    .map(|all_qs| all_qs.intersection(&qs).cloned().collect())
                    .unwrap_or(qs),
            );
        }
        sum += all_qs.unwrap().len();
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 6542);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 3299);
    }
}
