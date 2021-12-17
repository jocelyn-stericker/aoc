pub fn hits_target(
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    mut dx: i64,
    mut dy: i64,
) -> Option<i64> {
    let mut highest_point = 0;
    let mut x = 0;
    let mut y = 0;

    loop {
        x += dx;
        y += dy;
        if dx > 0 {
            dx -= 1;
        } else if dx < 0 {
            dx += 1;
        }
        dy -= 1;

        highest_point = highest_point.max(y);

        if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
            return Some(highest_point);
        }

        if x > x_max && y > y_max {
            return None;
        }

        if dx == 0 && !(x >= x_min && x <= x_max) {
            return None;
        }

        if dy < 0 && y < y_min {
            return None;
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let (x_s, y_s) = input
        .trim()
        .trim_start_matches("target area: ")
        .split_once(", ")
        .unwrap();
    let (x_min, x_max) = x_s.trim_start_matches("x=").split_once("..").unwrap();
    let (y_min, y_max) = y_s.trim_start_matches("y=").split_once("..").unwrap();

    let x_min: i64 = x_min.parse().unwrap();
    let y_min: i64 = y_min.parse().unwrap();
    let x_max: i64 = x_max.parse().unwrap();
    let y_max: i64 = y_max.parse().unwrap();

    let mut max_y_for_best_solution = 0;
    for dx in -1000..=1000 {
        for dy in -1000..=1000 {
            if let Some(max_y) = hits_target(x_min, x_max, y_min, y_max, dx, dy) {
                max_y_for_best_solution = max_y_for_best_solution.max(max_y);
            }
        }
    }
    max_y_for_best_solution
}

pub fn part_b(input: &str) -> i64 {
    let (x_s, y_s) = input
        .trim()
        .trim_start_matches("target area: ")
        .split_once(", ")
        .unwrap();
    let (x_min, x_max) = x_s.trim_start_matches("x=").split_once("..").unwrap();
    let (y_min, y_max) = y_s.trim_start_matches("y=").split_once("..").unwrap();

    let x_min: i64 = x_min.parse().unwrap();
    let y_min: i64 = y_min.parse().unwrap();
    let x_max: i64 = x_max.parse().unwrap();
    let y_max: i64 = y_max.parse().unwrap();

    let mut solutions = 0;
    for dx in -1000..=1000 {
        for dy in -1000..=1000 {
            if hits_target(x_min, x_max, y_min, y_max, dx, dy).is_some() {
                solutions += 1;
            }
        }
    }
    solutions
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("target area: x=20..30, y=-10..-5\n"), 45);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 4656);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("target area: x=20..30, y=-10..-5\n"), 112);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1908);
    }
}
