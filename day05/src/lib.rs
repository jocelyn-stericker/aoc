fn swap_case(c: char) -> char {
    let v: Vec<char> = {
        if c.is_uppercase() {
            c.to_lowercase().collect()
        } else {
            c.to_uppercase().collect()
        }
    };

    v[0]
}

pub fn part_a(input: &str) -> usize {
    let s: String = input
        .chars()
        .fold(Vec::new(), |mut string, c| {
            if c.is_whitespace() {
                // Do nothing
            } else if string.len() > 0 && string[string.len() - 1] == swap_case(c) {
                string.pop();
            } else {
                string.push(c);
            }

            string
        })
        .iter()
        .collect();

    s.len()
}

pub fn part_b(input: &str) -> usize {
    "qwertyuiopasdfghjklzxcvbnm"
        .chars()
        .map(|c_to_remove| {
            (
                c_to_remove,
                part_a(
                    &input
                        .replace(c_to_remove, "")
                        .replace(swap_case(c_to_remove), ""),
                ),
            )
        })
        .min_by_key(|r| r.1)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn part_a() {
        assert_ne!(super::part_a(include_str!("input.txt")), 10764);
        assert_eq!(super::part_a(include_str!("input.txt")), 10762);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("dabAcCaCBAcCcaDA"), 4);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 6946);
    }
}
