use std::collections::HashSet;

pub fn part_a(input: &str, dx: usize, dy: usize) -> i64 {
    let mut trees = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.trim().split('\n').enumerate() {
        max_y = y + 1;
        for (x, c) in line.chars().enumerate() {
            max_x = x + 1;
            if c == '#' {
                trees.insert((x, y));
            }
        }
    }

    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < max_y {
        if trees.contains(&(x % max_x, y)) {
            count += 1;
        }

        x += dx;
        y += dy;
    }

    count
}

pub fn part_b(input: &str) -> i64 {
    part_a(input, 1, 1)
        * part_a(input, 3, 1)
        * part_a(input, 5, 1)
        * part_a(input, 7, 1)
        * part_a(input, 1, 2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#", 3, 1), 7);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt"), 3, 1), 203);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#"), 336);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 3316272960);
    }
}
