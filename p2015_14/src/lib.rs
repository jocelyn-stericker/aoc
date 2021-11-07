use std::collections::BTreeMap;

pub fn part_a(input: &str) -> i64 {
    struct Reindeer {
        vel: i64,
        time_on: i64,
        time_off: i64,
        dis: i64,
    }
    let mut reindeers = BTreeMap::new();

    for line in input.trim().split('\n') {
        let parts: Vec<_> = line.split('.').next().unwrap().split(' ').collect();
        reindeers.insert(
            parts[0],
            Reindeer {
                vel: parts[3].parse().unwrap(),
                time_on: parts[6].parse().unwrap(),
                time_off: parts[13].parse().unwrap(),
                dis: 0,
            },
        );
    }

    for t in 0..2503 {
        for r in reindeers.values_mut() {
            let tc = t % (r.time_on + r.time_off);
            if tc < r.time_on {
                r.dis += r.vel;
            }
        }
    }

    reindeers.iter().max_by_key(|(_, v)| v.dis).unwrap().1.dis
}

pub fn part_b(input: &str) -> i64 {
    struct Reindeer {
        vel: i64,
        time_on: i64,
        time_off: i64,
        dis: i64,
        points: i64,
    }
    let mut reindeers = BTreeMap::new();

    for line in input.trim().split('\n') {
        let parts: Vec<_> = line.split('.').next().unwrap().split(' ').collect();
        reindeers.insert(
            parts[0],
            Reindeer {
                vel: parts[3].parse().unwrap(),
                time_on: parts[6].parse().unwrap(),
                time_off: parts[13].parse().unwrap(),
                dis: 0,
                points: 0,
            },
        );
    }

    for t in 0..2503 {
        for r in reindeers.values_mut() {
            let tc = t % (r.time_on + r.time_off);
            if tc < r.time_on {
                r.dis += r.vel;
            }
        }

        let max_dis = reindeers.iter().max_by_key(|(_, v)| v.dis).unwrap().1.dis;

        for r in reindeers.values_mut() {
            if r.dis == max_dis {
                r.points += 1;
            }
        }
    }

    reindeers
        .iter()
        .max_by_key(|(_, v)| v.points)
        .unwrap()
        .1
        .points
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2655);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1059);
    }
}
