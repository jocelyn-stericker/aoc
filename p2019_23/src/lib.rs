use std::collections::{HashMap, HashSet, VecDeque};

fn resolve(ram: &HashMap<usize, i64>, pc: usize, offset: usize, rb: i64) -> i64 {
    let opcode = ram.get(&pc).copied().unwrap_or(0);
    let div = (0..offset).fold(10, |div, _| div * 10);

    let p1 = ram.get(&(pc + offset)).copied().unwrap_or(0);

    if (opcode / div) % 10 == 1 {
        p1
    } else if (opcode / div) % 10 == 2 {
        ram.get(&((p1 + rb) as usize)).copied().unwrap_or(0)
    } else {
        ram.get(&(p1 as usize)).copied().unwrap_or(0)
    }
}

fn im(ram: &HashMap<usize, i64>, pc: usize, offset: usize, rb: i64) -> i64 {
    let opcode = ram.get(&pc).copied().unwrap_or(0);
    let div = (0..offset).fold(10, |div, _| div * 10);

    let p1 = ram.get(&(pc + offset)).copied().unwrap_or(0);

    if (opcode / div) % 10 == 1 {
        panic!("Invalidx");
    } else if (opcode / div) % 10 == 2 {
        p1 + rb
    } else {
        p1
    }
}

fn resolve_2(ram: &HashMap<usize, i64>, pc: usize, rb: i64) -> (i64, i64) {
    (resolve(ram, pc, 1, rb), resolve(ram, pc, 2, rb))
}

fn resolve_3(ram: &HashMap<usize, i64>, pc: usize, rb: i64) -> (i64, i64, i64) {
    (
        resolve(ram, pc, 1, rb),
        resolve(ram, pc, 2, rb),
        im(ram, pc, 3, rb),
    )
}

struct Computer {
    ram: HashMap<usize, i64>,
    pc: usize,
    rb: i64,

    checked: bool,
    out_queue: Vec<i64>,
    in_queue: VecDeque<i64>,
}

enum Status {
    Done,
    Blocked,
    Out(i64, i64, i64),
}

impl Computer {
    fn new(ram: HashMap<usize, i64>) -> Computer {
        Computer {
            ram,
            pc: 0,
            rb: 0,
            checked: false,
            out_queue: Vec::new(),
            in_queue: VecDeque::new(),
        }
    }

    pub fn exec(&mut self) -> Status {
        loop {
            let mod_idx;
            let mut mod_value;
            let opcode = self.ram.get(&self.pc).copied().unwrap_or(0);
            match opcode % 100 {
                // add
                1 => {
                    let (p1, p2, p3) = resolve_3(&self.ram, self.pc, self.rb);

                    mod_idx = Some(p3);
                    mod_value = Some(p1 + p2);

                    self.pc += 4;
                }
                // mul
                2 => {
                    let (p1, p2, p3) = resolve_3(&self.ram, self.pc, self.rb);

                    mod_idx = Some(p3);
                    mod_value = Some(p1 * p2);

                    self.pc += 4;
                }
                // in
                3 => {
                    mod_value = self.in_queue.pop_front();
                    if mod_value.is_none() {
                        if !self.checked {
                            self.checked = true;
                            return Status::Blocked;
                        } else {
                            mod_value = Some(-1);
                        }
                    }

                    self.checked = false;

                    let p1 = im(&self.ram, self.pc, 1, self.rb);
                    mod_idx = Some(p1);
                    self.pc += 2;
                }
                // out
                4 => {
                    self.out_queue.push(resolve(&self.ram, self.pc, 1, self.rb));

                    mod_idx = None;
                    mod_value = None;
                    self.pc += 2;
                }
                // jump-if-true
                5 => {
                    let (p1, p2) = resolve_2(&self.ram, self.pc, self.rb);
                    if p1 != 0 {
                        assert!(p2 >= 0);
                        self.pc = p2 as usize;
                    } else {
                        self.pc += 3;
                    }

                    mod_idx = None;
                    mod_value = None;
                }
                // jump-if-false
                6 => {
                    let (p1, p2) = resolve_2(&self.ram, self.pc, self.rb);
                    if p1 == 0 {
                        assert!(p2 >= 0);
                        self.pc = p2 as usize;
                    } else {
                        self.pc += 3;
                    }

                    mod_idx = None;
                    mod_value = None;
                }
                // less than
                7 => {
                    let (p1, p2, p3) = resolve_3(&self.ram, self.pc, self.rb);
                    mod_idx = Some(p3);
                    if p1 < p2 {
                        mod_value = Some(1);
                    } else {
                        mod_value = Some(0);
                    }

                    self.pc += 4;
                }
                // equals
                8 => {
                    let (p1, p2, p3) = resolve_3(&self.ram, self.pc, self.rb);
                    mod_idx = Some(p3);
                    if p1 == p2 {
                        mod_value = Some(1);
                    } else {
                        mod_value = Some(0);
                    }
                    self.pc += 4;
                }
                // rb
                9 => {
                    self.rb += resolve(&self.ram, self.pc, 1, self.rb);
                    mod_idx = None;
                    mod_value = None;
                    self.pc += 2;
                }
                // terminate
                99 => {
                    return Status::Done;
                }
                _ => {
                    panic!();
                }
            };

            if let (Some(mod_idx), Some(mod_value)) = (mod_idx, mod_value) {
                *self.ram.entry(mod_idx as usize).or_default() = mod_value;
            }

            if self.out_queue.len() == 3 {
                let ans = Status::Out(self.out_queue[0], self.out_queue[1], self.out_queue[2]);
                self.out_queue = vec![];
                return ans;
            }
        }
    }
}

pub fn soln(input: &str, test: bool) -> i64 {
    let ram: HashMap<usize, i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .enumerate()
        .collect();

    let mut computers: Vec<Computer> = vec![];
    for i in 0..50 {
        let mut c = Computer::new(ram.clone());
        c.in_queue.push_back(i);
        computers.push(c);
    }

    let mut prev_any_unblocked = true;
    let mut nat = (0, 0);
    let mut deliv: HashSet<i64> = HashSet::new();
    loop {
        let mut any_unblocked = false;
        for i in 0..50 {
            let computer = computers.get_mut(i).unwrap();
            match computer.exec() {
                Status::Blocked => {
                    continue;
                }
                Status::Done => {
                    panic!();
                }
                Status::Out(addr, a, b) => {
                    any_unblocked = true;
                    if addr == 255 {
                        if test {
                            return b;
                        } else {
                            nat = (a, b);
                        }
                    } else {
                        assert!(addr >= 0);
                        assert!(addr < 50);
                        computers[addr as usize].in_queue.push_back(a);
                        computers[addr as usize].in_queue.push_back(b);
                    }
                }
            }
        }

        let idle = !prev_any_unblocked && !any_unblocked;

        if idle {
            assert!(!test);
            computers[0].in_queue.push_back(nat.0);
            computers[0].in_queue.push_back(nat.1);
            if deliv.contains(&nat.1) {
                return nat.1;
            }
            deliv.insert(nat.1);
        }
        prev_any_unblocked = any_unblocked;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::soln(include_str!("input.txt"), true), 22877);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::soln(include_str!("input.txt"), false), 15210);
    }
}
