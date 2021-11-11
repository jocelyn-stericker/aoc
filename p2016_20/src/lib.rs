use std::collections::BTreeMap;

pub fn part_a(input: &str) -> u64 {
    let mut end_for = BTreeMap::new();

    for line in input.trim().split('\n') {
        let mut p = line.split('-').map(|p| p.parse().unwrap());
        let min: u64 = p.next().unwrap();
        let max: u64 = p.next().unwrap();
        let x = end_for.insert(min, max);
        if x.is_some() {
            panic!();
        }
    }

    let starts: Vec<u64> = end_for.keys().copied().collect();

    let end = 0;
    let mut next = end;
    let mut did_something = false;
    loop {
        for &start in &starts {
            if start <= next && end_for[&start] >= next && end_for[&start] + 1 > next {
                next = end_for[&start] + 1;
                did_something = true;
            }
        }
        if !did_something {
            break;
        }
        did_something = false;
    }

    next
}

pub fn part_b(input: &str) -> u64 {
    let mut end_for = BTreeMap::new();

    for line in input.trim().split('\n') {
        let mut p = line.split('-').map(|p| p.parse().unwrap());
        let min: u64 = p.next().unwrap();
        let max: u64 = p.next().unwrap();
        let x = end_for.insert(min, max);
        if x.is_some() {
            panic!();
        }
    }

    end_for.insert(4294967295 + 1, u64::MAX);

    let mut sum = 0;

    let starts: Vec<u64> = end_for.keys().copied().collect();

    let mut end = 0;
    while end < u64::MAX {
        let mut next = end;
        let mut did_something = false;
        loop {
            for &start in &starts {
                if start <= next && end_for[&start] >= next && end_for[&start] + 1 > next {
                    next = end_for[&start] + 1;
                    did_something = true;
                }
            }
            if !did_something {
                break;
            }
            did_something = false;
        }

        for &start in &starts {
            if start >= next {
                sum += start - next;
                end = end_for[&start];
                break;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        // 3111593 too low
        assert_eq!(super::part_a(include_str!("input.txt")), 31053880);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 117);
    }
}
