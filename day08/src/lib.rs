fn checksum(subtree: &[u32]) -> (usize, u32) {
    let header = (subtree[0], subtree[1]);
    let mut loc = 2;
    let mut hash = 0;

    // subtrees
    for _ in 0..header.0 {
        let (sub_len, sub_hash) = checksum(&subtree[loc..]);
        loc += sub_len;
        hash += sub_hash;
    }

    // metadata
    for _ in 0..header.1 {
        hash += subtree[loc];
        loc += 1;
    }

    (loc, hash)
}

fn value(subtree: &[u32]) -> (usize, u32) {
    let header = (subtree[0], subtree[1] as usize);
    let mut loc: usize = 2;
    let mut val = 0;

    let mut sub_vals: Vec<u32> = Vec::new();

    // subtrees
    for _ in 0..header.0 {
        let (sub_len, sub_val) = value(&subtree[loc..]);
        sub_vals.push(sub_val);
        loc += sub_len;
    }

    // metadata
    for meta in &subtree[loc..(loc + header.1)] {
        if header.0 == 0 {
            val += meta;
        } else {
            let meta = *meta as usize;
            // sub_vals is 1-indexed
            if meta > 0 && meta - 1 < sub_vals.len() {
                val += sub_vals[meta - 1];
            }
        }
    }

    (loc + header.1, val)
}

fn parse(input: &str) -> Vec<u32> {
    input
        .split(' ')
        .map(|line| line.trim())
        .filter(|line| line != &"")
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

pub fn part_a(input: &str) -> u32 {
    checksum(&parse(input)).1
}

pub fn part_b(input: &str) -> u32 {
    value(&parse(input)).1
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n\n"),
            138
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 35852);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n\n"), 66);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 33422);
    }
}
