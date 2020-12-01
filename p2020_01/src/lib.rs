pub fn part_a(input: &str) -> i64 {
    let mut a = vec![];
    for c in input.trim().split('\n') {
        a.push(c.parse::<i64>().unwrap());
    }
    for (i, c) in a.iter().enumerate() {
        for b in a.iter().skip(i + 1) {
            if c + b == 2020 {
                return c * b;
            }
        }
    }
    0
}

pub fn part_b(input: &str) -> i64 {
    let mut a = vec![];
    for c in input.trim().split('\n') {
        a.push(c.parse::<i64>().unwrap());
    }
    for (i, c) in a.iter().enumerate() {
        for (j, b) in a.iter().enumerate().skip(i + 1) {
            for d in a.iter().skip(j + 1) {
                if d + c + b == 2020 {
                    return d * c * b;
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
        assert_eq!(super::part_a(include_str!("input.txt")), 445536);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 138688160);
    }
}
