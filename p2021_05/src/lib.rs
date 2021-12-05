use std::collections::HashMap;

pub fn solve(input: &str, consider_diag: bool) -> i64 {
    let mut num_lines: HashMap<(i64, i64), i64> = HashMap::new();

    for line in input.trim().split('\n') {
        let mut parts = line.split(" -> ");
        let start: Vec<i64> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect();
        let end: Vec<i64> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect();

        let x0 = start[0];
        let y0 = start[1];
        let x1 = end[0];
        let y1 = end[1];
        if x0 == x1 {
            for y in y0.min(y1)..=y0.max(y1) {
                *num_lines.entry((x0, y)).or_default() += 1;
            }
        } else if y0 == y1 {
            for x in x0.min(x1)..=x0.max(x1) {
                *num_lines.entry((x, y0)).or_default() += 1;
            }
        } else if consider_diag {
            let dx = if x1 - x0 > 0 { 1 } else { -1 };
            let dy = if y1 - y0 > 0 { 1 } else { -1 };
            let mut x = x0;
            let mut y = y0;
            *num_lines.entry((x, y)).or_default() += 1;
            while x != x1 {
                x += dx;
                y += dy;
                *num_lines.entry((x, y)).or_default() += 1;
            }
            assert!(y == y1);
        }
    }

    num_lines.values().filter(|f| **f >= 2).count() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::solve(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
                false
            ),
            5
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::solve(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
                true
            ),
            12
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::solve(include_str!("input.txt"), false), 5084);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::solve(include_str!("input.txt"), true), 17882);
    }
}
