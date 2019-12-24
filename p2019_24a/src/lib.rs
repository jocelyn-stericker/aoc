use std::collections::BTreeSet;

fn adj_count(t: &BTreeSet<(i8, i8)>, x: i8, y: i8) -> u8 {
    let mut sum = 0;
    if t.contains(&(x - 1, y)) {
        sum += 1;
    }
    if t.contains(&(x + 1, y)) {
        sum += 1;
    }
    if t.contains(&(x, y - 1)) {
        sum += 1;
    }
    if t.contains(&(x, y + 1)) {
        sum += 1;
    }

    sum
}

fn next(t: &BTreeSet<(i8, i8)>, w: i8, h: i8) -> BTreeSet<(i8, i8)> {
    let mut hs = BTreeSet::new();
    for x in 0..w {
        for y in 0..h {
            let count = adj_count(t, x, y);
            let v = t.contains(&(x, y));
            if v && count == 1 {
                hs.insert((x, y));
            } else if !v && (count == 1 || count == 2) {
                hs.insert((x, y));
            }
        }
    }

    hs
}

fn bio(t: &BTreeSet<(i8, i8)>, w: i8, h: i8) -> i64 {
    let mut incr = 1;
    let mut score = 0;
    for y in 0..h {
        for x in 0..w {
            if t.contains(&(x, y)) {
                score += incr;
            }
            incr *= 2;
        }
    }

    score
}

fn print(t: &BTreeSet<(i8, i8)>, w: i8, h: i8) {
    for y in 0..h {
        for x in 0..w {
            if t.contains(&(x, y)) {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
    eprintln!();
}

pub fn part_a(input: &str) -> i64 {
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
                hs.insert((x as i8, y as i8));
            }
        }
    }

    let mut seen = BTreeSet::new();
    seen.insert(hs.clone());

    loop {
        hs = next(&hs, w, h);
        if seen.contains(&hs) {
            print(&hs, w, h);
            return bio(&hs, w, h);
        }
        seen.insert(hs.clone());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 28615131);
    }
}
