fn resolve(ram: &Vec<i64>, pc: usize, offset: usize) -> Option<i64> {
    let opcode = *ram.get(pc)?;
    let div = (0..offset).fold(10, |div, _| div * 10);
    eprintln!("{} {}", div, offset);

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

pub fn exec(system: i64, mut ram: Vec<i64>) -> Result<i64, String> {
    let mut pc = 0;
    let mut ans = None;
    let mut chk_zero = if system == 1 { false } else { true };
    loop {
        let mod_idx;
        let mod_value;
        let opcode = ram[pc];
        match opcode % 100 {
            // add
            1 => {
                if let (Some(p1), Some(p2), Some(p3)) = resolve_3(&ram, pc) {
                    mod_idx = Some(p3);
                    mod_value = Some(p1 + p2);
                } else {
                    return Err("1 read past end".to_owned());
                }
                pc += 4;
            }
            // mul
            2 => {
                if let (Some(p1), Some(p2), Some(p3)) = resolve_3(&ram, pc) {
                    mod_idx = Some(p3);
                    mod_value = Some(p1 * p2);
                } else {
                    return Err("2 read past end".to_owned());
                }
                pc += 4;
            }
            // in
            3 => {
                let p1 = ram.get(pc + 1);
                mod_idx = p1.copied();
                mod_value = Some(system);
                eprintln!("{:?} {:?}", opcode, mod_value);
                pc += 2;
            }
            // out
            4 => {
                let val = resolve(&ram, pc, 1).unwrap();
                if !chk_zero {
                    if val == 0 {
                        chk_zero = true;
                    } else {
                        panic!("bad check");
                    }
                } else {
                    ans = Some(val);
                }
                mod_idx = None;
                mod_value = None;
                pc += 2;
            }
            // jump-if-true
            5 => {
                if let (Some(p1), Some(p2)) = resolve_2(&ram, pc) {
                    if p1 != 0 {
                        assert!(p2 > 0);
                        pc = p2 as usize;
                    } else {
                        pc += 3;
                    }
                } else {
                    return Err("5 read past end".to_owned());
                }
                mod_idx = None;
                mod_value = None;
            }
            // jump-if-false
            6 => {
                if let (Some(p1), Some(p2)) = resolve_2(&ram, pc) {
                    if p1 == 0 {
                        assert!(p2 > 0);
                        pc = p2 as usize;
                    } else {
                        pc += 3;
                    }
                } else {
                    return Err("5 read past end".to_owned());
                }
                mod_idx = None;
                mod_value = None;
            }
            // less than
            7 => {
                if let (Some(p1), Some(p2), Some(p3)) = resolve_3(&ram, pc) {
                    mod_idx = Some(p3);
                    if p1 < p2 {
                        mod_value = Some(1);
                    } else {
                        mod_value = Some(0);
                    }
                } else {
                    return Err("1 read past end".to_owned());
                }
                pc += 4;
            }
            // equals
            8 => {
                if let (Some(p1), Some(p2), Some(p3)) = resolve_3(&ram, pc) {
                    mod_idx = Some(p3);
                    if p1 == p2 {
                        mod_value = Some(1);
                    } else {
                        mod_value = Some(0);
                    }
                } else {
                    return Err("1 read past end".to_owned());
                }
                pc += 4;
            }
            // terminate
            99 => {
                break;
            }
            _ => {
                return Err("Bad opcode".to_owned());
            }
        };

        if let (Some(mod_idx), Some(mod_value)) = (mod_idx, mod_value) {
            if let Some(ram) = ram.get_mut(mod_idx as usize) {
                *ram = mod_value;
            } else {
                return Err("Write out of bounds".to_owned());
            }
        }
    }

    Ok(ans.unwrap())
}

pub fn part_a(input: &str) -> Result<i64, String> {
    let ram: Vec<i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .collect();

    exec(1, ram)
}

pub fn part_b(input: &str) -> Result<i64, String> {
    let ram: Vec<i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .collect();

    exec(5, ram)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_ne!(super::part_a(include_str!("input.txt")), Ok(224));
        assert_ne!(super::part_a(include_str!("input.txt")), Ok(223));
        assert_eq!(super::part_a(include_str!("input.txt")), Ok(4511442));
    }

    #[test]
    fn example() {
        assert_eq!(
            super::part_b("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"),
            Ok(1)
        );
        assert_eq!(super::part_b("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), Ok(1));
        assert_eq!(super::part_b("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), Ok(999));
    }

    #[test]
    fn part_b() {
        assert_ne!(super::part_b(include_str!("input.txt")), Ok(13561141));
        assert_eq!(super::part_b(include_str!("input.txt")), Ok(12648139));
    }
}
