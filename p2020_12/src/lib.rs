#[derive(Debug, Clone, Copy)]
enum Dir {
    N(i64),
    S(i64),
    E(i64),
    W(i64),
    L(i64),
    R(i64),
    F(i64),
}

pub fn part_a(input: &str) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut angle = 90;
    for line in input.trim().split('\n') {
        let mut c = line.chars();
        let d = c.next().unwrap();
        let amt = c.collect::<String>().parse::<i64>().unwrap();
        let d = match d {
            'N' => Dir::N(amt),
            'S' => Dir::S(amt),
            'E' => Dir::E(amt),
            'W' => Dir::W(amt),
            'L' => Dir::L(amt),
            'R' => Dir::R(amt),
            'F' => Dir::F(amt),
            _ => panic!(),
        };

        let d = match d {
            Dir::N(_) | Dir::S(_) | Dir::E(_) | Dir::W(_) => d,
            Dir::L(amt) => {
                angle = (angle - amt + 360) % 360;
                Dir::N(0)
            }
            Dir::R(amt) => {
                angle = (angle + amt) % 360;
                Dir::N(0)
            }
            Dir::F(amt) => match angle {
                0 => Dir::N(amt),
                90 => Dir::E(amt),
                180 => Dir::S(amt),
                270 => Dir::W(amt),
                _ => panic!(),
            },
        };

        match d {
            Dir::N(amt) => y -= amt,
            Dir::S(amt) => y += amt,
            Dir::E(amt) => x += amt,
            Dir::W(amt) => x -= amt,
            _ => panic!(),
        }
    }

    x.abs() + y.abs()
}

pub fn part_b(input: &str) -> i64 {
    let mut x = 10;
    let mut y = -1;
    let mut ship_x = 0;
    let mut ship_y = 0;
    for line in input.trim().split('\n') {
        let mut c = line.chars();
        let inst = c.next().unwrap();
        let amt = c.collect::<String>().parse::<i64>().unwrap();
        let inst = match inst {
            'N' => Dir::N(amt),
            'S' => Dir::S(amt),
            'E' => Dir::E(amt),
            'W' => Dir::W(amt),
            'L' => Dir::L(amt),
            'R' => Dir::R(amt),
            'F' => Dir::F(amt),
            _ => panic!(),
        };

        match inst {
            Dir::N(amt) => y -= amt,
            Dir::S(amt) => y += amt,
            Dir::E(amt) => x += amt,
            Dir::W(amt) => x -= amt,
            Dir::L(amt) => {
                for _ in 0..((360 - amt) / 90) {
                    let t = x;
                    x = -y;
                    y = t;
                }
            }
            Dir::R(amt) => {
                for _ in 0..(amt / 90) {
                    let t = x;
                    x = -y;
                    y = t;
                }
            }
            Dir::F(amt) => {
                ship_x += x * amt;
                ship_y += y * amt;
            }
        };
    }

    ship_x.abs() + ship_y.abs()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_b("F10\nN3\nF7\nR90\nF11\n"), 286);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 796);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 39446);
    }
}
