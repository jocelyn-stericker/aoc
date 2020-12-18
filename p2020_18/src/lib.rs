// use std::collections::HashSet;

#[derive(Debug)]
enum Expr {
    Val(i64),
    Plus(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Paren(Box<Expr>),
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Val(i) => *i,
            Expr::Plus(a, b) => a.eval() + b.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
            Expr::Paren(a) => a.eval(),
        }
    }
}

fn parse<'a>(p: &'a [&'a str], before: Option<Expr>) -> (&'a [&'a str], Expr) {
    if p.is_empty() || p[0] == ")" {
        (p, before.unwrap())
    } else if let Ok(s) = p[0].parse::<i64>() {
        assert!(before.is_none());
        (&p[1..], Expr::Val(s))
    } else if p[0] == "(" {
        assert!(before.is_none());

        let mut unp: &[&str] = &p[1..];
        let mut result = None;
        while unp[0] != ")" {
            let a = parse(&unp, result);
            unp = a.0;
            result = Some(a.1);
        }

        (&unp[1..], Expr::Paren(Box::new(result.unwrap())))
    } else if p[0] == "*" {
        let before = before.unwrap();

        let mut unp: &[&str] = &p[1..];
        let mut result = None;
        while !unp.is_empty() && unp[0] != ")" {
            let a = parse(&unp, result);
            unp = a.0;
            result = Some(a.1);
        }

        let new_before = Expr::Mul(Box::new(before), Box::new(result.unwrap()));
        parse(unp, Some(new_before))
    } else if p[0] == "+" {
        let before = before.unwrap();
        let r = parse(&p[1..], None);
        let new_before = Expr::Plus(Box::new(before), Box::new(r.1));
        parse(r.0, Some(new_before))
    } else {
        panic!()
    }
}

pub fn part_b(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.trim().split('\n') {
        let all: Vec<_> = line.split(' ').collect();
        let mut unp: &[&str] = &all;
        let mut result = None;
        while !unp.is_empty() {
            let a = parse(&unp, result);
            unp = a.0;
            result = Some(a.1);
        }
        let result = result.unwrap();
        sum += result.eval();
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_b("1 + 2 * 3 + 4 * 5 + 6\n"), 231);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 74821486966872);
    }
}
