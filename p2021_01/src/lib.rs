pub fn part_a(input: &str) -> i64 {
    let c: Vec<i64> = input
        .trim()
        .split('\n')
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut increases = 0;
    for window in c.windows(2) {
        if window[1] > window[0] {
            increases += 1;
        }
    }
    increases
}

pub fn part_b(input: &str) -> i64 {
    let c: Vec<i64> = input
        .trim()
        .split('\n')
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut increases = 0;
    for window in c.windows(4) {
        if window[3] > window[0] {
            increases += 1;
        }
    }
    increases
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1400);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1429);
    }
}
