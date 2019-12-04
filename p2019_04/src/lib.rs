pub fn meets_criteria1(i: i64) -> bool {
    let s = i.to_string();
    let mut rule1_matched = false;
    for (ca, cb) in s.chars().zip(s.chars().skip(1)) {
        if ca == cb {
            rule1_matched = true;
        }
        if cb.to_string().parse::<i64>().unwrap() < ca.to_string().parse::<i64>().unwrap() {
            return false;
        }
    }

    rule1_matched
}

pub fn meets_criteria2(i: i64) -> bool {
    let mut prev_c = None;
    let mut count = 0;
    for c in i.to_string().chars() {
        if Some(c) == prev_c {
            count += 1;
        } else if count == 2 {
            return true;
        } else {
            count = 1;
        }
        prev_c = Some(c);
    }

    count == 2
}

pub fn part_a(input: &str) -> i64 {
    let range: Vec<_> = input
        .split('-')
        .filter(|line| line != &"")
        .map(|line| line.trim().parse::<i64>().expect("Invalid number"))
        .collect();

    let mut count = 0;
    for i in range[0]..range[1] {
        if meets_criteria1(i) {
            count += 1;
        }
    }
    count
}

pub fn part_b(input: &str) -> i64 {
    let range: Vec<_> = input
        .split('-')
        .filter(|line| line != &"")
        .map(|line| line.trim().parse::<i64>().expect("Invalid number"))
        .collect();

    let mut count = 0;
    for i in range[0]..range[1] {
        if meets_criteria1(i) && meets_criteria2(i) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::meets_criteria1(111111), true);
        assert_eq!(super::meets_criteria1(223450), false);
        assert_eq!(super::meets_criteria1(123789), false);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1154);
    }

    #[test]
    fn example2() {
        assert_eq!(super::meets_criteria2(112233), true);
        assert_eq!(super::meets_criteria2(123444), false);
        assert_eq!(super::meets_criteria2(111122), true);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 750);
    }
}
