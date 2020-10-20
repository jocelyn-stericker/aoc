// use std::collections::HashSet;
use std::collections::VecDeque;

pub fn dig(plain: &str) -> String {
    format!("{:x}", md5::compute(plain))
        .chars()
        .take(4)
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    pub fn to_c(&self) -> char {
        match self {
            Dir::U => 'U',
            Dir::D => 'D',
            Dir::L => 'L',
            Dir::R => 'R',
        }
    }

    pub fn dx(&self) -> i64 {
        match self {
            Dir::U => -1,
            Dir::D => 1,
            Dir::L => 0,
            Dir::R => 0,
        }
    }
    pub fn dy(&self) -> i64 {
        match self {
            Dir::U => 0,
            Dir::D => 0,
            Dir::L => -1,
            Dir::R => 1,
        }
    }
}

pub fn iopen(c: char) -> bool {
    matches!(c, 'b' | 'c' | 'd' | 'e' | 'f')
}

fn dirs(plain: &str) -> Vec<Dir> {
    let mut dirs = Vec::new();

    let c: Vec<_> = dig(&plain).chars().collect();
    if iopen(c[0]) {
        dirs.push(Dir::U);
    }
    if iopen(c[1]) {
        dirs.push(Dir::D);
    }
    if iopen(c[2]) {
        dirs.push(Dir::L);
    }
    if iopen(c[3]) {
        dirs.push(Dir::R);
    }

    dirs
}

pub fn part_a(input: &str) -> String {
    let input = input.trim();
    let mut states = VecDeque::new();
    states.push_back((input.to_owned(), (0, 0)));

    while let Some((state, (x, y))) = states.pop_front() {
        if x == 3 && y == 3 {
            return state.chars().skip(input.len()).collect();
        }
        for dir in dirs(&state) {
            states.push_back((
                format!("{}{}", state, dir.to_c()),
                (x + dir.dx(), y + dir.dy()),
            ));
        }
    }

    panic!();
}

pub fn part_b(input: &str) -> usize {
    let input = input.trim();
    let mut states = VecDeque::new();
    states.push_back((input.to_owned(), (0, 0)));

    let mut longest = 0;

    while let Some((state, (x, y))) = states.pop_front() {
        if x < 0 || y < 0 {
            continue;
        }
        if x > 3 || y > 3 {
            continue;
        }
        if x == 3 && y == 3 {
            longest = state.len() - input.len();
            continue;
        }
        for dir in dirs(&state) {
            states.push_back((
                format!("{}{}", state, dir.to_c()),
                (x + dir.dx(), y + dir.dy()),
            ));
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        use super::Dir;
        assert_eq!(super::dirs("hijkl"), vec![Dir::U, Dir::D, Dir::L]);
        assert_eq!(super::part_a("ihgpwlah"), "DDRRRD");
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), "RLDUDRDDRR"); // 16:33
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("ihgpwlah"), 370);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 590); // 20:20
    }
}
