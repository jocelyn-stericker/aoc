use std::collections::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Type {
    E,
    G,
    Invalid,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Object {
    Space,
    Wall,
    Mover(Type, i64, i64, usize), // hp, ap, id
}

impl Type {
    fn target(&self) -> Type {
        match self {
            Type::E => Type::G,
            Type::G => Type::E,
            Type::Invalid => Type::Invalid,
        }
    }
}

fn print_flood(f: &Vec<Vec<usize>>) {
    for line in f {
        for o in line {
            if o == &IMPOSSIBLE {
                print!("\t");
            } else {
                print!("{}\t", o);
            }
        }
        println!("");
    }
}

const IMPOSSIBLE: usize = 100000;

fn flood(world: &Vec<Vec<Object>>, x: usize, y: usize, target: Type) -> Vec<Vec<usize>> {
    let mut f: Vec<Vec<usize>> = world
        .iter()
        .map(|l| l.iter().map(|_t| IMPOSSIBLE).collect())
        .collect();

    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    q.push_back((x, y, 0));

    while !q.is_empty() {
        let (x, y, z) = q.pop_front().unwrap();
        if f[y][x] <= z {
            continue;
        }
        f[y][x] = z;
        for (dx, dy) in vec![(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let (x2, y2) = ((x as i64) + (dx as i64), (y as i64) + (dy as i64));
            if x2 < 0 || y2 < 0 || y2 >= world.len() as i64 || x2 >= world[y2 as usize].len() as i64
            {
                continue;
            }
            let c2 = world[y2 as usize][x2 as usize];
            match c2 {
                Object::Space => q.push_back((x2 as usize, y2 as usize, z + 1)),
                Object::Mover(t, _, _, _) if t == target => {
                    f[y2 as usize][x2 as usize] = std::cmp::min(f[y2 as usize][x2 as usize], z + 1);
                }
                _ => {}
            }
        }
    }

    f
}

fn do_move(
    world: &mut Vec<Vec<Object>>,
    pos: (usize, usize),
    c: Object,
    t: Type,
) -> Option<(usize, usize)> {
    let f = flood(world, pos.0, pos.1, t);
    let mut best = (0, 0, IMPOSSIBLE);
    for y in 0..world.len() {
        for x in 0..world[y].len() {
            let c = world[y][x];
            if let Object::Mover(c_type, _, _, _) = c {
                if c_type != t {
                    continue;
                }
            } else {
                continue;
            }

            for (dx, dy) in vec![(0, -1), (-1, 0), (1, 0), (0, 1)] {
                let (x2, y2) = ((x as i64) + (dx as i64), (y as i64) + (dy as i64));
                if x2 < 0
                    || y2 < 0
                    || y2 >= world.len() as i64
                    || x2 >= world[y2 as usize].len() as i64
                {
                    continue;
                }

                if x2 as usize == pos.0 && y2 as usize == pos.1 {
                    // can attack
                    return Some(pos);
                }
                let c2 = world[y2 as usize][x2 as usize];
                if c2 == Object::Space {
                    if best.2 > f[y2 as usize][x2 as usize] {
                        best = (x2, y2, f[y2 as usize][x2 as usize]);
                    }
                }
            }
        }
    }

    let f = flood(world, best.0 as usize, best.1 as usize, Type::Invalid);

    let mut best_move = (0, 0, IMPOSSIBLE);
    for (dx, dy) in vec![(0, -1), (-1, 0), (1, 0), (0, 1)] {
        let (x2, y2) = ((pos.0 as i64) + (dx as i64), (pos.1 as i64) + (dy as i64));
        if x2 < 0 || y2 < 0 || y2 >= world.len() as i64 || x2 >= world[y2 as usize].len() as i64 {
            continue;
        }
        let val = f[y2 as usize][x2 as usize];
        if val < best_move.2 {
            best_move = (x2, y2, val);
        }
    }

    if best_move.2 != IMPOSSIBLE {
        world[pos.1][pos.0] = Object::Space;
        world[best_move.1 as usize][best_move.0 as usize] = c;
        Some((best_move.0 as usize, best_move.1 as usize))
    } else {
        Some(pos)
    }
}

fn do_attack(world: &mut Vec<Vec<Object>>, pos: (usize, usize), ap: i64, t: Type) -> bool {
    let mut best = (0, 0, IMPOSSIBLE);

    for (dx, dy) in vec![(0, -1), (-1, 0), (1, 0), (0, 1)] {
        let (x2, y2) = ((pos.0 as i64) + (dx as i64), (pos.1 as i64) + (dy as i64));
        if x2 < 0 || y2 < 0 || y2 >= world.len() as i64 || x2 >= world[y2 as usize].len() as i64 {
            continue;
        }
        let c2 = world[y2 as usize][x2 as usize];
        match c2 {
            Object::Mover(c2_type, hp, _, _) if c2_type == t => {
                if hp < best.2 as i64 {
                    best = (x2, y2, hp as usize);
                }
            }
            _ => {}
        }
    }

    if best.2 != IMPOSSIBLE {
        let x2 = best.0;
        let y2 = best.1;
        let c2 = world[y2 as usize][x2 as usize];
        match c2 {
            Object::Mover(c2_type, hp, ap_b, id) if c2_type == t && hp - ap > 0 => {
                world[y2 as usize][x2 as usize] = Object::Mover(t, hp - ap, ap_b, id);
                return false;
            }
            Object::Mover(c2_type, hp, _ap, _) if c2_type == t && hp - ap <= 0 => {
                world[y2 as usize][x2 as usize] = Object::Space;
                return true;
            }
            _ => {
                // pass
            }
        }
    }

    return false;
}

fn print_world(world: &Vec<Vec<Object>>) {
    for line in world {
        for o in line {
            match o {
                Object::Mover(Type::E, _, _, _) => print!("E"),
                Object::Mover(Type::G, _, _, _) => print!("G"),
                Object::Mover(Type::Invalid, _, _, _) => print!("?"),
                Object::Space => print!("."),
                Object::Wall => print!("#"),
            }
        }
        println!("");
    }
}

fn is_done(world: &Vec<Vec<Object>>, i: u64) -> Option<u64> {
    let goblin_hp: u64 = world.iter().fold(0, |m, l| {
        l.iter().fold(m, |m, o| {
            if let Object::Mover(Type::G, hp, _ap, _) = o {
                m + (*hp as u64)
            } else {
                m
            }
        })
    });
    let elf_hp: u64 = world.iter().fold(0, |m, l| {
        l.iter().fold(m, |m, o| {
            if let Object::Mover(Type::E, hp, _ap, _) = o {
                m + (*hp as u64)
            } else {
                m
            }
        })
    });
    // print_world(&world);

    if goblin_hp == 0 {
        println!("OUTCOME: {} {}", elf_hp, i);
        Some(elf_hp * i)
    } else if elf_hp == 0 {
        println!("OUTCOME: {} {}", goblin_hp, i);
        Some(goblin_hp * i)
    } else {
        None
    }
}

pub fn part_a(input: &str, elf_ap: i64, elf_dead_ok: bool) -> u64 {
    let mut last_id = 0;
    let mut world: Vec<Vec<Object>> = input
        .split("\n")
        .filter(|l| l != &"")
        .map(|line| {
            line.chars()
                .map(|c| {
                    last_id += 1;
                    let id = last_id;
                    match c {
                        '.' => Object::Space,
                        '#' => Object::Wall,
                        'E' => Object::Mover(Type::E, 200, elf_ap, id),
                        'G' => Object::Mover(Type::G, 200, 3, id),
                        _ => panic!("Invalid object"),
                    }
                })
                .collect()
        })
        .collect();

    // print_world(&world);
    for i in 1.. {
        let mut q: Vec<(usize, usize, usize)> = Vec::new();
        for y in 0..world.len() {
            for x in 0..world[y].len() {
                let c = world[y][x];

                match c {
                    Object::Mover(_type_, _hp, _ap, id) => {
                        q.push((x, y, id));
                    }
                    _ => {
                        // pass
                    }
                }
            }
        }

        for (x, y, id) in q {
            let c = world[y][x];

            if let Some(score) = is_done(&world, i - 1) {
                print_world(&world);
                return score;
            }

            match c {
                Object::Mover(type_, _hp, ap, o_id) if o_id == id => {
                    let p2 = do_move(&mut world, (x, y), c, type_.target());
                    if let Some((x, y)) = p2 {
                        let killed = do_attack(&mut world, (x, y), ap, type_.target());
                        if killed && !elf_dead_ok && type_.target() == Type::E {
                            return 0;
                        }
                    }
                }
                _ => {
                    // pass
                }
            }
        }
    }
    0
}

pub fn part_b(input: &str) -> u64 {
    for ap in 3.. {
        println!("{}", ap);
        let r = part_a(input, ap, false);
        if r > 0 {
            return r;
        }
    }

    unreachable!();
}

#[test]
fn test_sample_1() {
    assert_eq!(part_a(include_str!("./sample_1.txt"), 3, true), 27730);
}

#[test]
fn test_sample_2() {
    assert_eq!(part_a(include_str!("./sample_2.txt"), 3, true), 36334);
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(include_str!("./input.txt"), 3, true), 201123);
}

#[test]
fn test_sample_3() {
    assert_eq!(part_b(include_str!("./sample_3.txt")), 4988);
}

#[test]
fn test_part_b() {
    assert_eq!(part_b(include_str!("./input.txt")), 54188);
}
