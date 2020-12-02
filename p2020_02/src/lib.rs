// use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    let mut total = 0;
    for line in input.trim().split('\n') {
        let mut x = line.split(' ');
        let min = x.next().unwrap().parse::<i64>().unwrap();
        let max = x.next().unwrap().parse::<i64>().unwrap();
        let c = x.next().unwrap().chars().next().unwrap();

        let mut count = 0;
        for c2 in x.next().unwrap().chars() {
            if c2 == c {
                count += 1;
            }
        }

        if count >= min && count <= max {
            total += 1;
        }
    }
    total
}

pub fn part_b(input: &str) -> i64 {
    let mut total = 0;
    for line in input.trim().split('\n') {
        let mut x = line.split(' ');
        let min = x.next().unwrap().parse::<usize>().unwrap();
        let max = x.next().unwrap().parse::<usize>().unwrap();
        let c = x.next().unwrap().chars().next().unwrap();

        let s: Vec<_> = x.next().unwrap().chars().collect();
        if (s.get(min - 1) == Some(&c)) != (s.get(max - 1) == Some(&c)) {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 465);
    }

    #[test]
    fn part_b() {
        // 385
        assert_eq!(super::part_b(include_str!("input.txt")), 294);
    }
}
