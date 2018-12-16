use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

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
    fn all_cmds() -> Vec<Cmd> {
        vec![
            Cmd::Addr,
            Cmd::Addi,
            Cmd::Mulr,
            Cmd::Muli,
            Cmd::Banr,
            Cmd::Bani,
            Cmd::Borr,
            Cmd::Bori,
            Cmd::Setr,
            Cmd::Seti,
            Cmd::Gtir,
            Cmd::Gtri,
            Cmd::Gtrr,
            Cmd::Eqir,
            Cmd::Eqri,
            Cmd::Eqrr,
        ]
    }

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

fn take_test_case<'a>(
    it: &mut impl Iterator<Item = &'a str>,
) -> Option<(Vec<usize>, (usize, usize, usize, usize), Vec<usize>)> {
    let l1 = it.next();
    if l1.is_none() {
        return None;
    }
    let l1 = l1.unwrap();
    if l1 == "" {
        it.next(); // extra blank line after test cases
        return None;
    }
    let l2 = it.next().unwrap();
    let l3 = it.next().unwrap();
    it.next(); // blank line between test cases
    let mut before = l1[9..19].split(", ").map(|v| v.parse::<usize>().unwrap());
    let mut after = l3[9..19].split(", ").map(|v| v.parse::<usize>().unwrap());
    let mut opl = l2.split(" ").map(|v| v.parse::<usize>().unwrap());

    Some((
        vec![
            before.next().unwrap(),
            before.next().unwrap(),
            before.next().unwrap(),
            before.next().unwrap(),
        ],
        (
            opl.next().unwrap(),
            opl.next().unwrap(),
            opl.next().unwrap(),
            opl.next().unwrap(),
        ),
        vec![
            after.next().unwrap(),
            after.next().unwrap(),
            after.next().unwrap(),
            after.next().unwrap(),
        ],
    ))
}

pub fn part_a(input: &str) -> u64 {
    let mut lines = input.split('\n');
    let mut test_cases = vec![];
    while let Some(test_case) = take_test_case(&mut lines) {
        test_cases.push(test_case);
    }

    let all_cmds = Cmd::all_cmds();
    let mut three_or_more_matches = 0;
    for test_case in &test_cases {
        let mut matches = 0;
        for cmd in &all_cmds {
            let test_cmd = test_case.1;
            matches += match cmd.run(test_cmd.1, test_cmd.2, test_cmd.3, &test_case.0) {
                Some(ref test_result) if test_result == &test_case.2 => 1,
                _ => 0,
            };
        }

        if matches >= 3 {
            three_or_more_matches += 1;
        }
    }

    three_or_more_matches
}

pub fn part_b(input: &str) -> usize {
    let mut lines = input.split('\n');
    let mut test_cases = vec![];
    while let Some(test_case) = take_test_case(&mut lines) {
        test_cases.push(test_case);
    }

    let program: Vec<Vec<usize>> = lines
        .map(|l| l.split(" ").map(|s| s.parse::<usize>().unwrap()).collect())
        .collect();

    let mut possible_opcodes: HashMap<usize, HashSet<Cmd>> = HashMap::new();

    let all_cmds = Cmd::all_cmds();
    for test_case in &test_cases {
        let test_cmd = test_case.1;
        let possibilites: HashSet<Cmd> = all_cmds
            .iter()
            .filter_map(
                |cmd| match cmd.run(test_cmd.1, test_cmd.2, test_cmd.3, &test_case.0) {
                    Some(ref test_result) if test_result == &test_case.2 => Some(*cmd),
                    _ => None,
                },
            )
            .collect();

        match possible_opcodes.entry(test_cmd.0) {
            Entry::Vacant(entry) => {
                entry.insert(possibilites);
            }
            Entry::Occupied(entry) => {
                let entry = entry.into_mut();
                let x = entry.intersection(&possibilites).cloned().collect();
                *entry = x;
            }
        }
    }

    // Solve it!
    let mut known_opcodes: HashMap<usize, Cmd> = HashMap::new();
    let mut known_cmds: HashSet<Cmd> = HashSet::new();

    while !possible_opcodes.is_empty() {
        let mut did_something = false;

        possible_opcodes = possible_opcodes
            .iter()
            .filter_map(|(key, values)| {
                if values.len() == 1 {
                    did_something = true;
                    let cmd = values.iter().next().unwrap();
                    known_opcodes.insert(*key, *cmd);
                    known_cmds.insert(*cmd);
                    None
                } else {
                    Some((
                        *key,
                        values
                            .iter()
                            .filter(|c| {
                                let contains = !known_cmds.contains(c);
                                if !contains {
                                    did_something = true;
                                }
                                contains
                            })
                            .cloned()
                            .collect(),
                    ))
                }
            })
            .collect();

        assert!(did_something);
    }

    println!("Opcodes: {:?}", known_opcodes);

    // Run the program
    let mut reg: Vec<usize> = vec![0, 0, 0, 0];

    for line in program {
        let cmd = known_opcodes.get(&line[0]).unwrap();
        reg = cmd
            .run(line[1], line[2], line[3], &reg)
            .expect("Memory out-of-bounds");
    }

    reg[0]
}

#[test]
fn test_sample() {
    assert_eq!(
        part_a("Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]\n"),
        1
    );
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(include_str!("input.txt")), 517);
}

#[test]
fn test_part_b() {
    assert_eq!(part_b(include_str!("input.txt")), 667);
}
