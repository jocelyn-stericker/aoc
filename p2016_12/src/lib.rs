use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Src {
    Const(i64),
    Reg(char),
}

#[derive(Debug, Copy, Clone)]
enum Cmd {
    Cpy(Src, char),
    Inc(char),
    Dec(char),
    Jnz(Src, i64),
}

pub fn solve(input: &str, c: i64) -> i64 {
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
                line[2].chars().next().unwrap(),
            ),
            "inc" => Cmd::Inc(line[1].chars().next().unwrap()),
            "dec" => Cmd::Dec(line[1].chars().next().unwrap()),
            "jnz" => Cmd::Jnz(
                if let Ok(c) = line[1].parse::<i64>() {
                    Src::Const(c)
                } else {
                    Src::Reg(line[1].chars().next().unwrap())
                },
                line[2].parse().unwrap(),
            ),
            _ => panic!(),
        });
    }

    let mut reg = HashMap::new();
    reg.insert('a', 0);
    reg.insert('b', 0);
    reg.insert('c', c);
    reg.insert('d', 0);

    let mut pc = 0;
    loop {
        match prog.get(pc) {
            Some(Cmd::Cpy(Src::Const(i), dst)) => {
                reg.insert(*dst, *i);
            }
            Some(Cmd::Cpy(Src::Reg(c), dst)) => {
                let i = *reg.get(c).unwrap();
                reg.insert(*dst, i);
            }
            Some(Cmd::Inc(dst)) => {
                let i = *reg.get(dst).unwrap();
                reg.insert(*dst, i + 1);
            }
            Some(Cmd::Dec(dst)) => {
                let i = *reg.get(dst).unwrap();
                reg.insert(*dst, i - 1);
            }
            Some(Cmd::Jnz(Src::Const(i), dst)) => {
                if *i != 0 {
                    pc = (pc as i64 + *dst) as usize;
                    continue;
                }
            }
            Some(Cmd::Jnz(Src::Reg(c), dst)) => {
                let i = *reg.get(c).unwrap();
                if i != 0 {
                    pc = (pc as i64 + *dst) as usize;
                    continue;
                }
            }
            None => break,
        }

        pc += 1;
    }

    *reg.get(&'a').unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::solve("cpy 41 a\n inc a\n inc a\n dec a\n jnz a 2\n dec a\n", 0),
            42
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::solve(include_str!("input.txt"), 0), 318077); //16:30
    }

    #[test]
    fn part_b() {
        assert_eq!(super::solve(include_str!("input.txt"), 1), 9227731); //18:16
    }
}
