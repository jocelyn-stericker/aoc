// use std::collections::HashSet;

pub fn mess(a: &str) -> String {
    let b: String = a
        .chars()
        .rev()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => panic!(),
        })
        .collect();
    format!("{}0{}", a, b)
}

pub fn checksum(a: &str) -> String {
    let mut a: String = a.to_owned();
    while a.len() % 2 == 0 {
        let b: Vec<_> = a.chars().collect();
        a = b
            .chunks(2)
            .map(|t| {
                let x = t[0];
                let y = t[1];
                if x == y {
                    '1'
                } else {
                    '0'
                }
            })
            .collect()
    }

    a
}

pub fn part_a(input: &str, n: usize) -> String {
    let mut a = input.trim().to_owned();
    while a.len() < n {
        a = mess(&a);
    }

    a = a.chars().take(n).collect();
    dbg!(a.len());

    a = checksum(&a);

    dbg!(a.len());

    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn example0() {
        assert_eq!(super::mess("111100001010"), "1111000010100101011110000");
        assert_eq!(super::checksum("110010110100"), "100");
    }
    #[test]
    fn example1() {
        assert_eq!(super::part_a("10000\n", 20), "01100");
    }

    #[test]
    fn part_a() {
        // 01110011011000110 is too low
        assert_eq!(
            super::part_a(include_str!("input.txt"), 272),
            "10101001010100001"
        ); // 16:33
    }

    #[test]
    fn part_b() {
        assert_eq!(
            super::part_a(include_str!("input.txt"), 35651584),
            "10101001010100001"
        ); // 16:33
    }
}
