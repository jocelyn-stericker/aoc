pub fn part_a(input: &str) -> String {
    let mut res = vec![];
    for i in 0.. {
        let digest: Vec<char> = format!(
            "{:x}",
            md5::compute(input.trim().to_owned() + &i.to_string())
        )
        .chars()
        .collect();
        if digest.starts_with(&['0', '0', '0', '0', '0']) {
            res.push(digest[5]);
            if res.len() == 8 {
                break;
            }
        }
    }
    res.iter().collect::<String>()
}

pub fn part_b(input: &str) -> String {
    let mut res = vec!['x', 'x', 'x', 'x', 'x', 'x', 'x', 'x'];
    for i in 0.. {
        let digest: Vec<char> = format!(
            "{:x}",
            md5::compute(input.trim().to_owned() + &i.to_string())
        )
        .chars()
        .collect();
        if digest.starts_with(&['0', '0', '0', '0', '0']) {
            if let Ok(x) = digest[5].to_string().parse::<usize>() {
                if res.get(x) == Some(&'x') {
                    res[x] = digest[6];
                }
            }
            if !res.iter().any(|x| *x == 'x') {
                break;
            }
        }
    }
    res.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("abc"), "18f47a30");
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), "2414bc77"); //15:19
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("abc"), "05ace8e3");
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), "437e60fc"); // 20:37
    }
}
