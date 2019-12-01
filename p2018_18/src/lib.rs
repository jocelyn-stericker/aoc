use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Square {
    Open,
    Trees,
    Lumber,
}

fn summarize(world: &HashMap<(i64, i64), Square>, max_x: usize, max_y: usize) -> Vec<Square> {
    let mut v = Vec::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            v.push(world.get(&(y as i64, x as i64)).unwrap().clone());
        }
    }

    v
}

pub fn solve(input: &str, rounds: usize, allow_jump: bool) -> usize {
    let mut world: HashMap<(i64, i64), Square> = HashMap::new();
    let mut worlds: HashMap<Vec<Square>, usize> = HashMap::new();
    let mut max_y = 0;
    let mut max_x = 0;

    for (y, line) in input.split("\n").filter(|l| l != &"").enumerate() {
        max_y = y;
        for (x, c) in line.chars().enumerate() {
            max_x = x;
            world.insert(
                (y as i64, x as i64),
                match c {
                    '.' => Square::Open,
                    '|' => Square::Trees,
                    '#' => Square::Lumber,
                    _ => panic!("Bad square"),
                },
            );
        }
    }

    let mut i = 0;
    while i < rounds {
        let prev_world = world.clone();

        world = world
            .iter()
            .map(|((y, x), square)| {
                let (y, x) = (*y, *x);
                let adjacent = vec![
                    (y + 1, x),
                    (y + 1, x + 1),
                    (y + 1, x - 1),
                    (y - 1, x + 1),
                    (y - 1, x),
                    (y - 1, x - 1),
                    (y, x - 1),
                    (y, x + 1),
                ];

                let adjacent = adjacent.iter().map(|(y, x)| prev_world.get(&(*y, *x)));

                let square = match square {
                    Square::Open => {
                        if adjacent
                            .filter(|item| {
                                if let Some(Square::Trees) = item {
                                    true
                                } else {
                                    false
                                }
                            })
                            .count()
                            >= 3
                        {
                            Square::Trees
                        } else {
                            Square::Open
                        }
                    }
                    Square::Trees => {
                        if adjacent
                            .filter(|item| {
                                if let Some(Square::Lumber) = item {
                                    true
                                } else {
                                    false
                                }
                            })
                            .count()
                            >= 3
                        {
                            Square::Lumber
                        } else {
                            Square::Trees
                        }
                    }
                    Square::Lumber => {
                        let lumber_cnt = adjacent
                            .clone()
                            .filter(|item| {
                                if let Some(Square::Lumber) = item {
                                    true
                                } else {
                                    false
                                }
                            })
                            .count();
                        let tree_cnt = adjacent
                            .filter(|item| {
                                if let Some(Square::Trees) = item {
                                    true
                                } else {
                                    false
                                }
                            })
                            .count();

                        if lumber_cnt >= 1 && tree_cnt >= 1 {
                            Square::Lumber
                        } else {
                            Square::Open
                        }
                    }
                };

                ((y, x), square)
            })
            .collect();

        if allow_jump {
            let summary = summarize(&world, max_x, max_y);
            match worlds.get(&summary) {
                Some(j) => {
                    let gen_per_jump = i - j + 1;
                    let jumps = (rounds - i) / gen_per_jump;
                    i += jumps * gen_per_jump + 1;
                }
                None => {
                    worlds.insert(summarize(&prev_world, max_x, max_y), i);
                    i += 1;
                }
            }
        } else {
            i += 1;
        }
    }

    let wooded = world.iter().filter(|(_p, s)| s == &&Square::Trees).count();
    let lumber = world.iter().filter(|(_p, s)| s == &&Square::Lumber).count();

    wooded * lumber
}

#[test]
fn test_sample() {
    assert_eq!(solve(include_str!("sample.txt"), 10, true), 1147);
}

#[test]
fn test_part_a() {
    assert_eq!(solve(include_str!("input.txt"), 10, true), 621205);
}

#[test]
fn test_jump() {
    assert_eq!(solve(include_str!("input.txt"), 635, false), 229508);
    assert_eq!(solve(include_str!("input.txt"), 635, true), 229508);
    assert_eq!(solve(include_str!("input.txt"), 2000, false), 215900);
    assert_eq!(solve(include_str!("input.txt"), 2000, true), 215900);
}

#[test]
fn test_part_b() {
    assert_eq!(solve(include_str!("input.txt"), 1000000000, true), 228490);
}
