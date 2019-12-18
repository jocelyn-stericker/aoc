use std::collections::{HashMap, VecDeque};

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

pub fn exec(mut ram: HashMap<usize, i64>, mut inputs: VecDeque<u8>) -> String {
    let mut sf: String = String::new();
    let mut pc = 0;
    let mut rb: i64 = 0;
    let mut b = inputs.len() > 0;
    loop {
        let mod_idx;
        let mut mod_value;
        let opcode = ram.get(&pc).copied().unwrap_or(0);
        match opcode % 100 {
            // add
            1 => {
                let (p1, p2, p3) = resolve_3(&ram, pc, rb);

                mod_idx = Some(p3);
                mod_value = Some(p1 + p2);

                pc += 4;
            }
            // mul
            2 => {
                let (p1, p2, p3) = resolve_3(&ram, pc, rb);

                mod_idx = Some(p3);
                mod_value = Some(p1 * p2);

                pc += 4;
            }
            // in
            3 => {
                mod_value = Some(inputs.pop_front().unwrap() as i64);

                let p1 = im(&ram, pc, 1, rb);
                mod_idx = Some(p1);
                pc += 2;
            }
            // out
            4 => {
                let val = resolve(&ram, pc, 1, rb);

                if b && val > 256 {
                    return val.to_string();
                }
                sf.push(val as u8 as char);

                mod_idx = None;
                mod_value = None;
                pc += 2;
            }
            // jump-if-true
            5 => {
                let (p1, p2) = resolve_2(&ram, pc, rb);
                if p1 != 0 {
                    assert!(p2 >= 0);
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }

                mod_idx = None;
                mod_value = None;
            }
            // jump-if-false
            6 => {
                let (p1, p2) = resolve_2(&ram, pc, rb);
                if p1 == 0 {
                    assert!(p2 >= 0);
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }

                mod_idx = None;
                mod_value = None;
            }
            // less than
            7 => {
                let (p1, p2, p3) = resolve_3(&ram, pc, rb);
                mod_idx = Some(p3);
                if p1 < p2 {
                    mod_value = Some(1);
                } else {
                    mod_value = Some(0);
                }

                pc += 4;
            }
            // equals
            8 => {
                let (p1, p2, p3) = resolve_3(&ram, pc, rb);
                mod_idx = Some(p3);
                if p1 == p2 {
                    mod_value = Some(1);
                } else {
                    mod_value = Some(0);
                }
                pc += 4;
            }
            // rb
            9 => {
                rb += resolve(&ram, pc, 1, rb);
                mod_idx = None;
                mod_value = None;
                pc += 2;
            }
            // terminate
            99 => {
                break;
            }
            _ => {
                panic!();
            }
        };

        if let (Some(mod_idx), Some(mod_value)) = (mod_idx, mod_value) {
            *ram.entry(mod_idx as usize).or_default() = mod_value;
        }
    }

    sf
}

pub fn part_a(input: &str) -> usize {
    let ram: HashMap<usize, i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .enumerate()
        .collect();

    let sf: Vec<Vec<char>> = exec(ram, Default::default())
        .split('\n')
        .filter(|line| line.trim() != "")
        .map(|l| l.chars().collect())
        .collect();
    let x_len = sf[0].len();
    let y_len = sf.len();
    let mut score = 0;
    for x in 0..x_len {
        for y in 0..y_len {
            let c = sf[y][x];
            if c == '#'
                && x > 0
                && y > 0
                && x + 1 < x_len
                && y + 1 < y_len
                && sf[y - 1][x] == '#'
                && sf[y + 1][x] == '#'
                && sf[y][x - 1] == '#'
                && sf[y][x + 1] == '#'
            {
                score += x * y;
            }
        }
    }

    score
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    L,
    U,
    D,
    R,
}

impl Dir {
    fn left(self) -> Dir {
        match self {
            Dir::L => Dir::D,
            Dir::D => Dir::R,
            Dir::R => Dir::U,
            Dir::U => Dir::L,
        }
    }
    fn right(self) -> Dir {
        match self {
            Dir::L => Dir::U,
            Dir::D => Dir::L,
            Dir::R => Dir::D,
            Dir::U => Dir::R,
        }
    }
    fn coord(self) -> (i64, i64) {
        match self {
            Dir::L => (-1, 0),
            Dir::D => (0, 1),
            Dir::R => (1, 0),
            Dir::U => (0, -1),
        }
    }
}

