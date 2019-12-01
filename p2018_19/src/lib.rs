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

pub fn solve(input: &str, reg_z: usize) -> usize {
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
    let mut reg: Vec<usize> = vec![reg_z, 0, 0, 0, 0, 0];

    loop {
        let line = reg[ip];
        if reg_z == 1 && line == 2 && reg[0] > 0 {
            if reg[4] % reg[2] == 0 {
                reg[0] += reg[2];
            }
            reg[1] = 1;
            reg[ip] = 12;
            continue;
        }

        match program.get(line) {
            None => return reg[0],
            Some(cmd) => {
                // println!("{:?}", reg);
                reg = cmd.0.run(cmd.1, cmd.2, cmd.3, &reg).unwrap();
                reg[ip] += 1;
            }
        }

        if line == 7 {
            println!("> {:?}", reg);
            println!("{}", reg[2]);
        }
    }
}

#[test]
fn test_sample() {
    assert_eq!(solve(include_str!("sample.txt"), 0), 7);
}

#[test]
fn test_part_a() {
    assert_eq!(solve(include_str!("input.txt"), 0), 1392);
}

#[test]
fn test_part_b() {
    assert_eq!(solve(include_str!("input.txt"), 1), 15826992);
}
