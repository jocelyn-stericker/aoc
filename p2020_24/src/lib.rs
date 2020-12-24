use std::collections::HashSet;

enum Dir {
    E,
    Se,
    Sw,
    W,
    Nw,
    Ne,
}

impl Dir {
    fn to_delta(&self) -> (i64, i64) {
        match self {
            Dir::E => (0, 2),
            Dir::Se => (1, 1),
            Dir::Sw => (1, -1),
            Dir::W => (0, -2),
            Dir::Nw => (-1, -1),
            Dir::Ne => (-1, 1),
        }
    }
}

pub fn part_a(input: &str) -> usize {
    let mut flipped = HashSet::new();
    for line in input.trim().split('\n') {
        let mut line = line.chars();
        let mut pt = (0, 0);
        while let Some(c1) = line.next() {
            let d = match c1 {
                'n' => match line.next().unwrap() {
                    'e' => Dir::Ne,
                    'w' => Dir::Nw,
                    _ => panic!(),
                },
                's' => match line.next().unwrap() {
                    'e' => Dir::Se,
                    'w' => Dir::Sw,
                    _ => panic!(),
                },
                'e' => Dir::E,
                'w' => Dir::W,
                _ => panic!(),
            };

            let d = d.to_delta();
            pt.0 += d.0;
            pt.1 += d.1;
        }
        if !flipped.remove(&pt) {
            flipped.insert(pt);
        }
    }
    flipped.len()
}

pub fn part_b(input: &str) -> usize {
    let mut black = HashSet::new();
    for line in input.trim().split('\n') {
        let mut line = line.chars();
        let mut pt = (0, 0);
        while let Some(c1) = line.next() {
            let d = match c1 {
                'n' => match line.next().unwrap() {
                    'e' => Dir::Ne,
                    'w' => Dir::Nw,
                    _ => panic!(),
                },
                's' => match line.next().unwrap() {
                    'e' => Dir::Se,
                    'w' => Dir::Sw,
                    _ => panic!(),
                },
                'e' => Dir::E,
                'w' => Dir::W,
                _ => panic!(),
            };

            let d = d.to_delta();
            pt.0 += d.0;
            pt.1 += d.1;
        }
        if !black.remove(&pt) {
            black.insert(pt);
            let is_valid = (pt.1 % 2).abs() == (pt.0 % 2).abs();
            assert!(is_valid);
        }
    }

    for _ in 0..100 {
        let min_x = black.iter().map(|pt| pt.0).min().unwrap() - 2;
        let max_x = black.iter().map(|pt| pt.0).max().unwrap() + 2;
        let min_y = black.iter().map(|pt| pt.1).min().unwrap() - 2;
        let max_y = black.iter().map(|pt| pt.1).max().unwrap() + 2;

        let mut next = HashSet::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let is_valid = (y % 2).abs() == (x % 2).abs();
                if is_valid {
                    let mut count = 0;
                    for d in &[Dir::E, Dir::Se, Dir::Sw, Dir::W, Dir::Nw, Dir::Ne] {
                        let d = d.to_delta();
                        let nx = x + d.0;
                        let ny = y + d.1;
                        if black.contains(&(nx, ny)) {
                            count += 1;
                        }
                    }
                    if black.contains(&(x, y)) {
                        if !(count == 0 || count > 2) {
                            next.insert((x, y));
                        }
                    } else if count == 2 {
                        next.insert((x, y));
                    }
                }
            }
        }

        black = next;
    }

    black.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_a() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 10);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 450);
    }

    #[test]
    fn example_b() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 2208);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 4059);
    }
}
