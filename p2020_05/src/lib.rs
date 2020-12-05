use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    let mut m = 0;
    for line in input.trim().split('\n') {
        let parts: Vec<_> = line.split(' ').collect();
        let mut min = 0;
        let mut max = 127;
        for c in parts[0].chars() {
            if c == 'B' {
                min = (min + max) / 2 + 1;
            } else {
                max = (min + max) / 2;
            }
        }

        let mut b_min = 0;
        let mut b_max = 7;
        for c in parts[1].chars() {
            if c == 'R' {
                b_min = (b_min + b_max) / 2 + 1;
            } else {
                b_max = (b_min + b_max) / 2;
            }
        }
        m = m.max(b_min + min * 8)
    }
    m
}

pub fn part_b(input: &str) -> i64 {
    let mut x_min = 0;
    let mut x_max = 0;
    let mut seats = HashSet::new();
    for line in input.trim().split('\n') {
        let parts: Vec<_> = line.split(' ').collect();
        let mut min = 0;
        let mut max = 127;
        for c in parts[0].chars() {
            if c == 'B' {
                min = (min + max) / 2 + 1;
            } else {
                max = (min + max) / 2;
            }
        }

        let mut b_min = 0;
        let mut b_max = 7;
        for c in parts[1].chars() {
            if c == 'R' {
                b_min = (b_min + b_max) / 2 + 1;
            } else {
                b_max = (b_min + b_max) / 2;
            }
        }
        x_min = x_min.min(b_min + min * 8);
        x_max = x_max.max(b_min + min * 8);

        seats.insert(b_min + min * 8);
    }

    for i in x_min..=x_max {
        if !seats.contains(&i) && seats.contains(&(i - 1)) && seats.contains(&(i + 1)) {
            return i;
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("BFFFBBF RRR\nFFFBBBF RRR\nBBFFBBF RLL"), 820);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 850);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 599);
    }
}
