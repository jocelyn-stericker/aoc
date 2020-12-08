use std::collections::HashSet;
//

#[derive(Copy, Clone)]
enum Inst {
    Acc(i64),
    Jump(i64),
    Nop(i64),
}

fn terminates(program: &[Inst]) -> Option<i64> {
    let mut pc: i64 = 0;
    let mut acc = 0;
    let mut states = HashSet::new();
    let mut instructions = 0;
    while let Some(cmd) = program.get(pc as usize) {
        instructions += 1;
        if instructions > 100000 {
            return None;
        }
        if states.contains(&(pc, acc)) {
            return None;
        }
        states.insert((pc, acc));
        match cmd {
            Inst::Acc(number) => {
                acc += number;
                pc += 1;
            }
            Inst::Jump(number) => {
                pc += number;
            }
            Inst::Nop(_number) => {
                pc += 1;
            }
        }
    }

    Some(acc)
}

pub fn part_a(input: &str) -> i64 {
    let mut program = Vec::new();
    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let cmd = parts.next().unwrap();
        let number = parts.next().unwrap().parse::<i64>().unwrap();
        program.push(match cmd {
            "acc" => Inst::Acc(number),
            "jmp" => Inst::Jump(number),
            "nop" => Inst::Nop(number),
            _ => panic!(),
        });
    }

    let mut pc: i64 = 0;
    let mut acc = 0;
    let mut states = HashSet::new();
    while let Some(cmd) = program.get(pc as usize) {
        if states.contains(&pc) {
            return acc;
        }
        states.insert(pc);
        match cmd {
            Inst::Acc(number) => {
                acc += number;
                pc += 1;
            }
            Inst::Jump(number) => {
                pc += number;
            }
            Inst::Nop(_number) => {
                pc += 1;
            }
        }
    }
    panic!();
}

pub fn part_b(input: &str) -> i64 {
    let mut program = Vec::new();
    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let cmd = parts.next().unwrap();
        let number = parts.next().unwrap().parse::<i64>().unwrap();
        program.push(match cmd {
            "acc" => Inst::Acc(number),
            "jmp" => Inst::Jump(number),
            "nop" => Inst::Nop(number),
            _ => panic!(),
        });
    }

    for i in 0..program.len() {
        eprintln!("{}", i);
        match *program.get(i).unwrap() {
            Inst::Acc(_number) => {}
            Inst::Jump(number) => {
                program[i] = Inst::Nop(number);
                if let Some(acc) = terminates(&program) {
                    return acc;
                }
                program[i] = Inst::Jump(number);
            }
            Inst::Nop(number) => {
                program[i] = Inst::Jump(number);
                if let Some(acc) = terminates(&program) {
                    return acc;
                }
                program[i] = Inst::Nop(number);
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1137);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1125);
    }
}
