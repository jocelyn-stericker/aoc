use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut d: Vec<_> = input
        .trim()
        .split('\n')
        .map(|i| i.parse::<i64>().unwrap())
        .collect();
    d.sort_unstable();

    let mut prev: i64 = 0;
    let mut one = 0;
    let mut three = 0;
    for i in &d {
        match *i - prev {
            3 => {
                three += 1;
            }
            1 => {
                one += 1;
            }
            _ => {
                panic!();
            }
        }
        prev = *i;
    }
    one * (three + 1)
}

pub fn count(prev: i64, d: &[i64], memo: &mut HashMap<usize, isize>) -> isize {
    if d.is_empty() {
        return 1;
    }
    if memo.contains_key(&d.len()) {
        return *memo.get(&d.len()).unwrap();
    }

    let mut s = 0;
    for (i, e) in d.iter().enumerate() {
        if *e - prev > 3 {
            break;
        }
        s += count(d[i], &d[i + 1..], memo);
    }

    memo.insert(d.len(), s);
    s
}

pub fn part_b(input: &str) -> isize {
    let mut d: Vec<_> = input
        .trim()
        .split('\n')
        .map(|i| i.parse::<i64>().unwrap())
        .collect();
    d.sort_unstable();
    d.push(d.last().unwrap() + 3);

    let mut memo = HashMap::new();
    count(0, &d, &mut memo)
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2312);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 12089663946752);
    }
}
