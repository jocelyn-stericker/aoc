#[derive(Debug, Clone)]
enum Number {
    Regular(u32),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn assert_regular(self) -> u32 {
        if let Number::Regular(a) = self {
            a
        } else {
            panic!();
        }
    }
    fn parse(input: &mut &[char]) -> Number {
        match input[0] {
            '[' => {
                *input = &(*input)[1..];
                let first_element = Number::parse(input);
                assert!(input[0] == ',');
                *input = &(*input)[1..];
                let second_element = Number::parse(input);
                assert!(input[0] == ']');
                *input = &(*input)[1..];
                Number::Pair(Box::new(first_element), Box::new(second_element))
            }
            d => {
                let n = Number::Regular(d.to_digit(10).unwrap());
                *input = &(*input)[1..];
                n
            }
        }
    }

    fn add_to_right(&mut self, num: u32) {
        match self {
            Number::Pair(a, _b) => {
                a.add_to_right(num);
            }
            Number::Regular(n) => {
                *n += num;
            }
        }
    }

    fn add_to_left(&mut self, num: u32) {
        match self {
            Number::Pair(_a, b) => {
                b.add_to_left(num);
            }
            Number::Regular(n) => {
                *n += num;
            }
        }
    }

    fn explode(self, depth: usize, exploded: &mut bool) -> (Option<u32>, Number, Option<u32>) {
        match self {
            Number::Pair(a, mut b) => {
                if depth == 3 {
                    match (*a, *b) {
                        (Number::Pair(a, b), mut c) => {
                            *exploded = true;
                            let a = a.assert_regular();
                            let b = b.assert_regular();
                            c.add_to_right(b);
                            (
                                Some(a),
                                Number::Pair(Box::new(Number::Regular(0)), Box::new(c)),
                                None,
                            )
                        }
                        (mut a, Number::Pair(b, c)) => {
                            *exploded = true;
                            let b = b.assert_regular();
                            let c = c.assert_regular();
                            a.add_to_left(b);
                            (
                                None,
                                Number::Pair(Box::new(a), Box::new(Number::Regular(0))),
                                Some(c),
                            )
                        }
                        (a, b) => (None, Number::Pair(Box::new(a), Box::new(b)), None),
                    }
                } else {
                    assert!(!*exploded);
                    let (le, mut a, mut re) = a.explode(depth + 1, exploded);
                    if let Some(re) = re.take() {
                        b.add_to_right(re);
                    }
                    if *exploded {
                        return (le, Number::Pair(Box::new(a), b), re);
                    }

                    let (mut le, b, re) = b.explode(depth + 1, exploded);
                    if let Some(le) = le.take() {
                        a.add_to_left(le);
                    }
                    (le, Number::Pair(Box::new(a), Box::new(b)), re)
                }
            }
            Number::Regular(_) => (None, self, None),
        }
    }

    fn split(self, did_split: &mut bool) -> Number {
        match self {
            Number::Pair(a, b) => {
                if *did_split {
                    return Number::Pair(a, b);
                }
                let a = a.split(did_split);
                if *did_split {
                    Number::Pair(Box::new(a), b)
                } else {
                    Number::Pair(Box::new(a), Box::new(b.split(did_split)))
                }
            }
            Number::Regular(a) => {
                assert!(!*did_split);
                if a > 9 {
                    *did_split = true;
                    Number::Pair(
                        Box::new(Number::Regular(a / 2)),
                        Box::new(Number::Regular(if a % 2 == 0 { a / 2 } else { a / 2 + 1 })),
                    )
                } else {
                    self
                }
            }
        }
    }

    fn reduce(self) -> Number {
        let mut me = self;

        loop {
            let mut iterations = 0;
            let mut exploded = true;
            while exploded {
                exploded = false;
                me = me.explode(0, &mut exploded).1;
                iterations += 1;
            }
            let mut did_split = false;
            me = me.split(&mut did_split);
            if !did_split && iterations == 1 {
                break;
            }
        }

        me
    }

    fn mag(&self) -> u32 {
        match self {
            Number::Pair(a, b) => 3 * a.mag() + 2 * b.mag(),
            Number::Regular(a) => *a,
        }
    }
}

pub fn part_a(input: &str) -> u32 {
    let mut nums = Vec::new();
    for line in input.trim().split('\n') {
        let chars: Vec<char> = line.chars().collect();
        let num = Number::parse(&mut (&chars as &[char])).reduce();
        nums.push(num);
    }

    let final_sum = nums
        .into_iter()
        .reduce(|a, b| Number::Pair(Box::new(a), Box::new(b)).reduce())
        .unwrap();

    final_sum.mag()
}

pub fn part_b(input: &str) -> u32 {
    let mut nums = Vec::new();
    for line in input.trim().split('\n') {
        let chars: Vec<char> = line.chars().collect();
        let num = Number::parse(&mut (&chars as &[char])).reduce();
        nums.push(num);
    }

    let mut max_mag = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i != j {
                max_mag = max_mag.max(
                    Number::Pair(Box::new(nums[i].clone()), Box::new(nums[j].clone()))
                        .reduce()
                        .mag(),
                );
            }
        }
    }

    max_mag
}

#[cfg(test)]
mod tests {
    //  #[test]
    //  fn explode1() {
    //      assert_eq!(super::part_a("[[[[[9,8],1],2],3],4]"), 0);
    //  }

    //  #[test]
    //  fn explode2() {
    //      assert_eq!(super::part_a("[7,[6,[5,[4,[3,2]]]]]"), 0);
    //  }

    // #[test]
    // fn explode3() {
    //     assert_eq!(super::part_a("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"), 0);
    // }

    #[test]
    fn example0() {
        assert_eq!(super::part_a("[9,1]",), 29);
    }
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            ),
            4140
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 4173);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 4706);
    }
}
