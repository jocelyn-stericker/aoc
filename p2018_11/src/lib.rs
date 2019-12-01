fn power_level(pos: &(i64, i64), serial: i64) -> i64 {
    let rack_id = pos.0 + 10;
    (((rack_id * pos.1 + serial) * rack_id / 100) % 10) - 5
}

fn get_grid(serial: i64) -> Vec<Vec<i64>> {
    (1..=300)
        .map(|x| (1..=300).map(|y| power_level(&(x, y), serial)).collect())
        .collect()
}

// Returns (value, (x, y))
pub fn part_a(serial: i64) -> (i64, (usize, usize)) {
    let grid = get_grid(serial);
    (1..=300 - 2)
        .map(|x| {
            (1..=300 - 2)
                .map(|y| {
                    (
                        (x..x + 3)
                            .fold(0, |m, x| (y..y + 3).fold(m, |m, y| m + grid[x - 1][y - 1])),
                        (x, y),
                    )
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

// Returns (value, (x, y), size)
pub fn part_b(serial: i64) -> (i64, (usize, usize), usize) {
    let grid = get_grid(serial);
    let mut prev_totals: Vec<Vec<i64>> = (1..=300)
        .map(|_x| (1..=300).map(|_y| 0).collect())
        .collect();
    let mut best = (-10000, (0, 0), 0);
    for size in 1..=300 {
        let next_totals: Vec<Vec<i64>> = (1..=300 - (size - 1))
            .map(|x| {
                (1..=300 - (size - 1))
                    .map(|y| {
                        let r = prev_totals[x - 1][y - 1]
                            + (x..x + size).fold(0, |m, x2| m + grid[x2 - 1][y + size - 2])
                            + (y..y + size - 1).fold(0, |m, y2| m + grid[x + size - 2][y2 - 1]);

                        let sig = (r, (x, y), size);
                        best = std::cmp::max(sig, best);
                        r
                    })
                    .collect()
            })
            .collect();

        prev_totals = next_totals;
    }

    best
}

#[test]
fn sample_power_level() {
    assert_eq!(power_level(&(3, 5), 8), 4);
    assert_eq!(power_level(&(122, 79), 57), -5);
    assert_eq!(power_level(&(217, 196), 39), 0);
    assert_eq!(power_level(&(101, 153), 71), 4);
}

#[test]
fn sample_part_a() {
    assert_eq!(part_a(18), (29, (33, 45)));
    assert_eq!(part_a(42), (30, (21, 61)));
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(5177), (30, (235, 22)));
}

#[test]
fn sample_part_b() {
    assert_eq!(part_b(18), (113, (90, 269), 16));
    assert_eq!(part_b(42), (119, (232, 251), 12));
}

#[test]
fn test_part_b() {
    assert_eq!(part_b(5177), (80, (231, 135), 8));
}
