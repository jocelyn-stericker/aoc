use std::collections::HashMap;

enum Rule {
    Ref(u64),
    Or(Box<Rule>, Box<Rule>),
    Ch(char),
    And3(Box<Rule>, Box<Rule>, Box<Rule>),
    And(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn matches<'a>(&self, rules: &'a HashMap<u64, Rule>, unparsed: &'a [char]) -> Vec<&'a [char]> {
        if unparsed.is_empty() {
            return vec![];
        }
        match self {
            Rule::Ref(i) => rules.get(i).unwrap().matches(rules, unparsed),
            Rule::Or(a, b) => {
                let mut r = Vec::new();
                for a in a.matches(rules, unparsed).into_iter() {
                    r.push(a);
                }
                for b in b.matches(rules, unparsed).into_iter() {
                    r.push(b);
                }

                r
            }
            Rule::Ch(c) => {
                if unparsed[0] == *c {
                    vec![&unparsed[1..]]
                } else {
                    vec![]
                }
            }
            Rule::And3(a, b, c) => {
                let mut r = Vec::new();
                for m in a.matches(rules, unparsed).into_iter() {
                    for n in b.matches(rules, m) {
                        for o in c.matches(rules, n) {
                            r.push(o);
                        }
                    }
                }
                r
            }
            Rule::And(a, b) => {
                let mut r = Vec::new();
                for m in a.matches(rules, unparsed).into_iter() {
                    for n in b.matches(rules, m) {
                        r.push(n);
                    }
                }
                r
            }
        }
    }
}

fn parse(s: &str) -> Rule {
    if s.contains(" | ") {
        let parts: Vec<_> = s.split(" | ").collect();

        Rule::Or(Box::new(parse(parts[0])), Box::new(parse(parts[1])))
    } else if s.starts_with('"') {
        Rule::Ch(s.chars().nth(1).unwrap())
    } else if s.contains(' ') {
        let parts: Vec<_> = s.split(' ').collect();
        if parts.len() == 3 {
            Rule::And3(
                Box::new(parse(parts[0])),
                Box::new(parse(parts[1])),
                Box::new(parse(parts[2])),
            )
        } else if parts.len() == 2 {
            Rule::And(Box::new(parse(parts[0])), Box::new(parse(parts[1])))
        } else {
            panic!();
        }
    } else if let Ok(i) = s.parse() {
        Rule::Ref(i)
    } else {
        eprintln!("{}", s);
        panic!();
    }
}

pub fn solve(input: &str) -> i64 {
    let mut parts = input.trim().split("\n\n");
    let rules = parts.next().unwrap();

    let mut r = HashMap::new();

    for rule in rules.trim().split('\n') {
        let mut parts = rule.split(": ");
        let id = parts.next().unwrap().parse::<u64>().unwrap();
        r.insert(id, parse(&parts.next().unwrap()));
    }

    let msgs = parts.next().unwrap();
    let mut c = 0;
    for msg in msgs.split('\n') {
        let msg: Vec<_> = msg.chars().collect();
        for m in r.get(&0).unwrap().matches(&r, &msg).into_iter() {
            if m.is_empty() {
                c += 1;
                break;
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        assert_eq!(super::solve(include_str!("input.txt")), 314);
    }
}
