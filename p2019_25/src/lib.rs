use std::collections::{HashMap, HashSet, VecDeque};

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

pub fn exec(mut ram: HashMap<usize, i64>) -> i64 {
    let mut pc = 0;
    let mut rb: i64 = 0;
    let mut inbuf: VecDeque<i64> = VecDeque::new();
    let mut chk: HashMap<usize, i64> = ram.clone();
    let mut chk_rb = rb;
    let mut chk_pc = pc;

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
                chk = ram.clone();
                chk_rb = rb;
                chk_pc = pc;
                if inbuf.is_empty() {
                    let mut input = String::new();
                    match std::io::stdin().read_line(&mut input) {
                        Ok(n) => {
                            for c in input.chars() {
                                inbuf.push_back(c as i64);
                            }
                        }
                        Err(error) => println!("error: {}", error),
                    }
                }
                mod_value = inbuf.pop_front();

                let p1 = im(&ram, pc, 1, rb);
                mod_idx = Some(p1);
                pc += 2;
            }
            // out
            4 => {
                let val = resolve(&ram, pc, 1, rb);

                eprint!("{}", val as u8 as char); // STOPSHIP

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
                eprintln!("Restoring from chk\n");
                ram = chk.clone();
                rb = chk_rb.clone();
                pc = chk_pc.clone();
                continue;
            }
            _ => {
                panic!();
            }
        };

        if let (Some(mod_idx), Some(mod_value)) = (mod_idx, mod_value) {
            *ram.entry(mod_idx as usize).or_default() = mod_value;
        }
    }

    0
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
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 242);
    }

    // #[test]
    // fn part_b() {
    //     assert_eq!(super::part_b(include_str!("input.txt")), 276);
    // }
}
