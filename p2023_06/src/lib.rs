pub fn part_a(input: &str) -> i64 {
    let mut lines = input.trim().split('\n');
    let times = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut memo = 1;
    for (time, record) in times.iter().zip(distances) {
        let mut ways = 0;
        for speed in 0..*time {
            let distance = speed * (time - speed);
            if distance > record {
                ways += 1;
            }
        }
        memo *= ways;
    }
    memo
}

pub fn part_b(input: &str) -> i64 {
    let mut lines = input.trim().split('\n');
    let time = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let record = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let mut ways = 0;
    for speed in 0..time {
        let distance = speed * (time - speed);
        if distance > record {
            ways += 1;
        }
    }
    ways
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "Time:      7  15   30
Distance:  9  40  200\n"
            ),
            288
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 6209190);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "Time:      7  15   30
Distance:  9  40  200\n"
            ),
            71503
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 28545089);
    }
}
