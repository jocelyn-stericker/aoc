use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .fold(0, |sum, i| sum + i)
}

pub fn part_b(input: &str) -> i64 {
    let input = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .cycle();

    let mut seen_values = HashSet::new();
    let mut current_value = 0;
    seen_values.insert(current_value);

    for i in input {
        current_value += i;
        if seen_values.contains(&current_value) {
            return current_value;
        }
        seen_values.insert(current_value);
    }

    unreachable!("input is cycled");
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("+1\n+1\n+1\n"), 3);
        assert_eq!(super::part_a("+1\n+1\n+1\n\n"), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_a("+1\n+1\n-2\n"), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(super::part_a("-1\n-2\n-3\n"), -6);
    }

    #[test]
    fn example4() {
        assert_eq!(super::part_b("+1\n-2\n+3\n+1\n"), 2);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 408);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 55250);
    }
}
