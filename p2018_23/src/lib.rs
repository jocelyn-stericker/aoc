use z3::{Ast, Config, Context, Optimize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Bot((i64, i64, i64), i64);

fn z_abs<'a>(context: &'a Context, num: &Ast<'a>) -> Ast<'a> {
    num.ge(&context.from_i64(0))
        .ite(num, &num.mul(&[&context.from_i64(-1)]))
}

impl Bot {
    fn covers(&self, other: &Bot) -> bool {
        ((self.0).0 - (other.0).0).abs()
            + ((self.0).1 - (other.0).1).abs()
            + ((self.0).2 - (other.0).2).abs()
            <= self.1
    }

    fn z_score<'a>(&self, context: &'a Context, for_pt: &(Ast<'a>, Ast<'a>, Ast<'a>)) -> Ast<'a> {
        let dist = context.from_i64(0).add(&[
            &z_abs(context, &(for_pt.0.sub(&[&context.from_i64((self.0).0)]))),
            &z_abs(context, &(for_pt.1.sub(&[&context.from_i64((self.0).1)]))),
            &z_abs(context, &(for_pt.2.sub(&[&context.from_i64((self.0).2)]))),
        ]);

        dist.le(&context.from_i64(self.1))
            .ite(&context.from_i64(1), &context.from_i64(0))
    }
}

fn parse(input: &str) -> Vec<Bot> {
    input
        .split("\n")
        .filter(|l| l != &"")
        .map(|l| {
            let mut l = l[5..].split(">, r=");
            let mut pts = l
                .next()
                .unwrap()
                .split(",")
                .map(|s| s.parse::<i64>().unwrap());
            let r = l.next().unwrap().parse::<i64>().unwrap();

            Bot(
                (
                    pts.next().unwrap(),
                    pts.next().unwrap(),
                    pts.next().unwrap(),
                ),
                r,
            )
        })
        .collect()
}

pub fn part_a(input: &str) -> usize {
    let input = parse(input);

    let biggest = input.iter().max_by_key(|p| p.1).unwrap();
    input.iter().filter(|p| biggest.covers(p)).count()
}

pub fn part_b(input: &str) -> u64 {
    let input = parse(input);
    let config = Config::new();
    let context = Context::new(&config);

    let z_pt = (
        context.named_int_const("x"),
        context.named_int_const("y"),
        context.named_int_const("z"),
    );

    let mut z_score = context.from_i64(0);
    let scores: Vec<Ast> = input.iter().map(|pt| pt.z_score(&context, &z_pt)).collect();
    let scores: Vec<&Ast> = scores.iter().collect();
    z_score = z_score.add(&scores[..]);

    let opt = Optimize::new(&context);
    opt.maximize(&z_score);
    opt.minimize(&context.from_i64(0).add(&[
        &z_abs(&context, &(z_pt.0.sub(&[&context.from_i64(0)]))),
        &z_abs(&context, &(z_pt.1.sub(&[&context.from_i64(0)]))),
        &z_abs(&context, &(z_pt.2.sub(&[&context.from_i64(0)]))),
    ]));

    assert_eq!(opt.check(), true);
    let model = opt.get_model();

    let pt = (
        model.eval(&z_pt.0).unwrap().as_u64().unwrap(),
        model.eval(&z_pt.1).unwrap().as_u64().unwrap(),
        model.eval(&z_pt.2).unwrap().as_u64().unwrap(),
    );

    println!("{:?}", pt);

    pt.0 + pt.1 + pt.2
}

#[test]
fn test_sample() {
    assert_eq!(part_a(include_str!("sample.txt")), 7);
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(include_str!("input.txt")), 674);
}

#[test]
fn test_sample_b() {
    assert_eq!(part_b(include_str!("sample2.txt")), 36);
}

#[test]
fn test_part_b() {
    // not 54272700 (too low)
    assert_eq!(part_b(include_str!("input.txt")), 129444177);
}
