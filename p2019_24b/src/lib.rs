use std::collections::BTreeSet;

fn adj_count(t: &BTreeSet<(i8, i8, i8)>, l: i8, x: i8, y: i8) -> u8 {
    let mut sum = 0;
    // Adj
    if t.contains(&(l, x - 1, y)) {
        sum += 1;
    }
    if t.contains(&(l, x + 1, y)) {
        sum += 1;
    }
    if t.contains(&(l, x, y - 1)) {
        sum += 1;
    }
    if t.contains(&(l, x, y + 1)) {
        sum += 1;
    }

    // Outside
    if x == 0 && t.contains(&(l + 1, 1, 2)) {
        sum += 1;
    }
    if y == 0 && t.contains(&(l + 1, 2, 1)) {
        sum += 1;
    }
    if x == 4 && t.contains(&(l + 1, 3, 2)) {
        sum += 1;
    }
    if y == 4 && t.contains(&(l + 1, 2, 3)) {
        sum += 1;
    }

    // Inside
    if x == 1 && y == 2 {
        for y2 in 0..5 {
            if t.contains(&(l - 1, 0, y2)) {
                sum += 1;
            }
        }
    }
    if x == 2 && y == 1 {
        for x2 in 0..5 {
            if t.contains(&(l - 1, x2, 0)) {
                sum += 1;
            }
        }
    }
    if x == 3 && y == 2 {
        for y2 in 0..5 {
            if t.contains(&(l - 1, 4, y2)) {
                sum += 1;
            }
        }
    }
    if x == 2 && y == 3 {
        for x2 in 0..5 {
            if t.contains(&(l - 1, x2, 4)) {
                sum += 1;
            }
        }
    }

    sum
}

fn next(t: &BTreeSet<(i8, i8, i8)>) -> BTreeSet<(i8, i8, i8)> {
    let min = t.iter().min_by_key(|k| k.0).unwrap().0;
    let max = t.iter().max_by_key(|k| k.0).unwrap().0;
    let mut hs = BTreeSet::new();
    for level in (min - 1)..=(max + 1) {
        for x in 0..5 {
            for y in 0..5 {
                if x == 2 && y == 2 {
                    continue;
                }
                let count = adj_count(t, level, x, y);
                let v = t.contains(&(level, x, y));
                if v && count == 1 {
                    hs.insert((level, x, y));
                } else if !v && (count == 1 || count == 2) {
                    hs.insert((level, x, y));
                }
            }
        }
    }

    hs
}

pub fn part_b(input: &str) -> usize {
    let t: Vec<Vec<char>> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.chars().collect())
        .collect();
    let h = t.len() as i8;
    let w = t[0].len() as i8;
    let mut hs = BTreeSet::new();
    for y in 0..h {
        for x in 0..w {
            if t[y as usize][x as usize] == '#' {
                hs.insert((0 as i8, x as i8, y as i8));
            }
        }
    }

    for _ in 0..200 {
        hs = next(&hs);
    }

    hs.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1926);
    }
}
