use std::collections::HashMap;

fn resolve(ram: &HashMap<usize, i64>, pc: usize, offset: usize, rb: i64) -> i64 {
    let opcode = ram.get(&pc).copied().unwrap_or(0);
    let div = (0..offset).fold(10, |div, _| div * 10);

    let p1 = ram.get(&(pc + offset)).copied().unwrap_or(0);

    if (opcode / div) % 10 == 1 {
        p1
    } else if (opcode / div) % 10 == 2 {
        ram.get(&((p1 + rb) as usize)).copied().unwrap_or(0)
    } else {
        ram.get(&(p1 as usize)).copied().unwrap_or(0)
    }
}

fn im(ram: &HashMap<usize, i64>, pc: usize, offset: usize, rb: i64) -> i64 {
    let opcode = ram.get(&pc).copied().unwrap_or(0);
    let div = (0..offset).fold(10, |div, _| div * 10);

    let p1 = ram.get(&(pc + offset)).copied().unwrap_or(0);

    if (opcode / div) % 10 == 1 {
        panic!("Invalidx");
    } else if (opcode / div) % 10 == 2 {
        p1 + rb
    } else {
        p1
    }
}

fn resolve_2(ram: &HashMap<usize, i64>, pc: usize, rb: i64) -> (i64, i64) {
    (resolve(ram, pc, 1, rb), resolve(ram, pc, 2, rb))
}

fn resolve_3(ram: &HashMap<usize, i64>, pc: usize, rb: i64) -> (i64, i64, i64) {
    (
        resolve(ram, pc, 1, rb),
        resolve(ram, pc, 2, rb),
        im(ram, pc, 3, rb),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

pub fn exec(mut ram: HashMap<usize, i64>, play: bool) -> i64 {
    let mut blocks: HashMap<(i64, i64), Type> = Default::default();

    let mut i = -1;
    let mut x = 0;
    let mut y = 0;
    let mut score = 0;

    let mut pc = 0;
    let mut rb: i64 = 0;
    loop {
        let mod_idx;
        let mod_value;
        let opcode = ram.get(&pc).copied().unwrap_or(0);
        match opcode % 100 {
            // add
            1 => {
                let (p1, p2, p3) = resolve_3(&ram, pc, rb);

                mod_idx = Some(p3);
                mod_value = Some(p1 + p2);

                pc += 4;
            }
            // mul
            2 => {
                let (p1, p2, p3) = resolve_3(&ram, pc, rb);

                mod_idx = Some(p3);
                mod_value = Some(p1 * p2);

                pc += 4;
            }
            // in
            3 => {
                // for i in 0..60 {
                //     for j in 0..60 {
                //         match blocks.get(&(i, j)) {
                //             None => eprint!(" "),
                //             Some(Type::Wall) => eprint!("#"),
                //             Some(Type::Block) => eprint!("x"),
                //             Some(Type::Paddle) => eprint!("|"),
                //             Some(Type::Ball) => eprint!("o"),
                //         }
                //     }
                //     eprintln!();
                // }

                let ball_pos = blocks.iter().find(|(_, v)| **v == Type::Ball).unwrap().0;
                let paddle_pos = blocks.iter().find(|(_, v)| **v == Type::Paddle).unwrap().0;
                if ball_pos.0 > paddle_pos.0 {
                    mod_value = Some(1);
                } else if ball_pos.0 < paddle_pos.0 {
                    mod_value = Some(-1)
                } else {
                    mod_value = Some(0)
                }

                let p1 = im(&ram, pc, 1, rb);
                mod_idx = Some(p1);
                pc += 2;
            }
            // out
            4 => {
                let val = resolve(&ram, pc, 1, rb);

                i = (i + 1) % 3;
                if i == 0 {
                    x = val;
                }

                if i == 1 {
                    y = val;
                }

                if i == 2 {
                    let t = val;
                    if x == -1 && y == 0 && play {
                        score = t;
                    }

                    if t == 1 {
                        blocks.insert((x, y), Type::Wall);
                    } else if t == 2 {
                        blocks.insert((x, y), Type::Block);
                    } else if t == 3 {
                        blocks.insert((x, y), Type::Paddle);
                    } else if t == 4 {
                        blocks.insert((x, y), Type::Ball);
                    } else {
                        blocks.remove(&(x, y));
                    }
                }

                mod_idx = None;
                mod_value = None;
                pc += 2;
            }
            // jump-if-true
            5 => {
                let (p1, p2) = resolve_2(&ram, pc, rb);
                if p1 != 0 {
                    assert!(p2 > 0);
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }

                mod_idx = None;
                mod_value = None;
            }
            // jump-if-false
            6 => {
                let (p1, p2) = resolve_2(&ram, pc, rb);
                if p1 == 0 {
                    assert!(p2 > 0);
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }

                mod_idx = None;
                mod_value = None;
            }
            // less than
            7 => {
                let (p1, p2, p3) = resolve_3(&ram, pc, rb);
                mod_idx = Some(p3);
                if p1 < p2 {
                    mod_value = Some(1);
                } else {
                    mod_value = Some(0);
                }

                pc += 4;
            }
            // equals
            8 => {
                let (p1, p2, p3) = resolve_3(&ram, pc, rb);
                mod_idx = Some(p3);
                if p1 == p2 {
                    mod_value = Some(1);
                } else {
                    mod_value = Some(0);
                }
                pc += 4;
            }
            // rb
            9 => {
                rb += resolve(&ram, pc, 1, rb);
                mod_idx = None;
                mod_value = None;
                pc += 2;
            }
            // terminate
            99 => {
                break;
            }
            _ => {
                panic!();
            }
        };

        if let (Some(mod_idx), Some(mod_value)) = (mod_idx, mod_value) {
            *ram.entry(mod_idx as usize).or_default() = mod_value;
        }
    }

    if play {
        score
    } else {
        blocks
            .values()
            .filter(|&&v| v == Type::Block)
            .collect::<Vec<_>>()
            .len() as i64
    }
}

pub fn part_a(input: &str) -> i64 {
    let ram: HashMap<usize, i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .enumerate()
        .collect();

    exec(ram, false)
}

pub fn part_b(input: &str) -> i64 {
    let mut ram: HashMap<usize, i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .enumerate()
        .collect();

    ram.insert(0, 2);

    exec(ram, true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 255);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 12338);
    }
}
