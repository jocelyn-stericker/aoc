use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Cmd {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Cmd {
    fn get_needed_values(
        &self,
        a: usize,
        b: usize,
        reg: &Vec<usize>,
    ) -> (Option<usize>, Option<usize>) {
        match self {
            // rr
            Cmd::Addr | Cmd::Mulr | Cmd::Banr | Cmd::Borr | Cmd::Gtrr | Cmd::Eqrr => {
                (reg.get(a).map(|a| *a), reg.get(b).map(|b| *b))
            }

            // r0
            Cmd::Setr => (reg.get(a).map(|a| *a), Some(0)),
            // i0
            Cmd::Seti => (Some(a), Some(0)),

            // ir
            Cmd::Gtir | Cmd::Eqir => (Some(a), reg.get(b).map(|b| *b)),

            // ri
            Cmd::Addi | Cmd::Muli | Cmd::Bani | Cmd::Bori | Cmd::Gtri | Cmd::Eqri => {
                (reg.get(a).map(|a| *a), Some(b))
            }
        }
    }

    fn run(&self, a: usize, b: usize, c: usize, reg: &Vec<usize>) -> Option<Vec<usize>> {
        let mut reg = reg.clone();
        match (self.get_needed_values(a, b, &reg), reg.get_mut(c)) {
            ((Some(a), Some(b)), Some(c)) => {
                *c = match self {
                    Cmd::Addi | Cmd::Addr | Cmd::Seti | Cmd::Setr => a + b,
                    Cmd::Muli | Cmd::Mulr => a * b,
                    Cmd::Bani | Cmd::Banr => a & b,
                    Cmd::Bori | Cmd::Borr => a | b,
                    Cmd::Gtir | Cmd::Gtri | Cmd::Gtrr => {
                        if a > b {
                            1
                        } else {
                            0
                        }
                    }
                    Cmd::Eqir | Cmd::Eqri | Cmd::Eqrr => {
                        if a == b {
                            1
                        } else {
                            0
                        }
                    }
                };

                Some(reg)
            }
            _ => None,
        }
    }
}

enum Mode {
    Min,
    Max,
}

fn solve(input: &str, mode: Mode) -> usize {
    let mut program = vec![];
    let mut ip = None;
    for line in input.split('\n').filter(|ln| ln != &"") {
        let line: Vec<&str> = line.split(' ').collect();
        if line.len() == 2 {
            ip = Some(line[1].parse::<usize>().unwrap());
        } else {
            let opcode = match line[0] {
                "addr" => Cmd::Addr,
                "addi" => Cmd::Addi,
                "mulr" => Cmd::Mulr,
                "muli" => Cmd::Muli,
                "banr" => Cmd::Banr,
                "bani" => Cmd::Bani,
                "borr" => Cmd::Borr,
                "bori" => Cmd::Bori,
                "setr" => Cmd::Setr,
                "seti" => Cmd::Seti,
                "gtir" => Cmd::Gtir,
                "gtri" => Cmd::Gtri,
                "gtrr" => Cmd::Gtrr,
                "ewir" => Cmd::Eqir,
                "eqri" => Cmd::Eqri,
                "eqrr" => Cmd::Eqrr,
                _ => panic!("Invalid command {:?}", line),
            };

            program.push((
                opcode,
                line[1].parse::<usize>().unwrap(),
                line[2].parse::<usize>().unwrap(),
                line[3].parse::<usize>().unwrap(),
            ));
        }
    }
    let ip = ip.unwrap();

    // Run the program
    let mut reg: Vec<usize> = vec![0, 0, 0, 0, 0, 0];
    let mut last_r5 = 0;
    let mut h: HashSet<usize> = HashSet::new();

    loop {
        let line = reg[ip];
        if line == 28 {
            match mode {
                Mode::Min => {
                    return reg[5];
                }
                Mode::Max => {
                    if h.contains(&reg[5]) {
                        return last_r5;
                    }
                    h.insert(reg[5]);
                    last_r5 = reg[5];
                }
            }
        }

        if line == 17 {
            reg[4] = reg[2] / 256;
            reg[2] = reg[4];
            reg[ip] = 8;
            continue;
        }

        match program.get(line) {
            None => {
                break;
            }
            Some(cmd) => {
                reg = cmd.0.run(cmd.1, cmd.2, cmd.3, &reg).unwrap();
                reg[ip] += 1;
            }
        }
    }

    reg[0]
}

pub fn part_a(input: &str) -> usize {
    solve(input, Mode::Min)
}

pub fn part_b(input: &str) -> usize {
    solve(input, Mode::Max)
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(include_str!("input.txt")), 6778585);
}

#[test]
fn test_part_b() {
    assert_eq!(part_b(include_str!("input.txt")), 6534225);
}
