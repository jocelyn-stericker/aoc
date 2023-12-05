pub fn part_a(input: &str) -> u32 {
    let mut memo = 0;
    for line in input.trim().split('\n') {
        let mut first = None;
        let mut last = None;
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                if first.is_none() {
                    first = Some(digit);
                }
                last = Some(digit);
            }
        }
        memo += first.unwrap() * 10 + last.unwrap();
    }
    memo
}

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_b(input: &str) -> u32 {
    let mut memo = 0;
    for line in input.trim().split('\n') {
        let mut first = None;
        let mut last = None;
        let mut word = String::new();
        for char in line.chars() {
            word.push(char);
            if let Some(digit) = char.to_digit(10) {
                if first.is_none() {
                    first = Some(digit);
                }
                last = Some(digit);
            } else if let Some(digit) = DIGITS.iter().position(|&s| word.ends_with(s)) {
                if first.is_none() {
                    first = Some(digit as u32);
                }
                last = Some(digit as u32);
            }
        }
        memo += first.unwrap() * 10 + last.unwrap();
    }
    memo
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 56049);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 54530);
    }
}
