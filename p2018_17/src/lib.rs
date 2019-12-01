use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Square {
    Sand,
    // "supporting"
    Water(bool),
}

fn draw(world: &HashMap<(usize, usize), Square>) {
    let mut min_x = 10000000;
    let mut max_x = 0;
    let mut min_y = 10000000;
    let mut max_y = 0;
    for (p, _s) in world {
        min_x = std::cmp::min(min_x, p.0);
        max_x = std::cmp::max(max_x, p.0);
        min_y = std::cmp::min(min_y, p.1);
        max_y = std::cmp::max(max_y, p.1);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match world.get(&(x, y)) {
                None => print!(" "),
                Some(Square::Sand) => print!("#"),
                Some(Square::Water(true)) => print!("w"),
                Some(Square::Water(false)) => print!("."),
            };
        }
        println!("");
    }
}

fn mark_supporting(world: &mut HashMap<(usize, usize), Square>, p: &(usize, usize), dx: i8) {
    match world.get(p) {
        None => {
            unreachable!();
        }
        Some(Square::Water(_)) => {
            world.insert(*p, Square::Water(true));
            if dx > 0 {
                mark_supporting(world, &(p.0 + 1, p.1), dx);
            } else {
                mark_supporting(world, &(p.0 - 1, p.1), dx);
            }
        }
        Some(_) => {
            // done
        }
    }
}

// -> piece below is "supporting"
fn dfs(
    world: &mut HashMap<(usize, usize), Square>,
    p: (usize, usize),
    max_y: usize,
    dx: i64,
) -> bool {
    if p.1 > max_y {
        return false;
    }

    match world.get(&p) {
        None => {
            if dfs(world, (p.0, p.1 + 1), max_y, 0) {
                if dx == 1 {
                    let a = dfs(world, (p.0 + 1, p.1), max_y, 1);
                    world.insert(p, Square::Water(false));
                    a
                } else if dx == -1 {
                    let a = dfs(world, (p.0 - 1, p.1), max_y, -1);
                    world.insert(p, Square::Water(false));
                    a
                } else {
                    let a = dfs(world, (p.0 + 1, p.1), max_y, 1);
                    let b = dfs(world, (p.0 - 1, p.1), max_y, -1);
                    world.insert(p, Square::Water(a && b));
                    if a && b {
                        mark_supporting(world, &(p.0 + 1, p.1), 1);
                        mark_supporting(world, &(p.0 - 1, p.1), -1);
                    }
                    a && b
                }
            } else {
                world.insert(p, Square::Water(false));
                false
            }
        }
        Some(Square::Water(supporting)) => *supporting,
        Some(Square::Sand) => true,
    }
}

fn solve(input: &str) -> HashMap<(usize, usize), Square> {
    let line_re_1 = Regex::new(r"^x=(?P<x>[0-9]+), y=(?P<y1>[0-9]+)..(?P<y2>[0-9]+)$").unwrap();
    let line_re_2 = Regex::new(r"^y=(?P<y>[0-9]+), x=(?P<x1>[0-9]+)..(?P<x2>[0-9]+)$").unwrap();

    let mut world: HashMap<(usize, usize), Square> = HashMap::new();
    let mut min_y = 1000000000;
    let mut max_y = 0;

    for l in input.split('\n').filter(|line| line != &"") {
        println!("{}", l);
        match (line_re_1.captures(l), line_re_2.captures(l)) {
            (Some(l), _) => {
                let (x, y1, y2) = (
                    l.name("x").unwrap().as_str().parse::<usize>().unwrap(),
                    l.name("y1").unwrap().as_str().parse::<usize>().unwrap(),
                    l.name("y2").unwrap().as_str().parse::<usize>().unwrap(),
                );

                for y in y1..=y2 {
                    world.insert((x, y), Square::Sand);
                }
                min_y = std::cmp::min(min_y, y1);
                max_y = std::cmp::max(max_y, y2);
            }
            (_, Some(l)) => {
                let (y, x1, x2) = (
                    l.name("y").unwrap().as_str().parse::<usize>().unwrap(),
                    l.name("x1").unwrap().as_str().parse::<usize>().unwrap(),
                    l.name("x2").unwrap().as_str().parse::<usize>().unwrap(),
                );
                min_y = std::cmp::min(min_y, y);
                max_y = std::cmp::max(max_y, y);

                for x in x1..=x2 {
                    world.insert((x, y), Square::Sand);
                }
            }
            _ => panic!("bad line"),
        }
    }

    dfs(&mut world, (500, min_y), max_y, 0);
    draw(&world);

    world
}

pub fn part_a(input: &str) -> usize {
    solve(input)
        .iter()
        .filter(|(_p, s)| match s {
            Square::Water(_) => true,
            _ => false,
        })
        .count()
}

pub fn part_b(input: &str) -> usize {
    solve(input)
        .iter()
        .filter(|(_p, s)| match s {
            Square::Water(true) => true,
            _ => false,
        })
        .count()
}

#[test]
fn test_sample() {
    assert_eq!(part_a(include_str!("sample.txt")), 57);
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(include_str!("input.txt")), 31471);
}

#[test]
fn test_part_b() {
    assert_eq!(part_b(include_str!("input.txt")), 24169);
}
