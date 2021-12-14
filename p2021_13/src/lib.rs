use std::collections::HashSet;

pub fn solve(input: &str) -> (usize, String) {
    let mut points: HashSet<(i64, i64)> = HashSet::new();
    let mut lines = input.trim().split('\n');

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(',').unwrap();
        points.insert((y.parse().unwrap(), x.parse().unwrap()));
    }

    let mut part_a = 0;

    for (i, line) in lines.enumerate() {
        let (_, instruction) = line.split_once("fold along ").unwrap();
        let (axis, split) = instruction.split_once('=').unwrap();
        let split: i64 = split.parse().unwrap();
        match axis {
            "x" => {
                let mut new_points = HashSet::new();
                for (y, x) in points.iter() {
                    if *x == split {
                        panic!();
                    }

                    if *x < split {
                        new_points.insert((*y, *x));
                    } else {
                        new_points.insert((*y, split - (*x - split)));
                    }
                }
                points = new_points;
            }
            "y" => {
                let mut new_points = HashSet::new();
                for (y, x) in points.iter() {
                    if *y == split {
                        panic!();
                    }

                    if *y < split {
                        new_points.insert((*y, *x));
                    } else {
                        new_points.insert((split - (*y - split), *x));
                    }
                }
                points = new_points;
            }
            _ => {
                panic!();
            }
        }

        if i == 0 {
            part_a = points.len();
        }
    }

    let mut message = String::new();
    let min_x = points.iter().map(|(_, x)| *x).min().unwrap();
    let max_x = points.iter().map(|(_, x)| *x).max().unwrap();
    let min_y = points.iter().map(|(y, _)| *y).min().unwrap();
    let max_y = points.iter().map(|(y, _)| *y).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.contains(&(y, x)) {
                message.push('#');
            } else {
                message.push('.');
            }
        }
        message.push('\n');
    }

    (part_a, message)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::solve(
                "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
            ),
            (
                17,
                "
#####
#...#
#...#
#...#
#####
"
                .trim_start()
                .to_string()
            )
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(
            super::solve(include_str!("input.txt")),
            (
                720,
                "
.##..#..#.###..###..###...##..#..#.####
#..#.#..#.#..#.#..#.#..#.#..#.#..#....#
#..#.####.#..#.#..#.#..#.#..#.#..#...#.
####.#..#.###..###..###..####.#..#..#..
#..#.#..#.#....#.#..#....#..#.#..#.#...
#..#.#..#.#....#..#.#....#..#..##..####
"
                .trim_start()
                .to_string()
            )
        );
    }
}
