#[derive(Copy, Clone)]
struct Keypad(i64);

impl Keypad {
    fn x(&self) -> i64 {
        (self.0 - 1) % 3
    }

    fn y(&self) -> i64 {
        (self.0 - 1) / 3
    }

    fn proc(&self, l: char) -> Keypad {
        match l {
            'D' => Keypad((self.y() + 1).min(2) * 3 + self.x() + 1),
            'U' => Keypad((self.y() - 1).max(0) * 3 + self.x() + 1),
            'L' => Keypad((self.x() - 1).max(0) + 3 * self.y() + 1),
            'R' => Keypad((self.x() + 1).min(2) + 3 * self.y() + 1),
            _ => panic!(),
        }
    }
}

pub fn part_a(input: &str) -> String {
    let mut k = Keypad(5);
    input
        .trim()
        .split('\n')
        .map(|line| {
            for c in line.trim().chars() {
                k = k.proc(c);
            }
            k.0.to_string()
        })
        .collect::<String>()
}

const CRUEL: [[Option<char>; 5]; 5] = [
    [None, None, Some('1'), None, None],
    [None, Some('2'), Some('3'), Some('4'), None],
    [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    [None, Some('A'), Some('B'), Some('C'), None],
    [None, None, Some('D'), None, None],
];

#[derive(Copy, Clone)]
struct KeypadB(char);

impl KeypadB {
    fn pos(&self) -> (i64, i64) {
        for (y, line) in CRUEL.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if c == &Some(self.0) {
                    return (x as i64, y as i64);
                }
            }
        }
        panic!();
    }

    fn proc(&self, l: char) -> KeypadB {
        let (mut x, mut y) = self.pos();
        match l {
            'D' => {
                y += 1;
            }
            'U' => {
                y -= 1;
            }
            'L' => {
                x -= 1;
            }
            'R' => {
                x += 1;
            }
            _ => panic!(),
        };

        if y < 0 || x < 0 {
            return *self;
        }

        if let Some(c) = CRUEL
            .get(y as usize)
            .and_then(|line| line.get(x as usize).and_then(|c| *c))
        {
            KeypadB(c)
        } else {
            *self
        }
    }
}

pub fn part_b(input: &str) -> String {
    let mut k = KeypadB('5');
    input
        .trim()
        .split('\n')
        .map(|line| {
            for c in line.trim().chars() {
                k = k.proc(c);
            }
            k.0.to_string()
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("ULL\nRRDDD\nLURDL\nUUUUD\n"), "1985");
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), "52981"); //12:44
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b("ULL\nRRDDD\nLURDL\nUUUUD\n"), "5DB3");
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), "74CD2"); // 24:56
    }
}
