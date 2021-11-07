use std::collections::BTreeMap;

pub fn part_a(input: &str) -> i64 {
    let mut known = BTreeMap::new();
    known.insert("children", 3);
    known.insert("cats", 7);
    known.insert("samoyeds", 2);
    known.insert("pomeranians", 3);
    known.insert("akitas", 0);
    known.insert("vizslas", 0);
    known.insert("goldfish", 5);
    known.insert("trees", 3);
    known.insert("cars", 2);
    known.insert("perfumes", 1);

    'a: for line in input.trim().split('\n') {
        let mut parts = line.split(", ");
        let sue: i64 = parts
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        for gift in parts {
            let mut parts = gift.split(": ");
            let name = parts.next().unwrap();
            let qty: i64 = parts.next().unwrap().parse().unwrap();

            if *known.get(name).unwrap() != qty {
                continue 'a;
            }
        }
        return sue;
    }
    0
}

pub fn part_b(input: &str) -> i64 {
    let mut known = BTreeMap::new();
    known.insert("children", 3);
    known.insert("cats", 7);
    known.insert("samoyeds", 2);
    known.insert("pomeranians", 3);
    known.insert("akitas", 0);
    known.insert("vizslas", 0);
    known.insert("goldfish", 5);
    known.insert("trees", 3);
    known.insert("cars", 2);
    known.insert("perfumes", 1);

    'a: for line in input.trim().split('\n') {
        let mut parts = line.split(", ");
        let sue: i64 = parts
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        for gift in parts {
            let mut parts = gift.split(": ");
            let name = parts.next().unwrap();
            let qty: i64 = parts.next().unwrap().parse().unwrap();

            let expected = *known.get(name).unwrap();

            if name == "cats" || name == "trees" {
                if qty <= expected {
                    continue 'a;
                }
            } else if name == "pomeranians" || name == "goldfish" {
                if qty >= expected {
                    continue 'a;
                }
            } else if expected != qty {
                continue 'a;
            }
        }
        return sue;
    }
    0
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 103);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 405);
    }
}
