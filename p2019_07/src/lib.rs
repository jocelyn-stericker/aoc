use std::collections::VecDeque;

fn resolve(ram: &Vec<i64>, pc: usize, offset: usize) -> Option<i64> {
    let opcode = *ram.get(pc)?;
    let div = (0..offset).fold(10, |div, _| div * 10);

    ram.get(pc + offset).and_then(|p1| {
        if (opcode / div) % 10 == 1 {
            Some(*p1)
        } else {
            ram.get(*p1 as usize).copied()
        }
    })
}

fn resolve_2(ram: &Vec<i64>, pc: usize) -> (Option<i64>, Option<i64>) {
    (resolve(ram, pc, 1), resolve(ram, pc, 2))
}

fn resolve_3(ram: &Vec<i64>, pc: usize) -> (Option<i64>, Option<i64>, Option<i64>) {
    (
        resolve(ram, pc, 1),
        resolve(ram, pc, 2),
        // HACK: always position mode.
        ram.get(pc + 3).copied(),
    )
}

pub fn exec(
    ram: &mut Vec<i64>,
    pc: &mut usize,
    mut input: VecDeque<i64>,
    exit_out: bool,
) -> Option<i64> {
    let mut ans = None;
    loop {
        let mod_idx;
        let mod_value;
        let opcode = ram[*pc];
        match opcode % 100 {
            // add
            1 => {
                if let (Some(p1), Some(p2), Some(p3)) = resolve_3(&ram, *pc) {
                    mod_idx = Some(p3);
                    mod_value = Some(p1 + p2);
                } else {
                    panic!("1 read past end".to_owned());
                }
                *pc += 4;
            }
            // mul
            2 => {
                if let (Some(p1), Some(p2), Some(p3)) = resolve_3(&ram, *pc) {
                    mod_idx = Some(p3);
                    mod_value = Some(p1 * p2);
                } else {
                    panic!("2 read past end".to_owned());
                }
                *pc += 4;
            }
            // in
            3 => {
                let p1 = ram.get(*pc + 1);
                mod_idx = p1.copied();
                mod_value = input.pop_front();
                *pc += 2;
            }
            // out
            4 => {
                let val = resolve(&ram, *pc, 1).unwrap();
                ans = Some(val);
                mod_idx = None;
                mod_value = None;
                *pc += 2;
                if exit_out {
                    return ans;
                }
            }
            // jump-if-true
            5 => {
                if let (Some(p1), Some(p2)) = resolve_2(&ram, *pc) {
                    if p1 != 0 {
                        assert!(p2 > 0);
                        *pc = p2 as usize;
                    } else {
                        *pc += 3;
                    }
                } else {
                    panic!("5 read past end".to_owned());
                }
                mod_idx = None;
                mod_value = None;
            }
            // jump-if-false
            6 => {
                if let (Some(p1), Some(p2)) = resolve_2(&ram, *pc) {
                    if p1 == 0 {
                        assert!(p2 > 0);
                        *pc = p2 as usize;
                    } else {
                        *pc += 3;
                    }
                } else {
                    panic!("5 read past end".to_owned());
                }
                mod_idx = None;
                mod_value = None;
            }
            // less than
            7 => {
                if let (Some(p1), Some(p2), Some(p3)) = resolve_3(&ram, *pc) {
                    mod_idx = Some(p3);
                    if p1 < p2 {
                        mod_value = Some(1);
                    } else {
                        mod_value = Some(0);
                    }
                } else {
                    panic!("1 read past end".to_owned());
                }
                *pc += 4;
            }
            // equals
            8 => {
                if let (Some(p1), Some(p2), Some(p3)) = resolve_3(&ram, *pc) {
                    mod_idx = Some(p3);
                    if p1 == p2 {
                        mod_value = Some(1);
                    } else {
                        mod_value = Some(0);
                    }
                } else {
                    panic!("1 read past end".to_owned());
                }
                *pc += 4;
            }
            // terminate
            99 => {
                break;
            }
            _ => {
                panic!("Bad opcode".to_owned());
            }
        };

        if let (Some(mod_idx), Some(mod_value)) = (mod_idx, mod_value) {
            if let Some(ram) = ram.get_mut(mod_idx as usize) {
                *ram = mod_value;
            } else {
                panic!("Write out of bounds".to_owned());
            }
        }
    }

    if exit_out {
        None
    } else {
        ans
    }
}

pub fn get_thruster(ram: Vec<i64>, pattern: Vec<i64>) -> i64 {
    let mut prev = 0;
    for p in &pattern {
        let mut x = VecDeque::new();
        x.push_back(*p);
        x.push_back(prev);
        prev = exec(&mut ram.clone(), &mut 0, x, false).unwrap();
    }

    prev
}

pub fn get_thruster_2(ram: Vec<i64>, pattern: Vec<i64>) -> i64 {
    let mut prev = 0;
    let mut rams: Vec<Vec<i64>> = (0..5).map(|_| ram.clone()).collect();
    let mut last_e = 0;
    let mut pcs: Vec<usize> = (0..5).map(|_| 0).collect();
    for i in 0.. {
        for j in 0..5 {
            let mut x = VecDeque::new();
            if i == 0 {
                x.push_back(pattern[j]);
            }

            x.push_back(prev);
            if let Some(x) = exec(&mut rams[j], &mut pcs[j], x, true) {
                if j == 4 {
                    last_e = x;
                }
                prev = x;
            } else {
                return last_e;
            }
        }
    }

    unreachable!();
}

pub fn part_a(input: &str) -> i64 {
    let ram: Vec<i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .collect();

    let mut m = 0;
    for a in 0..5 {
        for b in 0..5 {
            if b == a {
                continue;
            }

            for c in 0..5 {
                if c == b || c == a {
                    continue;
                }
                for d in 0..5 {
                    if d == c || d == b || d == a {
                        continue;
                    }
                    for e in 0..5 {
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }
                        m = m.max(get_thruster(ram.clone(), vec![a, b, c, d, e]));
                    }
                }
            }
        }
    }

    m
}

pub fn part_b(input: &str) -> i64 {
    let ram: Vec<i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .collect();

    let mut m = 0;
    for a in 5..10 {
        for b in 5..10 {
            if b == a {
                continue;
            }

            for c in 5..10 {
                if c == b || c == a {
                    continue;
                }
                for d in 5..10 {
                    if d == c || d == b || d == a {
                        continue;
                    }
                    for e in 5..10 {
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }
                        m = m.max(get_thruster_2(ram.clone(), vec![a, b, c, d, e]));
                    }
                }
            }
        }
    }

    m
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::get_thruster(
                vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                vec![4, 3, 2, 1, 0]
            ),
            43210
        );
        assert_eq!(
            super::get_thruster(
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                vec![0, 1, 2, 3, 4],
            ),
            54321
        );
        assert_eq!(
            super::get_thruster(
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                vec![1, 0, 4, 3, 2],
            ),
            65210
        );
    }
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 338603);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::get_thruster_2(
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ],
                vec![9, 8, 7, 6, 5],
            ),
            139629729
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 63103596);
    }
}
