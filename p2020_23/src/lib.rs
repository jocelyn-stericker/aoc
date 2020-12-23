use std::collections::BTreeMap;

fn _log(mut p: usize, child: &BTreeMap<usize, usize>) {
    for _ in 0..10 {
        eprint!("{} ", p);
        p = child[&p];
    }
    eprintln!();
}

pub fn part_b(input: &str) -> usize {
    let strt = 10;
    let max = 1000000;
    let mut parent = BTreeMap::new();
    let mut child = BTreeMap::new();

    let mut game: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    for i in strt..=max {
        game.push(i);
    }

    for (i, g) in game.iter().enumerate().skip(1) {
        parent.insert(*g, game[i - 1]);
        child.insert(game[i - 1], *g);
    }
    parent.insert(game[0], *game.last().unwrap());
    child.insert(*game.last().unwrap(), game[0]);

    assert!(game.len() == max as usize);
    assert!(parent.len() == max as usize);

    let mut current = game[0];

    for i in 0..10000000 {
        if i % 100000 == 0 {
            eprintln!("{}", i);
        }
        let a = child[&current];
        let b = child[&a];
        let c = child[&b];
        let d = child[&c];

        let mut dst = current - 1;
        if dst == 0 {
            dst = max
        }
        while dst == a || dst == b || dst == c {
            dst -= 1;
            if dst == 0 {
                dst = max
            }
        }

        let dst_c = child[&dst];

        // a, b, c are moved to be after dst
        child.insert(dst, a);
        parent.insert(a, dst);

        child.insert(c, dst_c);
        parent.insert(dst_c, c);

        child.insert(current, d);
        parent.insert(d, current);

        current = child[&current];
    }

    let a = child[&1];
    let b = child[&a];
    eprintln!("{} {}", a, b);

    a * b
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_b("389125467\n"), 149245887792);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 235551949822);
    }
}
