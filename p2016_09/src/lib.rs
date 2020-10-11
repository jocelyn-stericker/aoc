#[derive(Debug, Clone, Copy)]
enum Token {
    Char(char),
    Repeat(usize, usize),
}

impl Token {
    fn len(&self) -> usize {
        match self {
            Token::Char(_) => 1,
            Token::Repeat(a, b) => format!("({}x{})", a.to_string(), b.to_string()).len(),
        }
    }
}

pub fn part_a(input: &str) -> usize {
    let mut in_repeat_state = 0; // 0=not, 1=first, 2=second
    let mut lookahead_must_be_char = 0;
    let mut first = String::new();
    let mut second = String::new();

    let mut tokens = Vec::new();
    for c in input.trim().chars() {
        if lookahead_must_be_char == 0 && c == '(' && in_repeat_state == 0 {
            in_repeat_state = 1;
        } else if lookahead_must_be_char == 0 && c == 'x' && in_repeat_state == 1 {
            in_repeat_state = 2;
        } else if lookahead_must_be_char == 0 && c == ')' && in_repeat_state == 2 {
            in_repeat_state = 0;
            let lookahead = first.parse().unwrap();
            tokens.push(Token::Repeat(lookahead, second.parse().unwrap()));
            first = String::new();
            second = String::new();
            lookahead_must_be_char = lookahead;
        } else if in_repeat_state == 0 {
            tokens.push(Token::Char(c));
            if lookahead_must_be_char > 0 {
                lookahead_must_be_char -= 1;
            }
        } else if in_repeat_state == 1 {
            first.push(c);
        } else if in_repeat_state == 2 {
            second.push(c);
        }
    }

    let mut count = 0;
    for token in &tokens {
        count += match token {
            Token::Char(_) => 1,
            Token::Repeat(lookahead, repeats) => lookahead * (repeats - 1),
        }
    }

    count
}

fn compute_len(tokens: Vec<&Token>) -> usize {
    let token = tokens.get(0).unwrap();
    match token {
        Token::Char(_) => 1,
        Token::Repeat(mut lookahead, repeats) => {
            let mut sc = 0;
            let mut j = 1;
            while j <= lookahead {
                sc += (repeats - 1) * compute_len(tokens.iter().skip(j).copied().collect());
                lookahead -= tokens.get(j).unwrap().len() - 1;
                j += 1;
            }
            sc
        }
    }
}

pub fn part_b(input: &str) -> usize {
    let mut in_repeat_state = 0; // 0=not, 1=first, 2=second
    let mut first = String::new();
    let mut second = String::new();

    let mut tokens = Vec::new();
    for c in input.trim().chars() {
        if c == '(' && in_repeat_state == 0 {
            in_repeat_state = 1;
        } else if c == 'x' && in_repeat_state == 1 {
            in_repeat_state = 2;
        } else if c == ')' && in_repeat_state == 2 {
            in_repeat_state = 0;
            let lookahead = first.parse().unwrap();
            tokens.push(Token::Repeat(lookahead, second.parse().unwrap()));
            first = String::new();
            second = String::new();
        } else if in_repeat_state == 0 {
            tokens.push(Token::Char(c));
        } else if in_repeat_state == 1 {
            first.push(c);
        } else if in_repeat_state == 2 {
            second.push(c);
        }
    }

    let mut count = 0;
    for (i, _) in tokens.iter().enumerate() {
        count += compute_len(tokens.iter().skip(i).collect());
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("X(8x2)(3x3)ABCY\n"), 18);
        assert_eq!(super::part_a("(6x1)(1x3)A\n"), 6);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 150914); //20:00
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b("X(8x2)(3x3)ABCY"),
            "XABCABCABCABCABCABCY".len()
        );
        assert_eq!(
            super::part_b("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN\n"),
            445
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 11052855125); //40:00
    }
}
