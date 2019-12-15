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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N = 1,
    S = 2,
    W = 3,
    E = 4,
}

impl Dir {
    fn add(&self, pos: (i64, i64)) -> (i64, i64) {
        match self {
            Dir::N => (pos.0, pos.1 + 1),
            Dir::S => (pos.0, pos.1 - 1),
            Dir::E => (pos.0 + 1, pos.1),
            Dir::W => (pos.0 - 1, pos.1),
        }
    }
    fn sub(&self, pos: (i64, i64)) -> (i64, i64) {
        match self {
            Dir::N => (pos.0, pos.1 - 1),
            Dir::S => (pos.0, pos.1 + 1),
            Dir::E => (pos.0 - 1, pos.1),
            Dir::W => (pos.0 + 1, pos.1),
        }
    }

    fn neg(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}

pub fn exec(mut ram: HashMap<usize, i64>) -> (HashMap<(i64, i64), i64>, (i64, i64)) {
    let mut cur_pos: (i64, i64) = (0, 0);
    let mut distances: HashMap<(i64, i64), i64> = HashMap::new();
    let mut ox_pos: Option<(i64, i64)> = None;
    let mut stack: Vec<Dir> = vec![];

    distances.insert((0, 0), 0);

    let mut pc = 0;
    let mut rb: i64 = 0;
    loop {
        let mod_idx;
        let mut mod_value;
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
                mod_value = None;
                for dir in &[Dir::N, Dir::S, Dir::E, Dir::W] {
                    let new_pos = dir.add(cur_pos);
                    if let Some(old_dis) = distances.get(&new_pos) {
                        if *old_dis == -1 || *old_dis <= (stack.len() as i64) + 1 {
                            continue;
                        }
                    }
                    cur_pos = new_pos;
                    stack.push(*dir);
                    mod_value = Some(*dir as i64);
                    break;
                }

                if mod_value.is_none() {
                    if stack.is_empty() {
                        break;
                    }
                    let dir = stack.pop().unwrap().neg();
                    cur_pos = dir.add(cur_pos);
                    mod_value = Some(dir as i64);
                }

                let p1 = im(&ram, pc, 1, rb);
                mod_idx = Some(p1);
                pc += 2;
            }
            // out
            4 => {
                let val = resolve(&ram, pc, 1, rb);

                if val == 0 {
                    distances.insert(cur_pos, -1);
                    cur_pos = stack.pop().unwrap().sub(cur_pos);
                } else {
                    distances.insert(cur_pos, stack.len() as i64);
                    if val == 2 {
                        ox_pos = Some(cur_pos);
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
                    assert!(p2 >= 0);
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
                    assert!(p2 >= 0);
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

    (distances, ox_pos.unwrap())
}

pub fn part_a(input: &str) -> i64 {
    let ram: HashMap<usize, i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .enumerate()
        .collect();

    let (dis, ox_pos) = exec(ram);

    dis[&ox_pos]
}

pub fn part_b(input: &str) -> i64 {
    let ram: HashMap<usize, i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .enumerate()
        .collect();

    let (dis, ox_pos) = exec(ram);

    let mut coords: HashSet<(i64, i64)> = dis
        .into_iter()
        .filter(|(_k, v)| *v != -1)
        .map(|(k, _v)| k)
        .collect();
    let mut removed = vec![ox_pos];
    coords.remove(&ox_pos);

    for i in 0.. {
        let mut removed_next = vec![];
        while let Some(removed) = removed.pop() {
            for dir in &[Dir::N, Dir::S, Dir::E, Dir::W] {
                let np = dir.add(removed);
                if coords.contains(&np) {
                    coords.remove(&np);
                    removed_next.push(np);
                }
            }
        }

        std::mem::swap(&mut removed, &mut removed_next);

        if removed.len() == 0 {
            return i;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 242);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 276);
    }
}
