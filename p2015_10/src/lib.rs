// use std::collections::HashSet;

fn look_say(c: Vec<u8>) -> Vec<u8> {
    let mut s = Vec::new();

    let mut prev = None;
    let mut run: u8 = 0;
    for c in c {
        if Some(c) == prev {
            run += 1;
        } else {
            if let Some(other) = prev {
                s.push(run);
                s.push(other);
            }

            run = 1;
            prev = Some(c);
        }
    }

    s.push(run);
    s.push(prev.unwrap());

    s
}

pub fn part_a(input: &str) -> usize {
    let mut chars: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    for _ in 0..40 {
        chars = look_say(chars);
    }

    chars.len()
}

pub fn part_b(input: &str) -> usize {
    let mut chars: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    for _ in 0..50 {
        chars = look_say(chars);
    }

    chars.len()
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("111221\n"), 6);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 492982);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 6989950);
    }
}
