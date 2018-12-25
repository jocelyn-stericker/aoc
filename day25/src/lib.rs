#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pt(i64, i64, i64, i64);

impl Pt {
    fn dist(&self, other: &Pt) -> i64 {
        (self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()
    }
}

#[derive(Debug, Clone)]
struct Constellation(Vec<Pt>);

impl Constellation {
    fn dist(&self, other: &Pt) -> i64 {
        self.0.iter().map(|p| p.dist(other)).min().unwrap()
    }
}

pub fn solve(input: &str) -> usize {
    let mut consts: Vec<Constellation> = Vec::new();

    for pt in input.split("\n").filter(|l| l != &"").map(|l| {
        let n: Vec<i64> = l.split(",").map(|n| n.parse::<i64>().unwrap()).collect();
        Pt(n[0], n[1], n[2], n[3])
    }) {
        let mut new_c = Constellation(vec![pt]);
        for c in consts.iter().filter(|c| c.dist(&pt) <= 3) {
            for b in &c.0 {
                new_c.0.push(*b);
            }
        }
        consts = consts.iter().filter(|c| c.dist(&pt) > 3).cloned().collect();
        consts.push(new_c);
    }

    consts.len()
}

#[test]
fn test_sample() {
    assert_eq!(solve(include_str!("sample.txt")), 3);
}

#[test]
fn test_input() {
    assert_eq!(solve(include_str!("input.txt")), 373);
}
