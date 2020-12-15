use std::collections::HashMap;

pub fn solve(input: &str, n: usize) -> usize {
    let mut turns = HashMap::new();
    let mut spoken = 0;
    let mut prev_spoken = None;
    for (i, line) in input.trim().split(',').enumerate() {
        let line = line.parse::<usize>().unwrap();
        prev_spoken = turns.get(&line).copied();
        turns.insert(line, i);
        spoken = line;
    }
    for i in turns.len()..n {
        if i % 100000 == 0 {
            eprintln!("{}", i);
        }
        let val = i - 1 - prev_spoken.unwrap_or(i - 1);
        prev_spoken = turns.get(&val).copied();
        turns.insert(val, i);
        spoken = val;
    }
    spoken
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::solve("0,3,6\n", 2020), 436);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::solve(include_str!("input.txt"), 2020), 410);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::solve(include_str!("input.txt"), 30000000), 238);
    }
}
