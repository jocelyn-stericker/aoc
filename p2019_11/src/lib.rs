use std::collections::{HashMap, HashSet};

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

#[derive(Copy, Clone, Debug)]
enum Dir {
    L,
    U,
    D,
    R,
}

impl Dir {
    fn left(self) -> Dir {
        match self {
            Dir::L => Dir::D,
            Dir::D => Dir::R,
            Dir::R => Dir::U,
            Dir::U => Dir::L,
        }
    }
    fn right(self) -> Dir {
        match self {
            Dir::L => Dir::U,
            Dir::D => Dir::L,
            Dir::R => Dir::D,
            Dir::U => Dir::R,
        }
    }
    fn coord(self) -> (i64, i64) {
        match self {
            Dir::L => (-1, 0),
            Dir::D => (0, -1),
            Dir::R => (1, 0),
            Dir::U => (0, 1),
        }
    }
}

pub fn exec(
    mut ram: HashMap<usize, i64>,
    start: i64,
) -> (HashSet<(i64, i64)>, HashSet<(i64, i64)>) {
    let mut painted: HashSet<(i64, i64)> = Default::default();
    let mut world: HashSet<(i64, i64)> = Default::default();
    let mut pos: (i64, i64) = (0, 0);
    let mut dir: Dir = Dir::U;
    let mut is_paint = true;

    if start == 1 {
        world.insert(pos);
    }

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
                let p1 = im(&ram, pc, 1, rb);
                mod_idx = Some(p1);
                mod_value = Some(if world.contains(&pos) { 1 } else { 0 });
                // eprintln!("{:?} {:?}", opcode, mod_value);
                pc += 2;
            }
            // out
            4 => {
                let val = resolve(&ram, pc, 1, rb);
                if is_paint {
                    if val == 0 {
                        world.remove(&pos);
                    } else if val == 1 {
                        world.insert(pos);
                    } else {
                        panic!();
                    }
                    painted.insert(pos);
                } else {
                    if val == 0 {
                        dir = dir.left();
                    } else if val == 1 {
                        dir = dir.right();
                    } else {
                        panic!();
                    }
                    let (x, y) = dir.coord();
                    pos.0 += x;
                    pos.1 += y;
                }
                is_paint = !is_paint;
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

    (world, painted)
}

pub fn part_a(input: &str) -> usize {
    let ram: HashMap<usize, i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .enumerate()
        .collect();

    exec(ram, 0).1.len()
}

pub fn part_b(input: &str) -> String {
    let ram: HashMap<usize, i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .enumerate()
        .collect();

    let pic = exec(ram, 1).0;
    let min_x = pic.iter().map(|pt| pt.0).min().unwrap();
    let min_y = pic.iter().map(|pt| pt.1).min().unwrap();
    let max_x = pic.iter().map(|pt| pt.0).max().unwrap();
    let max_y = pic.iter().map(|pt| pt.1).max().unwrap();

    let mut soln = String::new();

    for j in min_y..=max_y {
        for i in min_x..=max_x {
            if pic.contains(&(i, min_y - j)) {
                soln += "#";
            } else {
                soln += " ";
            }
        }
        soln += "\n";
    }

    soln
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2018);
    }

    #[test]
    fn part_b() {
        assert_eq!(
            super::part_b(include_str!("input.txt")),
            " ##  ###  #### #  # ###  #  # ###  ### \n\
             #  # #  # #    # #  #  # # #  #  # #  #\n\
             #  # #  # ###  ##   #  # ##   ###  #  #\n\
             #### ###  #    # #  ###  # #  #  # ### \n\
             #  # #    #    # #  # #  # #  #  # # # \n\
             #  # #    #    #  # #  # #  # ###  #  #\n"
        );
    }
}
