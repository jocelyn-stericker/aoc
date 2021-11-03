use std::collections::HashSet;

pub fn part_a(input: &str) -> usize {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut s = HashSet::new();
    s.insert((0, 0));
    for c in input.trim().chars() {
        match c {
            '^' => {
                y -= 1;
            }
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            'v' => {
                y += 1;
            }
            _ => panic!(),
        }
        s.insert((x, y));
    }
    s.len()
}

pub fn part_b(input: &str) -> usize {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut rx: i64 = 0;
    let mut ry: i64 = 0;
    let mut s = HashSet::new();
    s.insert((0, 0));
    for (i, c) in input.trim().chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '^' => {
                    y -= 1;
                }
                '<' => {
                    x -= 1;
                }
                '>' => {
                    x += 1;
                }
                'v' => {
                    y += 1;
                }
                _ => panic!(),
            }
        } else {
            match c {
                '^' => {
                    ry -= 1;
                }
                '<' => {
                    rx -= 1;
                }
                '>' => {
                    rx += 1;
                }
                'v' => {
                    ry += 1;
                }
                _ => panic!(),
            }
        }
        s.insert((x, y));
        s.insert((rx, ry));
    }
    s.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2081);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2341);
    }
}
