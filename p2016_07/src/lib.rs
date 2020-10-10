use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut count = 0;

    'a: for line in input.trim().lines() {
        let chars: Vec<_> = line.trim().chars().collect();
        let mut in_bracket = false;
        let mut matches = false;
        'b: for (i, c) in chars.iter().enumerate() {
            if *c == '[' {
                in_bracket = true;
                continue 'b;
            }
            if *c == ']' {
                in_bracket = false;
                continue 'b;
            }

            if chars.get(i) == chars.get(i + 3)
                && chars.get(i + 1) == chars.get(i + 2)
                && chars.get(i) != chars.get(i + 1)
            {
                if in_bracket {
                    continue 'a;
                }
                matches = true;
            }
        }

        if matches {
            count += 1;
        }
    }

    count
}

pub fn part_b(input: &str) -> i64 {
    let mut count = 0;

    'a: for line in input.trim().lines() {
        let chars: Vec<_> = line.trim().chars().collect();
        let mut in_bracket = false;
        let mut aba = HashMap::new();

        'b: for (i, c) in chars.iter().enumerate() {
            if *c == '[' {
                in_bracket = true;
                continue 'b;
            }
            if *c == ']' {
                in_bracket = false;
                continue 'b;
            }

            if chars.get(i) == chars.get(i + 2) && chars.get(i) != chars.get(i + 1) {
                let en = if in_bracket {
                    let x = format!("{}{}", chars[i + 1], chars[i]);
                    let en = aba.entry(x).or_insert((false, false));
                    en.1 = true;
                    en
                } else {
                    let x = format!("{}{}", chars[i], chars[i + 1]);
                    let en = aba.entry(x).or_insert((false, false));
                    en.0 = true;
                    en
                };
                if en.0 && en.1 {
                    count += 1;
                    continue 'a;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a("abba[mnop]qrst\nabcd[bddb]xyyx\naaaa[qwer]tyui\nioxxoj[asdfgh]zxcvbn\n"),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b("aba[bab]xyz\nxyx[xyx]xyx\naaa[kek]eke\nzazbz[bzb]cdb\n"),
            3
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 118); //7:03
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 260); //13:41
    }
}
