use std::collections::BTreeSet;

pub fn solve(input: &str, expand: i64) -> i64 {
    let mut orig_map = BTreeSet::new();
    let mut max_x = 0;
    let mut max_y = 0;

    let mut used_cols = BTreeSet::new();
    let mut used_rows = BTreeSet::new();
    for (y, line) in input.trim().split('\n').enumerate() {
        let y = y as i64;
        max_y = max_y.max(y);
        for (x, char) in line.chars().enumerate() {
            let x = x as i64;
            max_x = max_x.max(y);
            if char == '#' {
                orig_map.insert((x, y));
                used_cols.insert(x);
                used_rows.insert(y);
            }
        }
        //
    }

    let mut dx = 0;
    let mut map = BTreeSet::new();
    for x in 0..=max_x {
        let mut dy = 0;
        if !used_cols.contains(&x) {
            dx += expand - 1;
        }
        for y in 0..=max_y {
            if !used_rows.contains(&y) {
                dy += expand - 1;
            }
            if orig_map.contains(&(x, y)) {
                map.insert((x + dx, y + dy));
            }
        }
    }

    let mut sum = 0;
    for (i, spot) in map.iter().enumerate() {
        for spot2 in map.iter().skip(i + 1) {
            sum += (spot.0 - spot2.0).abs() + (spot.1 - spot2.1).abs();
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::solve(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....\n",
                2
            ),
            374
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::solve(include_str!("input.txt"), 2), 9805264);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::solve(include_str!("input.txt"), 1000000), 779032247216);
    }
}
