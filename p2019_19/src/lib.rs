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

pub fn good(hm: &HashSet<(i64, i64)>, x: usize, y: usize) -> bool {
    for x2 in x..x + 100 {
        for y2 in y..y + 100 {
            if !hm.contains(&(x2 as i64, y2 as i64)) {
                return false;
            }
        }
    }

    true
}

pub fn exec(mut ram: HashMap<usize, i64>) -> i64 {
    let initial_ram = ram.clone();
    let mut hm = HashSet::new();
    let mut pc = 0;
    let mut rb: i64 = 0;

    let mut pos = (0, 600);
    let mut parity = 0;
    let max = 1500;

    loop {
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
                    if parity == 0 {
                        mod_value = Some(pos.0);
                        parity = 1;
                    } else {
                        mod_value = Some(pos.1);
                        parity = 0;
                    }

                    let p1 = im(&ram, pc, 1, rb);
                    mod_idx = Some(p1);
                    pc += 2;
                }
                // out
                4 => {
                    let val = resolve(&ram, pc, 1, rb);
                    // eprintln!(">> {}", val);

                    if val == 1 {
                        hm.insert(pos);
                    }

                    pos.0 += 1;
                    if pos.0 == max {
                        eprintln!("{}", pos.1);
                        for x in 0..max {
                            if hm.contains(&(x as i64, pos.1 as i64)) {
                                eprint!("#");
                            } else {
                                eprint!(".");
                            }
                        }
                        pos.0 = 0;
                        pos.1 += 1;
                    }

                    if pos.1 == max {
                        for y in 0..max {
                            for x in 0..max {
                                if good(&hm, x as usize, y as usize) {
                                    return (x * 10000 + y) as i64;
                                }
                            }
                        }
                        return 0;
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
        ram = initial_ram.clone();
        pc = 0;
    }

    panic!();
}

pub fn part_a(input: &str) -> i64 {
    let ram: HashMap<usize, i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .enumerate()
        .collect();

    exec(ram)
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn part_a() {
    //     // Not 650089
    //     assert_eq!(super::part_a(include_str!("input.txt")), 217);
    // }

    // #[test]
    // fn part_b() {
    //     assert_eq!(super::part_b(include_str!("input.txt")), 276);
    // }
}
