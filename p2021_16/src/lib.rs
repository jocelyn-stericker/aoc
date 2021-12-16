fn chars_to_num(c: &[char]) -> usize {
    let c: String = c.iter().collect();
    usize::from_str_radix(&c, 2).unwrap()
}

enum OperatorState {
    Sum(usize),
    Product(usize),
    Min(usize),
    Max(usize),
    Gt(Vec<usize>),
    Lt(Vec<usize>),
    Eq(Vec<usize>),
}

impl OperatorState {
    fn insert(&mut self, val: usize) {
        match self {
            OperatorState::Sum(state) => {
                *state += val;
            }
            OperatorState::Product(state) => {
                *state *= val;
            }
            OperatorState::Min(state) => {
                *state = (*state).min(val);
            }
            OperatorState::Max(state) => {
                *state = (*state).max(val);
            }
            OperatorState::Gt(vals) | OperatorState::Lt(vals) | OperatorState::Eq(vals) => {
                vals.push(val);
            }
        }
    }

    fn val(&self) -> usize {
        match self {
            OperatorState::Sum(val)
            | OperatorState::Product(val)
            | OperatorState::Min(val)
            | OperatorState::Max(val) => *val,
            OperatorState::Gt(vals) => {
                if vals[0] > vals[1] {
                    1
                } else {
                    0
                }
            }
            OperatorState::Lt(vals) => {
                if vals[0] < vals[1] {
                    1
                } else {
                    0
                }
            }
            OperatorState::Eq(vals) => {
                if vals[0] == vals[1] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn parse(i: &mut usize, decoded: &[char], part_a: bool) -> usize {
    let version = chars_to_num(&decoded[*i..*i + 3]);
    let type_id = chars_to_num(&decoded[*i + 3..*i + 6]);
    *i += 6;

    if type_id == 4 {
        // Literal
        let mut continues = true;

        let mut val = Vec::new();
        while continues {
            continues = decoded[*i] == '1';
            let mut decoded: Vec<char> = decoded[*i + 1..*i + 5].iter().copied().collect();
            val.append(&mut decoded);

            *i += 5;
        }
        let val = chars_to_num(&val);
        eprintln!("literal {}", val);
        if part_a {
            version
        } else {
            val
        }
    } else {
        let mut op = if part_a {
            OperatorState::Sum(version)
        } else {
            match type_id {
                0 => OperatorState::Sum(0),
                1 => OperatorState::Product(1),
                2 => OperatorState::Min(usize::MAX),
                3 => OperatorState::Max(0),
                5 => OperatorState::Gt(vec![]),
                6 => OperatorState::Lt(vec![]),
                7 => OperatorState::Eq(vec![]),
                _ => panic!(),
            }
        };

        let length_id = decoded[*i];
        *i += 1;
        if length_id == '0' {
            // If the length type ID is 0, then the next 15 bits are a number that represents the
            // total length in bits of the sub-packets contained by this packet.
            let len = chars_to_num(&decoded[*i..*i + 15]);
            *i += 15;
            let stop_at = *i + len;
            eprintln!("{}", len);
            while *i < stop_at {
                eprintln!("Parsing {:?}", *i);
                op.insert(parse(i, decoded, part_a));
            }
            eprintln!("{} {}", *i, stop_at);
            assert!(*i == stop_at);
        } else if length_id == '1' {
            // If the length type ID is 1, then the next 11 bits are a number that represents the
            // number of sub-packets immediately contained by this packet.
            let len = chars_to_num(&decoded[*i..*i + 11]);
            *i += 11;
            eprintln!("=== Start ===");
            for _ in 0..len {
                op.insert(parse(i, decoded, part_a));
            }
            eprintln!("=== End ===");
        }

        op.val()
    }
}

pub fn solve(input: &str, part_a: bool) -> usize {
    let mut decoded = String::new();
    for c in input.trim().chars() {
        decoded += &format!("{:04b}", c.to_digit(16).unwrap());
    }
    let mut i = 0;
    let decoded: Vec<char> = decoded.chars().collect();
    let mut version_sum = 0;
    while i < decoded.len() {
        if decoded.len() - i < 20 && chars_to_num(&decoded[i..]) == 0 {
            break;
        }
        version_sum += parse(&mut i, &decoded, part_a);
    }
    version_sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::solve("8A004A801A8002F478\n", true), 16);
        assert_eq!(super::solve("620080001611562C8802118E34\n", true), 12);
        assert_eq!(super::solve("C0015000016115A2E0802F182340\n", true), 23);
        assert_eq!(super::solve("A0016C880162017C3686B18A3D4780\n", true), 31);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::solve(include_str!("input.txt"), true), 883);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::solve(include_str!("input.txt"), false), 1675198555015);
    }
}
