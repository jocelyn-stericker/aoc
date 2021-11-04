use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
enum Instruction {
    SetNum(u16, String),
    SetReg(String, String),
    NumAnd(u16, String, String),
    And(String, String, String),
    Or(String, String, String),
    LShift(String, u16, String),
    RShift(String, u16, String),
    Not(String, String),
}

impl Instruction {
    fn deps(&self) -> Vec<String> {
        match self {
            Instruction::SetNum(_, _) => {
                vec![]
            }
            Instruction::SetReg(a, _) => {
                vec![a.clone()]
            }
            Instruction::And(a, b, _) => {
                vec![a.clone(), b.clone()]
            }
            Instruction::NumAnd(_, b, _) => {
                vec![b.clone()]
            }
            Instruction::Or(a, b, _) => {
                vec![a.clone(), b.clone()]
            }
            Instruction::LShift(a, _, _) => {
                vec![a.clone()]
            }
            Instruction::RShift(a, _, _) => {
                vec![a.clone()]
            }
            Instruction::Not(a, _) => {
                vec![a.clone()]
            }
        }
    }

    fn yields(&self) -> String {
        match self {
            Instruction::SetNum(_, a)
            | Instruction::SetReg(_, a)
            | Instruction::And(_, _, a)
            | Instruction::NumAnd(_, _, a)
            | Instruction::Or(_, _, a)
            | Instruction::LShift(_, _, a)
            | Instruction::RShift(_, _, a)
            | Instruction::Not(_, a) => a.clone(),
        }
    }

    fn exe(&self, comp: &mut HashMap<String, u16>) {
        match self {
            Instruction::SetNum(num, a) => {
                comp.insert(a.clone(), *num);
            }
            Instruction::SetReg(a, b) => {
                comp.insert(b.clone(), comp[a]);
            }
            Instruction::And(a, b, c) => {
                comp.insert(c.clone(), comp[a] & comp[b]);
            }
            Instruction::NumAnd(a, b, c) => {
                comp.insert(c.clone(), a & comp[b]);
            }
            Instruction::Or(a, b, c) => {
                comp.insert(c.clone(), comp[a] | comp[b]);
            }
            Instruction::LShift(a, b, c) => {
                comp.insert(c.clone(), comp[a] << *b);
            }
            Instruction::RShift(a, b, c) => {
                comp.insert(c.clone(), comp[a] >> *b);
            }
            Instruction::Not(a, b) => {
                comp.insert(b.clone(), !comp[a]);
            }
        }
    }
}

pub fn part_a(input: &str) -> u16 {
    let mut instructions = HashMap::new();
    let mut no_incoming = VecDeque::new();
    let mut edges = HashSet::new();
    for line in input.trim().split('\n') {
        let parts: Vec<_> = line.split(' ').collect();
        let inst = match (parts[0], parts[1]) {
            (_, "RSHIFT") => Instruction::RShift(
                parts[0].to_string(),
                parts[2].parse().unwrap(),
                parts[4].to_string(),
            ),
            (_, "LSHIFT") => Instruction::LShift(
                parts[0].to_string(),
                parts[2].parse().unwrap(),
                parts[4].to_string(),
            ),
            (_, "AND") => {
                if let Ok(num) = parts[0].parse::<u16>() {
                    Instruction::NumAnd(num, parts[2].to_string(), parts[4].to_string())
                } else {
                    Instruction::And(
                        parts[0].to_string(),
                        parts[2].to_string(),
                        parts[4].to_string(),
                    )
                }
            }
            (_, "OR") => Instruction::Or(
                parts[0].to_string(),
                parts[2].to_string(),
                parts[4].to_string(),
            ),
            ("NOT", _) => Instruction::Not(parts[1].to_string(), parts[3].to_string()),
            (_, "->") => {
                if let Ok(num) = parts[0].parse::<u16>() {
                    Instruction::SetNum(num, parts[2].to_string())
                } else {
                    Instruction::SetReg(parts[0].to_string(), parts[2].to_string())
                }
            }
            _ => panic!(),
        };
        if instructions.contains_key(&inst.yields()) {
            panic!();
        }
        if inst.deps().is_empty() {
            edges.insert(inst.yields());
            no_incoming.push_back(inst);
        } else {
            instructions.insert(inst.yields(), inst);
        }
    }

    let mut sorted = Vec::new();
    while let Some(next) = no_incoming.pop_front() {
        edges.insert(next.yields());
        sorted.push(next);

        let mut to_remove = Vec::new();
        for (key, value) in instructions.iter() {
            if value.deps().into_iter().all(|e| edges.contains(&e)) {
                to_remove.push(key.clone());
                no_incoming.push_back(value.clone());
            }
        }
        for key in to_remove {
            instructions.remove(&key);
        }
    }

    let mut comp = HashMap::new();
    for inst in &sorted {
        inst.exe(&mut comp);
    }

    comp["a"]
}

pub fn part_b(input: &str) -> u16 {
    let mut instructions = HashMap::new();
    let mut no_incoming = VecDeque::new();
    let mut edges = HashSet::new();
    for line in input.trim().split('\n') {
        let parts: Vec<_> = line.split(' ').collect();
        let inst = match (parts[0], parts[1]) {
            (_, "RSHIFT") => Instruction::RShift(
                parts[0].to_string(),
                parts[2].parse().unwrap(),
                parts[4].to_string(),
            ),
            (_, "LSHIFT") => Instruction::LShift(
                parts[0].to_string(),
                parts[2].parse().unwrap(),
                parts[4].to_string(),
            ),
            (_, "AND") => {
                if let Ok(num) = parts[0].parse::<u16>() {
                    Instruction::NumAnd(num, parts[2].to_string(), parts[4].to_string())
                } else {
                    Instruction::And(
                        parts[0].to_string(),
                        parts[2].to_string(),
                        parts[4].to_string(),
                    )
                }
            }
            (_, "OR") => Instruction::Or(
                parts[0].to_string(),
                parts[2].to_string(),
                parts[4].to_string(),
            ),
            ("NOT", _) => Instruction::Not(parts[1].to_string(), parts[3].to_string()),
            (_, "->") => {
                if let Ok(num) = parts[0].parse::<u16>() {
                    Instruction::SetNum(num, parts[2].to_string())
                } else {
                    Instruction::SetReg(parts[0].to_string(), parts[2].to_string())
                }
            }
            _ => panic!(),
        };
        if instructions.contains_key(&inst.yields()) {
            panic!();
        }
        if inst.deps().is_empty() {
            edges.insert(inst.yields());
            no_incoming.push_back(inst);
        } else {
            instructions.insert(inst.yields(), inst);
        }
    }

    let mut sorted = Vec::new();
    while let Some(next) = no_incoming.pop_front() {
        edges.insert(next.yields());
        sorted.push(next);

        let mut to_remove = Vec::new();
        for (key, value) in instructions.iter() {
            if value.deps().into_iter().all(|e| edges.contains(&e)) {
                to_remove.push(key.clone());
                no_incoming.push_back(value.clone());
            }
        }
        for key in to_remove {
            instructions.remove(&key);
        }
    }

    let mut comp = HashMap::new();
    for inst in &sorted {
        inst.exe(&mut comp);
    }

    let mut comp2 = HashMap::new();
    comp2.insert("b".to_string(), comp["a"]);
    for inst in &sorted {
        if let Instruction::SetNum(_, b) = inst {
            if b == "b" {
                continue;
            }
        }
        inst.exe(&mut comp2);
    }

    comp2["a"]
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 46065);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 14134);
    }
}