fn is_free(map: &Vec<Vec<char>>, pos: &(i64, i64), dir: Dir) -> bool {
    let x_len = map[0].len();
    let y_len = map.len();
    let dp = dir.coord();
    let np = (pos.0 + dp.0, pos.1 + dp.1);

    np.0 >= 0
        && (np.0 as usize) < x_len
        && np.1 >= 0
        && (np.1 as usize) < y_len
        && map[np.1 as usize][np.0 as usize] == '#'
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cmd {
    L,
    R,
    M(usize),
}

impl Cmd {
    fn to_str(&self) -> String {
        match self {
            &Cmd::L => "L".to_string(),
            &Cmd::R => "R".to_string(),
            &Cmd::M(n) => n.to_string(),
        }
    }
}

fn spell_cmd(a: &Vec<Cmd>) -> String {
    a.iter()
        .map(|a| a.to_str())
        .collect::<Vec<String>>()
        .join(",")
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Compression {
    a: Option<Vec<Cmd>>,
    b: Option<Vec<Cmd>>,
    c: Option<Vec<Cmd>>,
    stack: Vec<Cmd>,
    result: Vec<&'static str>,
}

impl Compression {
    fn commit(&self) -> Option<Compression> {
        if self.stack.len() < 5 {
            // is this ok?
            None
        } else if self.result.len() == 10 {
            return None;
        } else if self.a.is_none() {
            let mut other = self.clone();
            other.a = Some(other.stack.split_off(0));
            other.result.push("A");
            eprintln!("{:?}", &other);
            Some(other)
        } else if self.b.is_none() {
            let mut other = self.clone();
            other.b = Some(other.stack.split_off(0));
            other.result.push("B");
            eprintln!("{:?}", &other);
            Some(other)
        } else if self.c.is_none() {
            let mut other = self.clone();
            other.c = Some(other.stack.split_off(0));
            other.result.push("C");
            eprintln!("{:?}", &other);
            Some(other)
        } else {
            None
        }
    }
    fn eat(&self) -> Option<Compression> {
        if self.stack.is_empty() {
            return None;
        }

        if self.result.len() == 10 {
            return None;
        }

        for (i, s) in [&self.a, &self.b, &self.c].iter().enumerate() {
            if let Some(s) = s {
                if s == &self.stack {
                    let mut other = self.clone();
                    other.stack.clear();
                    if i == 0 {
                        other.result.push("A");
                    } else if i == 1 {
                        other.result.push("B");
                    } else if i == 2 {
                        other.result.push("C");
                    }
                    return Some(other);
                }
            }
        }

        None
    }
}

fn compress(stream: &[Cmd], mut c: Compression) -> Option<Compression> {
    if stream.is_empty() && c.stack.is_empty() {
        return Some(c);
    }
    if let Some(commit) = c.commit() {
        if let Some(soln) = compress(stream, commit) {
            return Some(soln);
        }
    }
    if let Some(eat) = c.eat() {
        if let Some(soln) = compress(stream, eat) {
            return Some(soln);
        }
    }
    if !stream.is_empty() {
        let cmd = stream[0];
        c.stack.push(cmd);
        if spell_cmd(&c.stack).len() <= 20 {
            if let Some(soln) = compress(&stream[1..], c.clone()) {
                return Some(soln);
            }
        }
        c.stack.pop();
    }

    None
}

pub fn part_b(input: &str) -> String {
    let mut ram: HashMap<usize, i64> = input
        .trim()
        .split(',')
        .filter(|line| line != &"")
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .enumerate()
        .collect();

    let sf: Vec<Vec<char>> = exec(ram.clone(), Default::default())
        .split('\n')
        .filter(|line| line.trim() != "")
        .map(|l| l.chars().collect())
        .collect();

    let mut pos: (i64, i64) = (0, 0);
    let mut dir = Dir::U;
    let x_len = sf[0].len();
    let y_len = sf.len();

    for x in 0..x_len {
        for y in 0..y_len {
            if sf[y][x] == '^' {
                pos = (x as i64, y as i64);
            }
        }
    }

    let mut stream = Vec::new();

    let mut steps = 0;

    loop {
        if is_free(&sf, &pos, dir) {
            steps += 1;
            let dp = dir.coord();
            pos.0 += dp.0;
            pos.1 += dp.1;
            continue;
        } else if steps > 0 {
            stream.push(Cmd::M(steps));
            steps = 0;
        }

        if is_free(&sf, &pos, dir.left()) {
            dir = dir.left();
            stream.push(Cmd::L);
            continue;
        }

        if is_free(&sf, &pos, dir.right()) {
            dir = dir.right();
            stream.push(Cmd::R);
            continue;
        }

        break;
    }

    eprintln!("{:?}", stream);
    let soln = compress(&stream, Default::default()).unwrap();
    let soln = format!(
        "{}\n{}\n{}\n{}\nn\n",
        soln.result.join(","),
        spell_cmd(&soln.a.unwrap()),
        spell_cmd(&soln.b.unwrap()),
        spell_cmd(&soln.c.unwrap())
    );

    eprintln!("{}", soln);
    let out = soln.chars().map(|c| c as u8).collect::<VecDeque<u8>>();
    *ram.get_mut(&0).unwrap() = 2;
    exec(ram.clone(), out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 6672);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), "923017");
    }
}
