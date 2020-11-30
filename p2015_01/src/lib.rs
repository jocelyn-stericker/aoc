// use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    let mut l = 0;
    for c in input.trim().split('\n').next().unwrap().chars() {
        match c {
            '(' => l += 1,
            ')' => l -= 1,
            _ => panic!(),
        }
    }
    l
}

pub fn part_b(input: &str) -> i64 {
    let mut l = 0;
    for (i, c) in input.trim().split('\n').next().unwrap().chars().enumerate() {
        match c {
            '(' => l += 1,
            ')' => l -= 1,
            _ => panic!(),
        }

        if l == -1 {
            return (i as i64) + 1;
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("\n"), 0);
        assert_eq!(super::part_b("()())\n"), 5);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 232);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1783);
    }
}
