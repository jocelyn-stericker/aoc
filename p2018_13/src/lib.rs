use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Left,
    Forward,
    Right,
}

fn turn_left(vel: &(i64, i64)) -> (i64, i64) {
    match vel {
        (0, -1) => (-1, 0),
        (1, 0) => (0, -1),
        (0, 1) => (1, 0),
        (-1, 0) => (0, 1),
        _ => unreachable!(),
    }
}

fn turn_right(vel: &(i64, i64)) -> (i64, i64) {
    let l = turn_left(vel);
    (-l.0, -l.1)
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>, trains: &BTreeMap<(i64, i64), ((i64, i64), Dir)>) {
    println!("");
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if let Some(t) = trains.get(&(x as i64, y as i64)) {
                match t.0 {
                    (0, 1) => print!("v"),
                    (0, -1) => print!("^"),
                    (1, 0) => print!(">"),
                    (-1, 0) => print!("<"),
                    _ => unreachable!(),
                }
            } else {
                print!("{}", c);
            }
        }
        print!("\n");
    }
}

pub fn solve(input: &str, handle_crash: bool) -> (i64, i64) {
    let mut map: Vec<Vec<char>> = input
        .split("\n")
        .filter(|&line| line != "\n")
        .map(|line| line.chars().collect())
        .collect();
    let mut trains: BTreeMap<(i64, i64), ((i64, i64), Dir)> = BTreeMap::new();

    for (y, line) in map.clone().iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                '>' => {
                    map[y][x] = '-';
                    trains.insert((x as i64, y as i64), ((1, 0), Dir::Left));
                }
                '<' => {
                    map[y][x] = '-';
                    trains.insert((x as i64, y as i64), ((-1, 0), Dir::Left));
                }
                '^' => {
                    map[y][x] = '|';
                    trains.insert((x as i64, y as i64), ((0, -1), Dir::Left));
                }
                'v' => {
                    map[y][x] = '|';
                    trains.insert((x as i64, y as i64), ((0, 1), Dir::Left));
                }
                _ => {}
            }
        }
    }

    loop {
        // print_map(&map, &trains);

        let mut crashed_trains: HashSet<(i64, i64)> = HashSet::new();
        for (pos, (vel, turn)) in trains.clone().iter() {
            if crashed_trains.contains(&pos) {
                continue;
            }

            let next_pos = (pos.0 + vel.0, pos.1 + vel.1);
            trains.remove(&pos);
            if trains.contains_key(&next_pos) {
                if handle_crash {
                    trains.remove(&next_pos);
                    crashed_trains.insert(next_pos);
                    continue;
                }
                return next_pos;
            }
            let next_behav = match map[next_pos.1 as usize][next_pos.0 as usize] {
                '-' | '|' => (*vel, *turn),
                '/' => (
                    match vel {
                        (0, -1) => (1, 0),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, 1),
                        (1, 0) => (0, -1),
                        _ => unreachable!(),
                    },
                    *turn,
                ),
                '\\' => (
                    match vel {
                        (0, -1) => (-1, 0),
                        (0, 1) => (1, 0),
                        (-1, 0) => (0, -1),
                        (1, 0) => (0, 1),
                        _ => unreachable!(),
                    },
                    *turn,
                ),
                '+' => (
                    match turn {
                        Dir::Left => turn_left(vel),
                        Dir::Forward => *vel,
                        Dir::Right => turn_right(vel),
                    },
                    match turn {
                        Dir::Left => Dir::Forward,
                        Dir::Forward => Dir::Right,
                        Dir::Right => Dir::Left,
                    },
                ),
                _ => unreachable!(),
            };

            trains.insert(next_pos, next_behav);
        }

        if handle_crash && trains.len() == 1 {
            return *trains.iter().next().unwrap().0;
        }
    }
}

#[test]
fn test_sample_a() {
    assert_eq!(solve(include_str!("sample.txt"), false), (7, 3));
}

#[test]
fn test_sample_b() {
    assert_eq!(solve(include_str!("sample_2.txt"), true), (6, 4));
}

#[test]
fn test_part_a() {
    assert_ne!(solve(include_str!("input.txt"), false), (30, 122));
    assert_ne!(solve(include_str!("input.txt"), false), (116, 90));
    assert_eq!(solve(include_str!("input.txt"), false), (116, 91));
}

#[test]
fn test_part_b() {
    assert_ne!(solve(include_str!("input.txt"), true), (138, 108));
    assert_eq!(solve(include_str!("input.txt"), true), (8, 23));
}
