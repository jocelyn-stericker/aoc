// use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.trim().split('\n') {
        let n: Vec<i64> = line.split("x").map(|n| n.parse().unwrap()).collect();
        let x = n[0];
        let y = n[1];
        let z = n[2];
        sum += (x * y + y * z + z * x) * 2 + (x * y).min(y * z).min(z * x);
    }

    sum
}

pub fn part_b(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.trim().split('\n') {
        let n: Vec<i64> = line.split("x").map(|n| n.parse().unwrap()).collect();
        let x = n[0];
        let y = n[1];
        let z = n[2];
        sum += (2 * x + 2 * y).min(2 * y + 2 * z).min(2 * z + 2 * x) + x * y * z
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("2x3x4\n"), 58);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("2x3x4\n"), 34);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1598415);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 3812909);
    }
}
