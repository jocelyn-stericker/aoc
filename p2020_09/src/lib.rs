// use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    let l: Vec<i64> = input
        .trim()
        .split('\n')
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    for i in 25..l.len() {
        let mut ok = false;
        'il: for j in (i - 25)..i {
            for k in (j + 1)..i {
                if l[j] + l[k] == l[i] {
                    ok = true;
                    break 'il;
                }
            }
        }
        if !ok {
            return l[i];
        }
    }
    panic!();
}

pub fn part_b(input: &str) -> i64 {
    let s = 22406676;
    let l: Vec<i64> = input
        .trim()
        .split('\n')
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    for i in 0..l.len() {
        let mut ps = 0;
        for j in i..l.len() {
            ps += l[j];
            if ps < 0 {
                panic!();
            }

            if ps > s {
                break;
            }
            if ps == s {
                let mut m1 = 100000000000000;
                let mut m2 = 0;
                for k in i..=j {
                    m1 = m1.min(l[k]);
                    m2 = m2.max(l[k]);
                }
                return m1 + m2;
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 22406676);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2942387);
    }
}
