use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut sum = 0;

    for mut case in input
        .trim()
        .split('\n')
        .map(|line| { line.trim().split('-') }.collect::<Vec<_>>())
    {
        let data: Vec<_> = case.pop().unwrap().split('[').collect();
        let their_check = data[1].split(']').next().unwrap();

        let mut counts = HashMap::new();
        for c in case.join("").chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut counts = counts.iter().collect::<Vec<_>>();
        counts.sort_unstable_by_key(|(a, b)| (-**b, **a));
        let our_check = counts.iter().map(|c| c.0).take(5).collect::<String>();
        if their_check == our_check {
            sum += data[0].parse::<i64>().unwrap();
        }
    }

    sum
}

fn decrypt(cypher: &str, sector: i64) -> String {
    let mut plain = Vec::new();

    for mut c in cypher.chars() {
        for _ in 0..sector {
            if c == 'z' {
                c = 'a';
            } else if c == '-' {
                c = '-';
            } else {
                c = (c as u8 + 1) as char;
            }
        }
        plain.push(c);
    }

    plain.iter().collect()
}

pub fn part_b(input: &str) -> i64 {
    for mut case in input
        .trim()
        .split('\n')
        .map(|line| { line.trim().split('-') }.collect::<Vec<_>>())
    {
        let data: Vec<_> = case.pop().unwrap().split('[').collect();
        let their_check = data[1].split(']').next().unwrap();

        let mut counts = HashMap::new();
        for c in case.join("").chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut counts = counts.iter().collect::<Vec<_>>();
        counts.sort_unstable_by_key(|(a, b)| (-**b, **a));
        let our_check = counts.iter().map(|c| c.0).take(5).collect::<String>();
        if their_check == our_check {
            let sector = data[0].parse::<i64>().unwrap();
            let plain = decrypt(&case.join("-"), sector);
            if plain.contains("north") {
                return sector;
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]\n"), 1514);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 173787); //12:56
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 548); // 21:30
    }
}
