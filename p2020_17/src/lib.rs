use std::collections::HashSet;

fn neighbours(pt: &(i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    let mut n = Vec::new();

    for dz in -1..=1 {
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dz != 0 || dy != 0 || dx != 0 {
                    n.push((pt.0 + dx, pt.1 + dy, pt.2 + dz));
                }
            }
        }
    }

    n
}

fn range(active: &HashSet<(i64, i64, i64)>) -> (i64, i64, i64, i64, i64, i64) {
    let mut min_x = 1000000;
    let mut max_x = -1000000;
    let mut min_y = 1000000;
    let mut max_y = -1000000;
    let mut min_z = 1000000;
    let mut max_z = -1000000;

    for (x, y, z) in active {
        min_x = min_x.min(*x - 1);
        max_x = max_x.max(*x + 1);
        min_y = min_y.min(*y - 1);
        max_y = max_y.max(*y + 1);
        min_z = min_z.min(*z - 1);
        max_z = max_z.max(*z + 1);
    }

    (min_x, max_x, min_y, max_y, min_z, max_z)
}

pub fn part_a(input: &str) -> usize {
    let mut active: HashSet<(i64, i64, i64)> = HashSet::new();

    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert((x as i64, y as i64, 0));
            }
        }
        //
    }
    eprintln!("--> {:?}", neighbours(&(1, 1, 1)).len());

    for _ in 0..6 {
        let mut next_active = HashSet::new();
        let (min_x, max_x, min_y, max_y, min_z, max_z) = range(&active);
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    let mut active_count = 0;
                    for n in neighbours(&(x, y, z)).into_iter() {
                        if active.contains(&n) {
                            active_count += 1;
                        }
                    }

                    if active.contains(&(x, y, z)) {
                        if active_count == 2 || active_count == 3 {
                            next_active.insert((x, y, z));
                        }
                    } else if active_count == 3 {
                        next_active.insert((x, y, z));
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
        assert_eq!(super::part_a(".#.\n..#\n###\n"), 112);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 424);
    }
}
