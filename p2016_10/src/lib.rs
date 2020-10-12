use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum Bin {
    Bot(i64),
    Output(i64),
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum Cmd {
    Init { val: i64, dst: Bin },
    Gives { src: i64, high: Bin, low: Bin },
}

pub fn part_a(input: &str) -> i64 {
    let mut cmds = Vec::new();

    for line in input.trim().lines() {
        let cmd: Vec<_> = line.trim().split(' ').collect();
        if cmd[0] == "value" {
            cmds.push(Cmd::Init {
                val: cmd[1].parse().unwrap(),
                dst: Bin::Bot(cmd[3].parse().unwrap()),
            });
        } else if cmd[0] == "bot" {
            cmds.push(Cmd::Gives {
                src: cmd[1].parse().unwrap(),
                low: if cmd[3] == "bot" {
                    Bin::Bot(cmd[4].parse().unwrap())
                } else {
                    Bin::Output(cmd[4].parse().unwrap())
                },
                high: if cmd[6] == "bot" {
                    Bin::Bot(cmd[7].parse().unwrap())
                } else {
                    Bin::Output(cmd[7].parse().unwrap())
                },
            });
        }
    }

    let mut state: HashMap<Bin, Vec<i64>> = HashMap::new();
    for cmd in &cmds {
        if let Cmd::Init { val, dst } = cmd {
            state.entry(*dst).or_default().push(*val);
        }
    }

    loop {
        let mut did_something = false;
        for cmd in &cmds {
            if let Cmd::Gives { src, high, low } = cmd {
                if let Some(bin) = state.get(&Bin::Bot(*src)) {
                    let mut bin = bin.clone();

                    if bin.len() == 2 {
                        bin.sort_unstable();
                        if bin[0] == 17 && bin[1] == 61 {
                            return *src;
                        }
                        state.entry(*low).or_default().push(bin[0]);
                        state.entry(*high).or_default().push(bin[1]);
                        did_something = true;
                        state.remove(&Bin::Bot(*src));
                    }
                }
            }
        }
        if !did_something {
            break;
        }
    }
    panic!();
}

pub fn part_b(input: &str) -> i64 {
    let mut cmds = Vec::new();

    for line in input.trim().lines() {
        let cmd: Vec<_> = line.trim().split(' ').collect();
        if cmd[0] == "value" {
            cmds.push(Cmd::Init {
                val: cmd[1].parse().unwrap(),
                dst: Bin::Bot(cmd[3].parse().unwrap()),
            });
        } else if cmd[0] == "bot" {
            cmds.push(Cmd::Gives {
                src: cmd[1].parse().unwrap(),
                low: if cmd[3] == "bot" {
                    Bin::Bot(cmd[4].parse().unwrap())
                } else {
                    Bin::Output(cmd[4].parse().unwrap())
                },
                high: if cmd[6] == "bot" {
                    Bin::Bot(cmd[7].parse().unwrap())
                } else {
                    Bin::Output(cmd[7].parse().unwrap())
                },
            });
        }
    }

    let mut state: HashMap<Bin, Vec<i64>> = HashMap::new();
    for cmd in &cmds {
        if let Cmd::Init { val, dst } = cmd {
            state.entry(*dst).or_default().push(*val);
        }
    }

    loop {
        let mut did_something = false;
        for cmd in &cmds {
            if let Cmd::Gives { src, high, low } = cmd {
                if let Some(bin) = state.get(&Bin::Bot(*src)) {
                    let mut bin = bin.clone();

                    if bin.len() == 2 {
                        bin.sort_unstable();
                        state.entry(*low).or_default().push(bin[0]);
                        state.entry(*high).or_default().push(bin[1]);
                        did_something = true;
                        state.remove(&Bin::Bot(*src));
                    }
                }
            }
        }
        if !did_something {
            break;
        }
    }

    state.get(&Bin::Output(0)).unwrap()[0]
        * state.get(&Bin::Output(1)).unwrap()[0]
        * state.get(&Bin::Output(2)).unwrap()[0]
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("R5, L5, R5, R3\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 141); //20:25
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1209); //23:20
    }
}
