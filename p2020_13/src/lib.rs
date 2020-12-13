// use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    let mut lines = input.trim().split('\n');
    let timestamp = lines.next().unwrap().parse::<i64>().unwrap();
    let mut min_bus = 0;
    let mut min_time = 1000000000;
    for bus in lines.next().unwrap().split(',').filter(|t| *t != "x") {
        let bus = bus.parse::<i64>().unwrap();
        let wait = bus - (timestamp % bus);
        if wait < min_time {
            min_time = wait;
            min_bus = bus;
        }
    }
    min_time * min_bus
}

// part_b used Chinese Remainder Theorm
// https://www.dcode.fr/chinese-remainder

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("939\n7,13,x,x,59,x,31,19\n"), 295);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2095);
    }
}
