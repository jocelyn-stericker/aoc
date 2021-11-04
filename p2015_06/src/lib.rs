use std::collections::{HashMap, HashSet};

pub fn part_a(input: &str) -> usize {
    let mut universe = HashSet::new();
    for line in input.trim().split('\n') {
        let args: Vec<_> = line.split(' ').collect();
        let verb = args[0];
        let start: Vec<i64> = args[1].split(',').map(|i| i.parse().unwrap()).collect();
        let end: Vec<i64> = args[3].split(',').map(|i| i.parse().unwrap()).collect();

        let x0 = start[0];
        let y0 = start[1];
        let x1 = end[0];
        let y1 = end[1];
        for x in x0..=x1 {
            for y in y0..=y1 {
                match verb {
                    "toggle" => {
                        if universe.contains(&(y, x)) {
                            universe.remove(&(y, x));
                        } else {
                            universe.insert((y, x));
                        }
                    }
                    "off" => {
                        universe.remove(&(y, x));
                    }
                    "on" => {
                        universe.insert((y, x));
                    }
                    _ => panic!(),
                }
            }
        }
        //
    }
    universe.len()
}

pub fn part_b(input: &str) -> usize {
    let mut universe = HashMap::new();
    for line in input.trim().split('\n') {
        let args: Vec<_> = line.split(' ').collect();
        let verb = args[0];
        let start: Vec<i64> = args[1].split(',').map(|i| i.parse().unwrap()).collect();
        let end: Vec<i64> = args[3].split(',').map(|i| i.parse().unwrap()).collect();

        let x0 = start[0];
        let y0 = start[1];
        let x1 = end[0];
        let y1 = end[1];
        for x in x0..=x1 {
            for y in y0..=y1 {
                match verb {
                    "toggle" => {
                        *universe.entry((y, x)).or_insert(0) += 2;
                    }
                    "off" => {
                        let x = universe.entry((y, x)).or_insert(0);
                        if *x > 0 {
                            *x -= 1;
                        }
                    }
                    "on" => {
                        *universe.entry((y, x)).or_insert(0) += 1;
                    }
                    _ => panic!(),
                }
            }
        }
        //
    }
    universe.values().sum()
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 543903);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 14687245);
    }
}
