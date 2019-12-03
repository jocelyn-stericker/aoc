use std::collections::HashMap;

#[derive(Debug)]
enum Cmd {
    Right(i32),
    Up(i32),
    Down(i32),
    Left(i32),
}

impl Cmd {
    fn direction(&self) -> (i32, i32) {
        match self {
            Cmd::Right(_) => (0, 1),
            Cmd::Left(_) => (0, -1),
            Cmd::Up(_) => (1, 0),
            Cmd::Down(_) => (-1, 0),
        }
    }

    fn amount(&self) -> i32 {
        match self {
            Cmd::Right(x) | Cmd::Left(x) | Cmd::Up(x) | Cmd::Down(x) => *x,
        }
    }
}

fn parse_line(input: &str) -> Vec<Cmd> {
    input
        .split(',')
        .filter(|line| line != &"")
        .map(|line| {
            let amt = line[1..].parse::<i32>().expect("Invalid number");
            match &line[0..1] {
                "R" => Cmd::Right(amt),
                "U" => Cmd::Up(amt),
                "L" => Cmd::Left(amt),
                "D" => Cmd::Down(amt),
                _ => panic!(),
            }
        })
        .collect()
}

fn points(cmds: &Vec<Cmd>) -> HashMap<(i32, i32), i32> {
    let mut points = HashMap::new();

    let mut pos = (0, 0);
    let mut steps = 0;
    for cmd in cmds {
        let dir = cmd.direction();
        for _ in 0..cmd.amount() {
            steps += 1;
            pos.0 += dir.0;
            pos.1 += dir.1;
            points.insert(pos, steps);
        }
    }

    points
}

pub fn part_a(input: &str) -> i32 {
    let mut lines = input.split('\n');
    let a = points(&parse_line(lines.next().unwrap()));
    let b = points(&parse_line(lines.next().unwrap()));

    let mut min_dis = std::i32::MAX;

    for pt in a.keys() {
        if b.contains_key(pt) {
            min_dis = min_dis.min(pt.0.abs() + pt.1.abs());
        }
    }

    min_dis
}

pub fn part_b(input: &str) -> i32 {
    let mut lines = input.split('\n');
    let a = points(&parse_line(lines.next().unwrap()));
    let b = points(&parse_line(lines.next().unwrap()));

    let mut min_dis = std::i32::MAX;

    for pt in a.keys() {
        if b.contains_key(pt) {
            min_dis = min_dis.min(a[pt] + b[pt]);
        }
    }

    min_dis
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n"),
            159
        );
        assert_eq!(super::part_a("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n"), 135);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n"),
            610
        );
        assert_eq!(super::part_b("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n"), 410);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 221);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 18542);
    }
}
