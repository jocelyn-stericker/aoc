use std::collections::VecDeque;

use std::collections::HashMap;

pub fn part_a(input: &str) -> u64 {
    let mut mask = String::new();
    let mut vals = HashMap::new();
    for line in input.trim().split('\n') {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        if key == "mask" {
            mask = value.chars().rev().collect();
        } else {
            let mut parts = key.split('[');
            parts.next().unwrap();
            let addr = parts
                .next()
                .unwrap()
                .split(']')
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let binary: String = format!("{:b}", value.parse::<i64>().unwrap())
                .chars()
                .rev()
                .collect();
            let mut new_binary = String::new();
            let mut maski = mask.chars();
            let mut bini = binary.chars();
            for _ in 0..mask.len() {
                let maskc = maski.next().unwrap();
                let binc = bini.next();
                if maskc != 'X' {
                    new_binary.push(maskc);
                } else {
                    new_binary.push(binc.unwrap_or('0'));
                }
            }

            let new_binary: String = new_binary.chars().rev().collect();
            let val = std::primitive::u64::from_str_radix(&new_binary, 2).unwrap();
            vals.insert(addr, val);
        }
    }
    let mut sum = 0;
    for v in vals.values() {
        sum += *v;
    }
    sum
}

pub fn part_b(input: &str) -> u64 {
    let mut mask = String::new();
    let mut mem = HashMap::new();
    for line in input.trim().split('\n') {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        if key == "mask" {
            mask = value.chars().rev().collect();
        } else {
            let mut parts = key.split('[');
            parts.next().unwrap();
            let addr = parts
                .next()
                .unwrap()
                .split(']')
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();

            let binary: String = format!("{:b}", addr).chars().rev().collect();
            let mut new_binary = String::new();
            let mut maski = mask.chars();
            let mut bini = binary.chars();
            for _ in 0..mask.len() {
                let maskc = maski.next().unwrap();
                let binc = bini.next();
                if maskc == 'X' {
                    new_binary.push('X');
                } else if maskc == '1' {
                    new_binary.push('1');
                } else {
                    new_binary.push(binc.unwrap_or('0'));
                }
            }

            let new_binary: String = new_binary.chars().rev().collect();

            let mut worlds = Vec::new();
            worlds.push(String::new());
            for c in new_binary.chars() {
                if c == 'X' {
                    let mut new_strings = Vec::new();
                    for string in &worlds {
                        new_strings.push(format!("{}0", string));
                        new_strings.push(format!("{}1", string));
                    }
                    worlds = new_strings;
                } else {
                    for string in worlds.iter_mut() {
                        string.push(c);
                    }
                }
            }

            let worlds: Vec<u64> = worlds
                .iter()
                .map(|string| std::primitive::u64::from_str_radix(&string, 2).unwrap())
                .collect();

            for world in &worlds {
                mem.insert(*world, value.parse::<u64>().unwrap());
            }

            //let val = std::primitive::u64::from_str_radix(&new_binary, 2).unwrap();
            //vals.insert(addr, val);
        }
    }
    let mut sum = 0;
    for v in mem.values() {
        sum += *v;
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn example2() {
        assert_eq!(super::part_b("mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1\n"), 208);
    }

    #[test]
    fn part_b() {
        // not 13401933266411
        assert_eq!(super::part_b(include_str!("input.txt")), 3974538275659);
    }

    #[test]
    fn example1() {
        assert_eq!(super::part_a("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0"), 165);
    }

    #[test]
    fn part_a() {
        // not 13401933266411
        assert_eq!(super::part_a(include_str!("input.txt")), 10717676595607);
    }
}
