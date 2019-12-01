pub fn part_a(input: &str) -> i64 {
    input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .fold(0, |sum, i| sum + (i / 3 - 2))
}

fn fuel(i: i64) -> i64 {
    if i <= 0 {
        0
    } else {
        let base_fuel = (i / 3 - 2).max(0);
        base_fuel + fuel(base_fuel)
    }
}

pub fn part_b(input: &str) -> i64 {
    input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .fold(0, |sum, i| sum + fuel(i))
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("12\n"), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("1969\n"), 966);
        assert_eq!(super::part_b("100756\n"), 50346);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 3252208);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 4875451);
    }
}
