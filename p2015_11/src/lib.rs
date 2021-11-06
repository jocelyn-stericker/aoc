// use std::collections::HashSet;

fn increment(input: &mut Vec<char>) {
    for c in input.iter_mut().rev() {
        if *c == 'z' {
            *c = 'a';
        } else {
            *c = std::char::from_u32(*c as u32 + 1).unwrap();
            return;
        }
    }
}

fn valid(input: &[char]) -> bool {
    let mut inc_state = 3;
    let mut inc_next: Option<char> = None;
    let mut dup_state = 0;
    let mut dup_expect = None;
    let mut dup_count = 0;

    for c in input {
        if inc_state > 0 {
            if Some(*c) == inc_next {
                inc_state -= 1;
            } else {
                inc_state = 2;
            }
        }
        inc_next = std::char::from_u32(*c as u32 + 1);

        if Some(*c) == dup_expect {
            dup_state += 1;
            if dup_state % 2 == 1 {
                dup_count += 1;
            }
        } else {
            dup_state = 0;
        }
        dup_expect = Some(*c);
    }

    inc_state == 0 && dup_count >= 2
}

pub fn part_a(input: &str) -> String {
    let mut pw: Vec<char> = input.trim().chars().collect();
    increment(&mut pw);
    while !valid(&pw) {
        increment(&mut pw);
    }

    pw.into_iter().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("abcdefgh\n"), String::from("abcdffaa"));
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), String::from(""));
    }
}
