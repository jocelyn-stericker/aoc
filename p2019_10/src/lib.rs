use num::Rational;
use std::collections::{BTreeMap, BTreeSet};

pub fn get_targets(
    map: &Vec<Vec<i8>>,
    i: usize,
    j: usize,
) -> BTreeMap<(bool, Option<Rational>), BTreeSet<(isize, isize, isize)>> {
    let height = map.len();
    let width = map[0].len();

    let mut s: BTreeMap<(bool, Option<Rational>), BTreeSet<(isize, isize, isize)>> =
        Default::default();
    for x in 0..height {
        for y in 0..width {
            if i == x && j == y || map[x][y] != 1 {
                continue;
            }

            let key = if y == j {
                (x > i, None)
            } else {
                (
                    y < j,
                    Some(Rational::new(
                        (x as isize) - (i as isize),
                        (y as isize) - (j as isize),
                    )),
                )
            };

            s.entry(key).or_insert_with(|| BTreeSet::new()).insert((
                ((x as isize) - (i as isize)) * ((x as isize) - (i as isize))
                    + ((y as isize) - (j as isize)) * ((y as isize) - (j as isize)),
                (x as isize),
                (y as isize),
            ));
        }
    }

    s
}

pub fn parse(input: &str) -> Vec<Vec<i8>> {
    let map: Vec<Vec<i8>> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();

    map
}

pub fn best(map: &Vec<Vec<i8>>) -> (usize, usize, usize) {
    let height = map.len();
    let width = map[0].len();

    let mut best = 0;
    let mut bi = 0;
    let mut bj = 0;

    for i in 0..height {
        for j in 0..width {
            if map[i][j] == 1 {
                let score = get_targets(&map, i, j).len();
                if score > best {
                    best = score;
                    bi = i;
                    bj = j;
                }
            }
        }
    }

    (bi, bj, best)
}

pub fn part_a(input: &str) -> usize {
    best(&parse(input)).2
}

pub fn part_b(input: &str) -> isize {
    let map = parse(input);
    let (i, j, _) = best(&map);
    let mut targets = get_targets(&map, i, j);

    let mut l = 0;

    loop {
        let mut did_something = false;

        for ((_, _), ref mut v) in targets.iter_mut() {
            if !v.is_empty() {
                did_something = true;
                let (_, x, y) = v.get(&v.iter().next().unwrap().clone()).unwrap();
                l += 1;
                if l == 200 {
                    return 100 * y + x;
                }

                v.remove(&v.iter().next().unwrap().clone());
            }
        }

        if !did_something {
            break;
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a(include_str!("example1.txt")), 210);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 292);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b(include_str!("example1.txt")), 802);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 317);
    }
}
