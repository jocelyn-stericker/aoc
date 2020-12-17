use std::collections::HashSet;

fn neighbours(pt: &(i64, i64, i64, i64)) -> Vec<(i64, i64, i64, i64)> {
    let mut n = Vec::new();

    for dz in -1..=1 {
        for dy in -1..=1 {
            for dx in -1..=1 {
                for dw in -1..=1 {
                    if dz != 0 || dy != 0 || dx != 0 || dw != 0 {
                        n.push((pt.0 + dw, pt.1 + dx, pt.2 + dy, pt.3 + dz));
                    }
                }
            }
        }
    }

    n
}

fn range(active: &HashSet<(i64, i64, i64, i64)>) -> (i64, i64, i64, i64, i64, i64, i64, i64) {
    let mut min_w = 1000000;
    let mut max_w = -1000000;
    let mut min_x = 1000000;
    let mut max_x = -1000000;
    let mut min_y = 1000000;
    let mut max_y = -1000000;
    let mut min_z = 1000000;
    let mut max_z = -1000000;

    for (w, x, y, z) in active {
        min_w = min_w.min(*w - 1);
        max_w = max_w.max(*w + 1);
        min_x = min_x.min(*x - 1);
        max_x = max_x.max(*x + 1);
        min_y = min_y.min(*y - 1);
        max_y = max_y.max(*y + 1);
        min_z = min_z.min(*z - 1);
        max_z = max_z.max(*z + 1);
    }

    (min_w, max_w, min_x, max_x, min_y, max_y, min_z, max_z)
}

pub fn part_b(input: &str) -> usize {
    let mut active: HashSet<(i64, i64, i64, i64)> = HashSet::new();

    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert((0, x as i64, y as i64, 0));
            }
        }
        //
    }

    for _ in 0..6 {
        let mut next_active = HashSet::new();
        let (min_w, max_w, min_x, max_x, min_y, max_y, min_z, max_z) = range(&active);
        for w in min_w..=max_w {
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    for z in min_z..=max_z {
                        let mut active_count = 0;
                        for n in neighbours(&(w, x, y, z)).into_iter() {
                            if active.contains(&n) {
                                active_count += 1;
                            }
                        }

                        if active.contains(&(w, x, y, z)) {
                            if active_count == 2 || active_count == 3 {
                                next_active.insert((w, x, y, z));
                            }
                        } else if active_count == 3 {
                            next_active.insert((w, x, y, z));
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut active, &mut next_active);
    }

    active.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_b(".#.\n..#\n###\n"), 848);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2460);
    }
}
