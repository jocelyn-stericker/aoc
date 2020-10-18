// use std::collections::HashSet;
use fancy_regex::Regex;

fn has_triple(s: &str) -> Option<char> {
    let re = Regex::new(r"([0-9a-f])\1\1").unwrap();
    re.captures(s)
        .unwrap()
        .map(|c| c.get(1).unwrap().as_str().chars().next().unwrap())
}

pub fn stretch(mut digest: String) -> String {
    for _ in 0..2017 {
        digest = format!("{:x}", md5::compute(&digest)).chars().collect();
    }

    digest
}

pub fn part_a(input: &str) -> i64 {
    let mut matches = 0;
    for i in 0.. {
        let digest: String = format!(
            "{:x}",
            md5::compute(input.trim().to_owned() + &i.to_string())
        )
        .chars()
        .collect();

        if let Some(c) = has_triple(&digest) {
            let mut s = String::new();
            for _ in 0..5 {
                s.push(c);
            }
            for j in (i + 1)..=(i + 1000) {
                let subdigest: String = format!(
                    "{:x}",
                    md5::compute(input.trim().to_owned() + &j.to_string())
                )
                .chars()
                .collect();

                if subdigest.contains(&s) {
                    matches += 1;
                    if matches == 64 {
                        return i;
                    }
                }
            }
        }
    }
    0
}

pub fn part_b(input: &str) -> i64 {
    let mut matches = 0;
    let mut digests = Vec::new();
    for i in 0.. {
        while digests.len() <= i {
            digests.push(stretch(
                input.trim().to_owned() + &digests.len().to_string(),
            ));
        }
        let digest = &digests[i];

        if let Some(c) = has_triple(&digest) {
            let mut s = String::new();
            for _ in 0..5 {
                s.push(c);
            }
            for j in (i + 1)..=(i + 1000) {
                while digests.len() <= j {
                    digests.push(stretch(
                        input.trim().to_owned() + &digests.len().to_string(),
                    ));
                }
                let subdigest = &digests[j];

                if subdigest.contains(&s) {
                    matches += 1;
                    if matches == 64 {
                        return i as i64;
                    }
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn has_t() {
        assert_eq!(super::has_triple("baaa2"), Some('a'));
        assert_eq!(super::has_triple("baab2"), None);
    }

    #[test]
    fn example1() {
        assert_eq!(super::part_a("abc"), 22728);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt").trim()), 25427); // 15:50
    }

    #[test]
    fn stretch() {
        assert_eq!(
            super::stretch("abc0".to_owned()),
            "a107ff634856bb300138cac6568c0f24"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("abc"), 22551);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt").trim()), 22045); // 26:33
    }
}
