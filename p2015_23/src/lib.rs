use std::collections::BTreeMap;

#[derive(Debug)]
enum Cmd {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i64),
    Jie(char, i64),
    Jio(char, i64),
}

pub fn part_a(input: &str) -> usize {
    let mut cmds = Vec::new();
    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        cmds.push(match parts.next().unwrap() {
            "hlf" => Cmd::Hlf(parts.next().unwrap().parse().unwrap()),
            "tpl" => Cmd::Tpl(parts.next().unwrap().parse().unwrap()),
            "inc" => Cmd::Inc(parts.next().unwrap().parse().unwrap()),
            "jmp" => Cmd::Jmp(parts.next().unwrap().parse().unwrap()),
            "jie" => {
                let c = parts.next().unwrap().parse().unwrap();
                Cmd::Jie(c, parts.next().unwrap().parse().unwrap())
            }
            "jio" => {
                let c = parts.next().unwrap().parse().unwrap();
                Cmd::Jio(c, parts.next().unwrap().parse().unwrap())
            }
            _ => {
                panic!()
            }
        });
    }

    let mut pc: i64 = 0;
    let mut reg = BTreeMap::new();
    reg.insert('a', 0);
    reg.insert('b', 0);
    while let Some(cmd) = cmds.get(pc as usize) {
        eprintln!("{} {:?}", pc, cmd);
        match cmd {
            Cmd::Hlf(c) => {
                *reg.get_mut(c).unwrap() /= 2;
            }
            Cmd::Tpl(c) => {
                *reg.get_mut(c).unwrap() *= 3;
            }
            Cmd::Inc(c) => {
                *reg.get_mut(c).unwrap() += 1;
            }
            Cmd::Jmp(c) => {
                pc += *c - 1;
            }
            Cmd::Jie(c, d) => {
                if reg[c] % 2 == 0 {
                    pc += *d - 1;
                }
            }
            Cmd::Jio(c, d) => {
                if reg[c] % 2 == 1 {
                    pc += *d - 1;
                }
            }
        }
        pc += 1;
        eprintln!(" -> {} {:?}", pc, reg);
    }

    reg[&'b']
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 0);
    }
}
