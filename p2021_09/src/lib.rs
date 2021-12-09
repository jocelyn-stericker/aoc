use std::collections::HashMap;

pub fn part_a(input: &str) -> u32 {
    let mut map = HashMap::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            max_y = y;
            max_x = x;
            map.insert((y as i64, x as i64), c.to_digit(10).unwrap());
        }
    }

    let mut score = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            let y = y as i64;
            let x = x as i64;
            let p = map[&(y, x)];
            let mut low_point = true;
            for (dy, dx) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some(other) = map.get(&(y + dy, x + dx)) {
                    if *other <= p {
                        low_point = false;
                        break;
                    }
                }
            }
            if low_point {
                score += 1 + p;
            }
        }
    }

    score
}

pub fn part_b(input: &str) -> u32 {
    let mut map = HashMap::new();
    let mut basins: HashMap<(i64, i64), usize> = HashMap::new();

    let mut max_y = 0;
    let mut max_x = 0;
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            max_y = y;
            max_x = x;
            map.insert((y as i64, x as i64), c.to_digit(10).unwrap());
        }
    }

    let mut low_points = Vec::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let y = y as i64;
            let x = x as i64;
            let p = map[&(y, x)];
            let mut low_point = true;
            for (dy, dx) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some(other) = map.get(&(y + dy, x + dx)) {
                    if *other <= p {
                        low_point = false;
                        break;
                    }
                }
            }
            if low_point {
                low_points.push((y, x));
            }
        }
    }

    for (basin, point) in low_points.into_iter().enumerate() {
        let mut to_fill = vec![point];
        while let Some(point) = to_fill.pop() {
            basins.insert(point, basin);

            let height = map[&point];
            for (dy, dx) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let other_point = (point.0 + dy, point.1 + dx);
                if let Some(other) = map.get(&other_point) {
                    if *other > height && *other != 9 && !basins.contains_key(&other_point) {
                        to_fill.push(other_point);
                    }
                }
            }
        }
    }

    let mut basin_sizes: HashMap<usize, u32> = HashMap::new();
    for basin_num in basins.values() {
        *basin_sizes.entry(*basin_num).or_default() += 1;
    }

    let mut basin_sizes: Vec<_> = basin_sizes.into_iter().collect();
    basin_sizes.sort_by_key(|item| item.1);
    basin_sizes.reverse();

    basin_sizes[0].1 * basin_sizes[1].1 * basin_sizes[2].1
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            ),
            15
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 524);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            ),
            1134
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1235430);
    }
}
