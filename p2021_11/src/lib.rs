use std::collections::HashMap;

fn flash(oct: &mut HashMap<(i32, i32), u32>, y: i32, x: i32) -> u64 {
    let mut flashes = 1;
    for (dy, dx) in [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ] {
        if let Some(val) = oct.get_mut(&(y + dy, x + dx)) {
            *val += 1;
            if *val == 10 {
                flashes += flash(oct, y + dy, x + dx);
            }
        }
    }
    flashes
}

pub fn part_a(input: &str) -> u64 {
    let mut oct = HashMap::new();

    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            oct.insert((y as i32, x as i32), c.to_digit(10).unwrap());
        }
    }

    let mut flashes = 0;
    for _ in 0..100 {
        for y in 0..10 {
            for x in 0..10 {
                if let Some(val) = oct.get_mut(&(y, x)) {
                    *val += 1;
                    if *val == 10 {
                        flashes += flash(&mut oct, y, x);
                    }
                }
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                if let Some(val) = oct.get_mut(&(y, x)) {
                    if *val >= 10 {
                        *val = 0;
                    }
                }
            }
        }
    }

    flashes
}

pub fn part_b(input: &str) -> u64 {
    let mut oct = HashMap::new();

    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            oct.insert((y as i32, x as i32), c.to_digit(10).unwrap());
        }
    }

    for i in 1.. {
        for y in 0..10 {
            for x in 0..10 {
                if let Some(val) = oct.get_mut(&(y, x)) {
                    *val += 1;
                    if *val == 10 {
                        flash(&mut oct, y, x);
                    }
                }
            }
        }

        let mut is_sync = true;
        for y in 0..10 {
            for x in 0..10 {
                if let Some(val) = oct.get_mut(&(y, x)) {
                    if *val >= 10 {
                        *val = 0;
                    } else {
                        is_sync = false;
                    }
                }
            }
        }

        if is_sync {
            return i;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            ),
            1656
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1661);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            ),
            195
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 334);
    }
}
