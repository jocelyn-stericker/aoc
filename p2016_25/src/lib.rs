use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Copy, Clone)]
enum Src {
    Const(i64),
    Reg(char),
}

#[derive(Debug, Copy, Clone)]
enum Cmd {
    Cpy(Src, Src),
    Inc(char),
    Dec(char),
    Jnz(Src, Src),
    Tgl(char),
    Out(char),
}

pub fn has_signal(input: &str, a: i64) -> bool {
    let mut prog = Vec::new();
    for line in input.trim().split('\n') {
        let line: Vec<_> = line.trim().split(' ').collect();
        prog.push(match line[0] {
            "cpy" => Cmd::Cpy(
                if let Ok(c) = line[1].parse::<i64>() {
                    Src::Const(c)
                } else {
                    Src::Reg(line[1].chars().next().unwrap())
                },
                if let Ok(c) = line[2].parse::<i64>() {
                    Src::Const(c)
                } else {
                    Src::Reg(line[2].chars().next().unwrap())
                },
            ),
            "inc" => Cmd::Inc(line[1].chars().next().unwrap()),
            "dec" => Cmd::Dec(line[1].chars().next().unwrap()),
            "jnz" => Cmd::Jnz(
                if let Ok(c) = line[1].parse::<i64>() {
                    Src::Const(c)
                } else {
                    Src::Reg(line[1].chars().next().unwrap())
                },
                if let Ok(c) = line[2].parse::<i64>() {
                    Src::Const(c)
                } else {
                    Src::Reg(line[2].chars().next().unwrap())
                },
            ),
            "tgl" => Cmd::Tgl(line[1].chars().next().unwrap()),
            "out" => Cmd::Out(line[1].chars().next().unwrap()),
            _ => panic!(),
        });
    }

    let mut reg = BTreeMap::new();
    reg.insert('a', a);
    reg.insert('b', 0);
    reg.insert('c', 0);
    reg.insert('d', 0);

    let mut pc = 0;
    let mut states = BTreeSet::new();

    let mut next_expected = 0;
    let mut found_loop = false;

    loop {
        let mut state = (reg.clone(), pc);
        if states.contains(&state) {
            found_loop = true;
        }
        states.insert(state);
        let mut change = None;
        match prog.get(pc) {
            Some(Cmd::Cpy(Src::Const(i), Src::Reg(dst))) => {
                reg.insert(*dst, *i);
            }
            Some(Cmd::Cpy(Src::Const(_), Src::Const(_))) => {
                //
            }
            Some(Cmd::Cpy(Src::Reg(c), Src::Reg(dst))) => {
                let i = *reg.get(c).unwrap();
                reg.insert(*dst, i);
            }
            Some(Cmd::Cpy(Src::Reg(_), Src::Const(_))) => {
                //
            }
            Some(Cmd::Inc(dst)) => {
                let i = *reg.get(dst).unwrap();
                reg.insert(*dst, i + 1);
            }
            Some(Cmd::Dec(dst)) => {
                let i = *reg.get(dst).unwrap();
                reg.insert(*dst, i - 1);
            }
            Some(Cmd::Jnz(Src::Const(i), Src::Const(dst))) => {
                if *i != 0 {
                    pc = (pc as i64 + *dst) as usize;
                    continue;
                }
            }
            Some(Cmd::Jnz(Src::Reg(c), Src::Const(dst))) => {
                let i = *reg.get(c).unwrap();
                if i != 0 {
                    pc = (pc as i64 + *dst) as usize;
                    continue;
                }
            }
            Some(Cmd::Jnz(Src::Const(i), Src::Reg(dst))) => {
                let dst = *reg.get(dst).unwrap();
                if *i != 0 {
                    pc = (pc as i64 + dst) as usize;
                    continue;
                }
            }
            Some(Cmd::Jnz(Src::Reg(c), Src::Reg(dst))) => {
                let dst = *reg.get(dst).unwrap();
                let i = *reg.get(c).unwrap();
                if i != 0 {
                    pc = (pc as i64 + dst) as usize;
                    continue;
                }
            }
            Some(Cmd::Tgl(c)) => {
                let i = *reg.get(c).unwrap();
                let j = i + (pc as i64);
                if j >= 0 {
                    if let Some(cmd) = prog.get(j as usize) {
                        change = Some((
                            j,
                            match cmd {
                                Cmd::Cpy(a, b) => Cmd::Jnz(*a, *b),
                                Cmd::Inc(a) => Cmd::Dec(*a),
                                Cmd::Dec(a) => Cmd::Inc(*a),
                                Cmd::Jnz(a, b) => Cmd::Cpy(*a, *b),
                                Cmd::Tgl(a) => Cmd::Inc(*a),
                                Cmd::Out(a) => todo!(),
                            },
                        ));
                    }
                }
            }
            Some(Cmd::Out(c)) => {
                let v = reg[c];
                if v != next_expected {
                    return false;
                }
                if found_loop {
                    return true;
                }
                next_expected = if next_expected == 0 { 1 } else { 0 };
            }
            None => break,
        }

        if let Some((i, inst)) = change {
            if i >= 0 && (i as usize) < prog.len() {
                prog[i as usize] = inst;
            }
        }

        pc += 1;
    }

    panic!();
}

pub fn solve(input: &str) -> i64 {
    for i in 1.. {
        eprintln!("{:?}", i);
        if has_signal(input, i) {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::solve(include_str!("input.txt")), 198);
    }
}
