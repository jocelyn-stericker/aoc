use std::collections::HashSet;

pub fn print(paper: &HashSet<(i64, i64)>) -> String {
    let min_y = paper.iter().map(|p| p.0).min().unwrap();
    let min_x = paper.iter().map(|p| p.1).min().unwrap();
    let max_y = paper.iter().map(|p| p.0).max().unwrap();
    let max_x = paper.iter().map(|p| p.1).max().unwrap();
    let mut s = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if paper.contains(&(y, x)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    s
}

pub fn solve(input: &str) -> (usize, String) {
    let mut paper: HashSet<(i64, i64)> = HashSet::new();
    let mut lines = input.trim().split('\n');
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(',').unwrap();
        paper.insert((y.parse().unwrap(), x.parse().unwrap()));
    }

    let mut part_a = 0;

    for (y, line) in lines.enumerate() {
        let (_, split) = line.split_once("fold along ").unwrap();
        let (axis, pos) = split.split_once('=').unwrap();
        let pos: i64 = pos.parse().unwrap();
        match axis {
            "x" => {
                let mut new_paper = HashSet::new();
                for (y, x) in paper.iter() {
                    assert!(*x != pos);
                    if *x < pos {
                        new_paper.insert((*y, *x));
                    } else {
                        new_paper.insert((*y, pos - (*x - pos)));
                    }
                }
                paper = new_paper;
            }
            "y" => {
                let mut new_paper = HashSet::new();
                for (y, x) in paper.iter() {
                    assert!(*y != pos);
                    if *y < pos {
                        new_paper.insert((*y, *x));
                    } else {
                        new_paper.insert((pos - (*y - pos), *x));
                    }
                }
                paper = new_paper;
            }
            _ => {
                panic!();
            }
        }
        if y == 0 {
            part_a = paper.len()
        }
    }

    (part_a, print(&paper))
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
fold along x=5",
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
    fn solve() {
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
