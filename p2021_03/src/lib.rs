pub fn part_a(input: &str) -> i64 {
    let lines: Vec<_> = input.trim().split('\n').collect();
    let mut ones = vec![0usize; lines[0].len()];

    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for d in 0usize..lines[0].len() {
        let ones = ones[d];
        let zeros = lines.len() - ones;
        if ones >= zeros {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = i64::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i64::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

pub fn rating(input: &str, most_common: bool) -> i64 {
    let mut lines: Vec<_> = input.trim().split('\n').collect();
    for d in 0usize..lines[0].len() {
        let mut ones = 0;
        let line_count = lines.len();
        for line in &lines {
            let c = line.chars().nth(d).unwrap();
            if c == '1' {
                ones += 1;
            }
        }

        let zeros = line_count - ones;

        let to_check = if most_common {
            if ones >= zeros {
                '1'
            } else {
                '0'
            }
        } else if ones >= zeros {
            '0'
        } else {
            '1'
        };

        lines = lines
            .into_iter()
            .filter(|s| s.chars().collect::<Vec<_>>()[d] == to_check)
            .collect();

        if lines.len() == 1 {
            return i64::from_str_radix(lines[0], 2).unwrap();
        }
    }

    panic!();
}

pub fn part_b(input: &str) -> i64 {
    rating(input, true) * rating(input, false)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_b(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            ),
            230
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1082324);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1353024);
    }
}
