fn combinations(containers: &[i64], remaining: i64) -> i64 {
    match remaining {
        0 => 1,
        i if i < 0 => 0,
        _ => {
            let mut count = 0;
            for i in 0..containers.len() {
                count += combinations(&containers[i + 1..], remaining - containers[i]);
            }

            count
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut containers: Vec<i64> = Vec::new();
    for line in input.trim().split('\n') {
        containers.push(line.parse().unwrap());
    }
    combinations(&containers, 150)
}

fn best(containers: &[i64], remaining: i64, used: i64) -> i64 {
    match remaining {
        0 => used,
        i if i < 0 => i64::MAX,
        _ => {
            let mut best_so_far = i64::MAX;
            for i in 0..containers.len() {
                best_so_far = best_so_far.min(best(
                    &containers[i + 1..],
                    remaining - containers[i],
                    used + 1,
                ));
            }

            best_so_far
        }
    }
}

fn with_best(containers: &[i64], remaining: i64, used: i64, best: i64) -> i64 {
    match remaining {
        0 if used == best => 1,
        i if i <= 0 => 0,
        _ => {
            let mut count = 0;
            for i in 0..containers.len() {
                count += with_best(
                    &containers[i + 1..],
                    remaining - containers[i],
                    used + 1,
                    best,
                );
            }

            count
        }
    }
}

pub fn part_b(input: &str) -> i64 {
    let mut containers: Vec<i64> = Vec::new();
    for line in input.trim().split('\n') {
        containers.push(line.parse().unwrap());
    }
    with_best(&containers, 150, 0, best(&containers, 150, 0))
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_b("20\n15\n10\n5\n5\n"), 3);
    // }

    #[test]
    fn part_a() {
        // 1317636 too high
        assert_eq!(super::part_a(include_str!("input.txt")), 1638);
    }

    #[test]
    fn part_b() {
        // not 4
        assert_eq!(super::part_b(include_str!("input.txt")), 17);
    }
}
