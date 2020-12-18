// use std::collections::HashSet;

#[derive(Debug)]
enum Expr {
    Val(i64),
    Plus(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Paren(Box<Expr>),
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Val(i) => *i,
            Expr::Plus(a, b) => a.eval() + b.eval(),
            Expr::Sub(a, b) => a.eval() - b.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
            Expr::Paren(a) => a.eval(),
        }
    }
}

fn parse<'a>(p: &'a [&'a str], before: Option<Expr>) -> (&'a [&'a str], Expr) {
    eprintln!("Parse {:?}", &p);

    if p.is_empty() || p[0] == ")" {
        (p, before.unwrap())
    } else if let Ok(s) = p[0].parse::<i64>() {
        eprintln!("V");
        assert!(before.is_none());
        (&p[1..], Expr::Val(s))
    } else if p[0] == "(" {
        eprintln!("PA");
        assert!(before.is_none());

        let mut unp: &[&str] = &p[1..];
        let mut result = None;
        while unp[0] != ")" {
            eprintln!(" subpa {:?}", &unp);
            let a = parse(&unp, result);
            unp = a.0;
            result = Some(a.1);
        }
        eprintln!("DONE");

        (&unp[1..], Expr::Paren(Box::new(result.unwrap())))
    } else if p[0] == "+" {
        eprintln!("PL");
        let before = before.unwrap();
        let r = parse(&p[1..], None);
        let new_before = Expr::Plus(Box::new(before), Box::new(r.1));
        parse(r.0, Some(new_before))
    } else if p[0] == "-" {
        eprintln!("MI");
        let before = before.unwrap();
        let r = parse(&p[1..], None);
        let new_before = Expr::Sub(Box::new(before), Box::new(r.1));
        parse(r.0, Some(new_before))
    } else if p[0] == "*" {
        eprintln!("TI");
        let before = before.unwrap();
        let r = parse(&p[1..], None);
        let new_before = Expr::Mul(Box::new(before), Box::new(r.1));
        parse(r.0, Some(new_before))
    } else {
        panic!()
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.trim().split('\n') {
        let all: Vec<_> = line.split(' ').collect();
        let mut unp: &[&str] = &all;
        let mut result = None;
        while !unp.is_empty() {
            eprintln!("X {:?}", &unp);
            let a = parse(&unp, result);
            unp = a.0;
            result = Some(a.1);
        }
        let result = result.unwrap();
        eprintln!("{:?}", &result);
        eprintln!("{:?}", result.eval());
        sum += result.eval();
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("1 + 2 * 3 + 4 * 5 + 6\n"), 71);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_a("1 + ( 2 * 3 ) + ( 4 * ( 5 + 6 ) )\n"), 51);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 5783053349377);
    }
}
