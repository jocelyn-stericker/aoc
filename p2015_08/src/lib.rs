use std::collections::VecDeque;

// use std::collections::HashSet;

enum EscapedState {
    None,
    Quote,
    Hex(isize),
}

pub fn part_a(input: &str) -> usize {
    let mut count = 0;
    let mut count_b = 0;
    for line in input.trim().split('\n') {
        let mut chars: VecDeque<char> = line.chars().collect();
        count_b += chars.len();

        chars.pop_back();
        chars.pop_front();

        let mut escaped_state = EscapedState::None;
        for c in &chars {
            match escaped_state {
                EscapedState::None => match c {
                    '\\' => {
                        escaped_state = EscapedState::Quote;
                        count += 1;
                    }
                    _ => {
                        count += 1;
                    }
                },
                EscapedState::Quote => match c {
                    '\\' | '"' => {
                        escaped_state = EscapedState::None;
                    }
                    'x' => {
                        escaped_state = EscapedState::Hex(2);
                    }
                    _ => {
                        eprintln!("{} {}", line, c);
                        panic!();
                    }
                },
                EscapedState::Hex(i) => {
                    if i == 1 {
                        escaped_state = EscapedState::None;
                    } else {
                        escaped_state = EscapedState::Hex(i - 1);
                    }
                }
            }
        }
    }
    count_b - count
}

pub fn part_b(input: &str) -> usize {
    let mut count = 0;
    let mut count_b = 0;
    for line in input.trim().split('\n') {
        let chars: Vec<char> = line.chars().collect();
        count_b += chars.len();

        count += 2;

        for c in &chars {
            match c {
                '"' | '\\' => {
                    count += 2;
                }
                _ => {
                    count += 1;
                }
            }
        }
    }
    count - count_b
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        let x = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        assert_eq!(super::part_b(x), 19);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1371);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2117);
    }
}
