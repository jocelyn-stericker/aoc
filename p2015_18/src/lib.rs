use std::collections::BTreeSet;

pub fn part_a(input: &str) -> usize {
    let mut on = BTreeSet::new();
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                on.insert((x as i32, y as i32));
            }
        }
    }

    for _ in 0..100 {
        let mut next = BTreeSet::new();
        for x in 0..100 {
            for y in 0..100 {
                let mut neighbours = 0;
                for (dx, dy) in [
                    (-1i32, -1i32),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    if on.contains(&(x + dx, y + dy)) {
                        neighbours += 1;
                    }
                }

                if on.contains(&(x, y)) && (neighbours == 2 || neighbours == 3)
                    || !on.contains(&(x, y)) && neighbours == 3
                {
                    next.insert((x, y));
                }
            }
        }

        std::mem::swap(&mut on, &mut next);
    }

    on.len()
}

pub fn part_b(input: &str) -> usize {
    let mut on = BTreeSet::new();
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                on.insert((x as i32, y as i32));
            }
        }
    }

    for _ in 0..100 {
        on.insert((0, 0));
        on.insert((0, 99));
        on.insert((99, 99));
        on.insert((99, 0));
        let mut next = BTreeSet::new();
        for x in 0..100 {
            for y in 0..100 {
                let mut neighbours = 0;
                for (dx, dy) in [
                    (-1i32, -1i32),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    if on.contains(&(x + dx, y + dy)) {
                        neighbours += 1;
                    }
                }

                if on.contains(&(x, y)) && (neighbours == 2 || neighbours == 3)
                    || !on.contains(&(x, y)) && neighbours == 3
                {
                    next.insert((x, y));
                }
            }
        }

        std::mem::swap(&mut on, &mut next);
    }

    on.insert((0, 0));
    on.insert((0, 99));
    on.insert((99, 99));
    on.insert((99, 0));

    on.len()
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1061);
    }

    #[test]
    fn part_b() {
        // 1005 is too low
        assert_eq!(super::part_b(include_str!("input.txt")), 1006);
    }
}
