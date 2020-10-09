use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn turn(&self, turn: char) -> Dir {
        match turn {
            'R' => match self {
                Dir::N => Dir::E,
                Dir::E => Dir::S,
                Dir::S => Dir::W,
                Dir::W => Dir::N,
            },
            'L' => match self {
                Dir::N => Dir::W,
                Dir::E => Dir::N,
                Dir::S => Dir::E,
                Dir::W => Dir::S,
            },
            _ => panic!(),
        }
    }
}

struct Step {
    turn: char,
    amount: i64,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Delta {
    x: i64,
    y: i64,
}

impl Delta {
    fn add(&self, dir: Dir, amount: i64) -> Delta {
        match dir {
            Dir::N => Delta {
                x: self.x,
                y: self.y - amount,
            },
            Dir::S => Delta {
                x: self.x,
                y: self.y + amount,
            },
            Dir::E => Delta {
                x: self.x + amount,
                y: self.y,
            },
            Dir::W => Delta {
                x: self.x - amount,
                y: self.y,
            },
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut dir = Dir::N;
    let mut delta = Delta { x: 0, y: 0 };

    for step in input.split(", ").map(|line| {
        eprintln!("{}", line.chars().skip(1).collect::<String>());

        Step {
            turn: line.chars().next().unwrap(),
            amount: line
                .trim()
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i64>()
                .expect("Invalid number"),
        }
    }) {
        dir = dir.turn(step.turn);
        delta = delta.add(dir, step.amount);
    }

    delta.x.abs() + delta.y.abs()
}

pub fn part_b(input: &str) -> i64 {
    let mut dir = Dir::N;
    let mut delta = Delta { x: 0, y: 0 };
    let mut states = HashSet::new();
    states.insert(delta);

    for step in input.split(", ").map(|line| {
        eprintln!("{}", line.chars().skip(1).collect::<String>());

        Step {
            turn: line.chars().next().unwrap(),
            amount: line
                .trim()
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i64>()
                .expect("Invalid number"),
        }
    }) {
        dir = dir.turn(step.turn);
        for _ in 0..step.amount {
            delta = delta.add(dir, 1);
            if states.contains(&delta) {
                return delta.x.abs() + delta.y.abs();
            }
            states.insert(delta);
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("R2, L3\n"), 5);
        assert_eq!(super::part_a("R2, R2, R2\n"), 2);
        assert_eq!(super::part_a("R5, L5, R5, R3\n"), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("R8, R4, R4, R8\n"), 4);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 250); //13:18
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 151); // 18:43
    }
}
