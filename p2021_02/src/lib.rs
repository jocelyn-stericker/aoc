pub fn part_a(input: &str) -> i64 {
    let mut x = 0;
    let mut y = 0;

    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let amount: i64 = parts.next().unwrap().parse().unwrap();
        match direction {
            "forward" => {
                x += amount;
            }
            "down" => {
                y -= amount;
            }
            "up" => {
                y += amount;
            }
            _ => panic!(),
        }
    }
    x * y
}

pub fn part_b(input: &str) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let amount: i64 = parts.next().unwrap().parse().unwrap();
        match direction {
            "forward" => {
                x += amount;
                y += aim * amount;
            }
            "down" => {
                aim += amount;
            }
            "up" => {
                aim -= amount;
            }
            _ => panic!(),
        }
    }
    x * y
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), -1480518);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1282809906);
    }
}
