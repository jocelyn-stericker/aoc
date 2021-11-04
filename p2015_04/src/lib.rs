pub fn part_a(input: &str) -> i64 {
    let key = input.trim().split('\n').next().unwrap();
    for i in 1.. {
        let inp = format!("{}{}", key, i);
        let res = format!("{:32x}", md5::compute(inp));
        if res.starts_with("00000") {
            return i;
        }
    }
    panic!();
}

pub fn part_b(input: &str) -> i64 {
    let key = input.trim().split('\n').next().unwrap();
    for i in 1.. {
        let inp = format!("{}{}", key, i);
        let res = format!("{:32x}", md5::compute(inp));
        if res.starts_with("000000") {
            return i;
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 346386);
    }
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 9958218);
    }
}
