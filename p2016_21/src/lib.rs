pub fn part_a(input: &str, orig: &str) -> String {
    let mut pw: Vec<_> = orig.chars().collect();

    for line in input.trim().split('\n') {
        let words: Vec<_> = line.split(' ').collect();
        match (words[0], words[1]) {
            ("swap", "position") => {
                let i = words[2].parse::<usize>().unwrap();
                let j = words[5].parse::<usize>().unwrap();
                pw.swap(i, j);
            }
            ("swap", "letter") => {
                let a = words[2].parse::<char>().unwrap();
                let b = words[5].parse::<char>().unwrap();

                let i = pw.iter().position(|c| *c == a).unwrap();
                let j = pw.iter().position(|c| *c == b).unwrap();
                pw.swap(i, j);
            }
            ("rotate", "left") => {
                pw.rotate_left(words[2].parse::<usize>().unwrap());
            }
            ("rotate", "right") => {
                pw.rotate_right(words[2].parse::<usize>().unwrap());
            }
            ("rotate", "based") => {
                let a = words[6].parse::<char>().unwrap();
                let i = pw.iter().position(|c| *c == a).unwrap();

                let j = 1 + i + {
                    if i >= 4 {
                        1
                    } else {
                        0
                    }
                };
                let l = pw.len();
                pw.rotate_right(j % l);
            }
            ("reverse", "positions") => {
                let i = words[2].parse::<usize>().unwrap();
                let j = words[4].parse::<usize>().unwrap();
                pw[i..=j].reverse();
            }
            ("move", "position") => {
                let i = words[2].parse::<usize>().unwrap();
                let j = words[5].parse::<usize>().unwrap();
                let c = pw.remove(i);
                pw.insert(j, c);
            }
            _ => {
                panic!()
            }
        }
        //
    }
    pw.into_iter().collect()
}

pub fn part_b(input: &str) -> String {
    let orig_pw = String::from("fbgdceah");

    let mut pw: Vec<_> = "fbgdceah".chars().collect();
    let inst: Vec<_> = input.trim().split('\n').collect();

    for (inst_count, line) in input.trim().split('\n').rev().enumerate() {
        let words: Vec<_> = line.split(' ').collect();
        match (words[0], words[1]) {
            ("swap", "position") => {
                let i = words[2].parse::<usize>().unwrap();
                let j = words[5].parse::<usize>().unwrap();
                pw.swap(i, j);
            }
            ("swap", "letter") => {
                let a = words[2].parse::<char>().unwrap();
                let b = words[5].parse::<char>().unwrap();

                let i = pw.iter().position(|c| *c == a).unwrap();
                let j = pw.iter().position(|c| *c == b).unwrap();
                pw.swap(i, j);
            }
            ("rotate", "left") => {
                pw.rotate_right(words[2].parse::<usize>().unwrap());
            }
            ("rotate", "right") => {
                pw.rotate_left(words[2].parse::<usize>().unwrap());
            }
            ("rotate", "based") => {
                for t in 0..8 {
                    let mut pw_test = pw.clone();
                    pw_test.rotate_right(t);

                    if orig_pw
                        == part_a(
                            &inst[inst.len() - inst_count - 1..].join("\n"),
                            &pw_test.iter().copied().collect::<String>(),
                        )
                    {
                        pw.rotate_right(t);
                        break;
                    }
                }
            }
            ("reverse", "positions") => {
                let i = words[2].parse::<usize>().unwrap();
                let j = words[4].parse::<usize>().unwrap();
                pw[i..=j].reverse();
            }
            ("move", "position") => {
                // 012345
                // abcdef
                // bcdaef
                let i = words[2].parse::<usize>().unwrap();
                let j = words[5].parse::<usize>().unwrap();
                let c = pw.remove(j);
                pw.insert(i, c);
            }
            _ => {
                panic!()
            }
        }

        if orig_pw
            != part_a(
                &inst[inst.len() - inst_count - 1..].join("\n"),
                &pw.iter().copied().collect::<String>(),
            )
        {
            panic!();
        }
    }
    pw.into_iter().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(
            super::part_a(include_str!("input.txt"), "abcdefgh"),
            String::from("agcebfdh")
        );
    }

    #[test]
    fn part_b() {
        // not baedhgcf
        assert_eq!(
            super::part_b(include_str!("input.txt")),
            String::from("afhdbegc")
        );
    }
}
