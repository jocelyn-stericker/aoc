use std::collections::HashMap;
use std::collections::HashSet;

struct Counts {
    two: u64,
    three: u64,
}

pub fn part_a(input: &str) -> u64 {
    let counts = input.split('\n').filter(|line| line != &"").fold(
        Counts { two: 0, three: 0 },
        |counts, word: &str| {
            let mut char_counts: HashMap<char, u64> = HashMap::new();
            for c in word.chars() {
                *char_counts.entry(c).or_insert(0) += 1;
            }
            let mut has_two = false;
            let mut has_three = false;
            for (_key, val) in char_counts {
                match val {
                    2 => {
                        has_two = true;
                    }
                    3 => {
                        has_three = true;
                    }
                    _ => {
                        // pass
                    }
                }
            }
            Counts {
                two: {
                    if has_two {
                        counts.two + 1
                    } else {
                        counts.two
                    }
                },
                three: {
                    if has_three {
                        counts.three + 1
                    } else {
                        counts.three
                    }
                },
            }
        },
    );

    counts.two * counts.three
}

pub fn part_b(input: &str) -> Option<String> {
    let mut potential_answers: HashSet<(usize, String)> = HashSet::new();
    for line in input.split('\n').filter(|line| line != &"") {
        for i in 0..line.len() {
            let potential_answer = format!("{}{}", &line[0..i], &line[i + 1..line.len()]);
            let potential_answer = (i, potential_answer);
            if potential_answers.contains(&potential_answer) {
                return Some(potential_answer.1);
            }
            potential_answers.insert(potential_answer);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"),
            12
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 7904);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"),
            Some("fgij".to_owned())
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(
            super::part_b(include_str!("input.txt")),
            Some("wugbihckpoymcpaxefotvdzns".to_owned())
        );
    }
}
