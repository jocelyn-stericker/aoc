use std::collections::HashSet;

pub fn print(on: &HashSet<(i64, i64)>) {
    for y in 0..7 {
        for x in 0..50 {
            if on.contains(&(x, y)) {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut on = HashSet::new(); // (y, x)

    for line in input.trim().lines() {
        let cmd: Vec<_> = line.trim().split(' ').collect();
        let a: i64 = cmd[1].parse().unwrap();
        let b: i64 = cmd[2].parse().unwrap();
        let cmd = cmd[0];

        match cmd {
            "rect" => {
                for y in 0..a {
                    // a tall
                    for x in 0..b {
                        // b wide
                        on.insert((y, x));
                    }
                }
            }
            "colr" => {
                // a is row to shift
                // b is the number to shift by
                let mut to_add = HashSet::new();
                for (y, x) in on.iter() {
                    if *y == a {
                        to_add.insert((*y, (x + b) % 6));
                    }
                }
                on = on.into_iter().filter(|(y, _x)| *y != a).collect();
                for (y, x) in to_add.into_iter() {
                    on.insert((y, x));
                }
            }
            "rowr" => {
                // a is col to shift
                // b is the number to shift by
                let mut to_add = HashSet::new();
                for (y, x) in on.iter() {
                    if *x == a {
                        to_add.insert(((y + b) % 50, *x));
                    }
                }
                on = on.into_iter().filter(|(_y, x)| *x != a).collect();
                for (y, x) in to_add.into_iter() {
                    on.insert((y, x));
                }
            }
            _ => panic!(),
        }
    }
    print(&on);

    on.len() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        // not 81
        assert_eq!(super::part_a(include_str!("input.txt")), 115); // 20:40
                                                                   // part_b: 21:42
    }
}
