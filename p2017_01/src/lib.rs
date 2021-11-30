pub fn part_a(input: &str) -> i64 {
    let mut chars: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    chars.push(chars[0]);
    let mut sum: i64 = 0;
    for i in 0..chars.len() - 1 {
        let curr = chars[i];
        let next = chars[i + 1];
        if curr == next {
            sum += curr as i64;
        }
    }
    sum
}

pub fn part_b(input: &str) -> i64 {
    let chars: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut sum: i64 = 0;
    for i in 0..chars.len() - 1 {
        let curr = chars[i];
        let next = chars[(i + chars.len() / 2) % chars.len()];
        if curr == next {
            sum += curr as i64;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1171);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1024);
    }
}
