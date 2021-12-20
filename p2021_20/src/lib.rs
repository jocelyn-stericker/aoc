use std::collections::HashSet;

fn enhance(on: HashSet<(i64, i64)>, algorithm: &[char], border: bool) -> HashSet<(i64, i64)> {
    let mut next_on = HashSet::new();

    let min_x = on.iter().map(|(_, x)| *x).min().unwrap();
    let min_y = on.iter().map(|(y, _)| *y).min().unwrap();
    let max_x = on.iter().map(|(_, x)| *x).max().unwrap();
    let max_y = on.iter().map(|(y, _)| *y).max().unwrap();

    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let mut num = String::new();
            for y2 in y - 1..=y + 1 {
                for x2 in x - 1..=x + 1 {
                    if x2 < min_x || y2 < min_y || x2 > max_x || y2 > max_y {
                        if border {
                            num.push('1');
                        } else {
                            num.push('0');
                        }
                    } else if on.contains(&(y2, x2)) {
                        num.push('1');
                    } else {
                        num.push('0');
                    }
                }
            }
            let num = usize::from_str_radix(&num, 2).unwrap();
            match algorithm[num] {
                '#' => {
                    next_on.insert((y, x));
                }
                '.' => {}
                _ => panic!(),
            }
        }
    }

    next_on
}

fn _print(on: &HashSet<(i64, i64)>) -> String {
    let min_x = on.iter().map(|(_, x)| *x).min().unwrap();
    let min_y = on.iter().map(|(y, _)| *y).min().unwrap();
    let max_x = on.iter().map(|(_, x)| *x).max().unwrap();
    let max_y = on.iter().map(|(y, _)| *y).max().unwrap();

    let mut s = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if on.contains(&(y as i64, x as i64)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }

    s
}

pub fn solve(input: &str, iter: usize) -> usize {
    let mut lines = input.trim().split('\n');
    let algorithm: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut on = HashSet::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                on.insert((y as i64, x as i64));
            }
        }
    }

    let mut border = false;
    let should_swap = algorithm[0] == '#';
    for _ in 0..iter {
        on = enhance(on, &algorithm, border);
        if should_swap {
            border = !border;
        }
    }

    // eprintln!("{}\n", print(&on));
    on.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::solve("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###", 2), 35);
    }

    #[test]
    fn example2() {
        assert_eq!(super::solve("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###", 50), 3351);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::solve(include_str!("input.txt"), 2), 5583);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::solve(include_str!("input.txt"), 50), 19592);
    }
}
