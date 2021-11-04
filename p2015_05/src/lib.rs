use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut nice = 0;
    'a: for line in input.trim().split('\n') {
        let mut vowels = 0;
        let mut dups = 0;
        let chars: Vec<_> = line.chars().collect();
        for c in &chars {
            if ['a', 'e', 'i', 'o', 'u'].contains(c) {
                vowels += 1;
            }
        }
        for i in 1..chars.len() {
            let p = chars[i - 1];
            let c = chars[i];
            if p == c {
                dups += 1;
            }
            if p == 'a' && c == 'b'
                || p == 'c' && c == 'd'
                || p == 'p' && c == 'q'
                || p == 'x' && c == 'y'
            {
                continue 'a;
            }
        }

        if vowels >= 3 && dups >= 1 {
            nice += 1;
        }
    }
    nice
}

pub fn part_b(input: &str) -> i64 {
    let mut nice = 0;
    for line in input.trim().split('\n') {
        let mut dup_one = 0;
        let mut dup_two = 0;
        let mut dups = HashMap::new();
        let chars: Vec<_> = line.chars().collect();

        for i in 1..chars.len() {
            let p = chars[i - 1];
            let c = chars[i];

            if let Some(j) = dups.get(&(p, c)) {
                if i > j + 1 {
                    dup_two += 1;
                }
            } else {
                dups.insert((p, c), i);
            }
        }

        for i in 2..chars.len() {
            let a = chars[i - 2];
            let c = chars[i];
            if a == c {
                dup_one += 1;
            }
        }

        if dup_two >= 1 && dup_one >= 1 {
            nice += 1;
        }
    }
    nice
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("ugknbfddgicrmopn\n"), 1);
        assert_eq!(super::part_a("aaa\n"), 1);
        assert_eq!(super::part_a("jchzalrnumimnmhp\n"), 0);
        assert_eq!(super::part_a("haegwjzuvuyypxyu\n"), 0);
        assert_eq!(super::part_b("uurcxstgmygtbstg\n"), 0);
        assert_eq!(super::part_b("ieodomkazucvgmuy\n"), 0);
        assert_eq!(super::part_b("qjhvhtzxzqqjkmpb\n"), 1);
        assert_eq!(super::part_b("xxyxx\n"), 1);
        assert_eq!(super::part_b("aaa\n"), 0);
        assert_eq!(super::part_b("aaba\n"), 0);
        assert_eq!(super::part_b("aabaa\n"), 1);
        assert_eq!(super::part_b("xyxy\n"), 1);
        assert_eq!(super::part_b("aabcdegfgaa\n"), 1);
    }
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 255);
    }
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 55);
    }
}
