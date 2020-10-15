use std::collections::{HashSet, VecDeque};

fn is_wall(x: i64, y: i64, fav: i64) -> bool {
    let mut yolo = x * x + 3 * x + 2 * x * y + y + y * y + fav;
    let mut ones = 0;
    while yolo > 0 {
        if yolo % 2 == 1 {
            ones += 1;
        }
        yolo /= 2;
    }

    ones % 2 == 1
}

pub fn part_a(input: &str) -> i64 {
    let fav: i64 = input.trim().parse().unwrap();
    let mut walls = HashSet::new();
    for x in 0..1000 {
        for y in 0..1000 {
            if is_wall(x, y, fav) {
                walls.insert((x, y));
            }
        }
    }

    let mut q = VecDeque::new();
    q.push_back((1, 1, 0));

    let mut visited = HashSet::new();
    visited.insert((1, 1));

    while let Some((x, y, steps)) = q.pop_front() {
        if x == 31 && y == 39 {
            return steps;
        }
        eprintln!("{:?} {:?}", x, y);
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].iter() {
            let nx = *nx;
            let ny = *ny;
            if nx >= 0
                && ny >= 0
                && nx < 1000
                && ny < 1000
                && !walls.contains(&(nx, ny))
                && !visited.contains(&(nx, ny))
            {
                q.push_back((nx, ny, steps + 1));
                visited.insert((nx, ny));
            }
        }
    }

    panic!();
}

pub fn part_b(input: &str) -> i64 {
    let fav: i64 = input.trim().parse().unwrap();
    let mut walls = HashSet::new();
    for x in 0..1000 {
        for y in 0..1000 {
            if is_wall(x, y, fav) {
                walls.insert((x, y));
            }
        }
    }

    let mut q = VecDeque::new();
    q.push_back((1, 1, 0));

    let mut visited = HashSet::new();
    visited.insert((1, 1));

    while let Some((x, y, steps)) = q.pop_front() {
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].iter() {
            let nx = *nx;
            let ny = *ny;
            if nx >= 0
                && ny >= 0
                && nx < 1000
                && ny < 1000
                && !walls.contains(&(nx, ny))
                && !visited.contains(&(nx, ny))
                && steps < 50
            {
                q.push_back((nx, ny, steps + 1));
                visited.insert((nx, ny));
            }
        }
    }

    visited.len() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::is_wall(0, 0, 10), false);
        assert_eq!(super::is_wall(1, 0, 10), true);
        assert_eq!(super::is_wall(0, 0, 10), false);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 86); // 12:43
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 127); // 18:28
    }
}
