use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Doors {
    n: bool,
    e: bool,
    s: bool,
    w: bool,
}

impl Doors {
    fn new() -> Doors {
        Doors {
            n: false,
            e: false,
            s: false,
            w: false,
        }
    }
}

fn solve(
    input: &Vec<char>,
    idx: &mut usize,
    mut pos: (i64, i64),
    maze: &mut HashMap<(i64, i64), Doors>,
) {
    let starting_point = pos;

    loop {
        let room = maze.entry(pos).or_insert_with(Doors::new);

        match input.get(*idx) {
            Some('N') => {
                room.n = true;
                pos = (pos.0, pos.1 + 1);
                maze.entry(pos).or_insert_with(Doors::new).s = true;
                *idx += 1;
            }
            Some('E') => {
                room.e = true;
                pos = (pos.0 + 1, pos.1);
                maze.entry(pos).or_insert_with(Doors::new).w = true;
                *idx += 1;
            }
            Some('S') => {
                room.s = true;
                pos = (pos.0, pos.1 - 1);
                maze.entry(pos).or_insert_with(Doors::new).n = true;
                *idx += 1;
            }
            Some('W') => {
                room.w = true;
                pos = (pos.0 - 1, pos.1);
                maze.entry(pos).or_insert_with(Doors::new).e = true;
                *idx += 1;
            }
            Some('|') => {
                pos = starting_point;
                *idx += 1;
            }
            Some('(') => {
                *idx += 1;
                solve(&input, idx, pos, maze);
            }
            Some(')') => {
                *idx += 1;
                return;
            }
            Some('$') => {
                return;
            }
            Some(_) | None => {
                unreachable!();
            }
        }
    }
}

fn bfs(maze: &HashMap<(i64, i64), Doors>) -> HashMap<(i64, i64), u64> {
    let mut dist: HashMap<(i64, i64), u64> = HashMap::new();
    let mut queue: VecDeque<((i64, i64), u64)> = VecDeque::new();

    queue.push_back(((0, 0), 0));

    while let Some(((x, y), v)) = queue.pop_front() {
        if v >= dist.get(&(x, y)).cloned().unwrap_or(100000000000) {
            continue;
        }

        dist.insert((x, y), v);

        let door = maze.get(&(x, y)).unwrap();
        if door.n {
            queue.push_back(((x, y + 1), v + 1));
        }
        if door.s {
            queue.push_back(((x, y - 1), v + 1));
        }
        if door.e {
            queue.push_back(((x + 1, y), v + 1));
        }
        if door.w {
            queue.push_back(((x - 1, y), v + 1));
        }
    }

    dist
}

fn build_maze(input: &str) -> HashMap<(i64, i64), Doors> {
    let chars: Vec<char> = input.chars().filter(|l| l != &'\n').collect();
    let mut maze: HashMap<(i64, i64), Doors> = HashMap::new();
    let mut idx: usize = 0;
    assert_eq!(chars.get(idx), Some(&'^'));
    idx += 1;
    solve(&chars, &mut idx, (0, 0), &mut maze);
    assert_eq!(chars.get(idx), Some(&'$'));

    maze
}

pub fn part_a(input: &str) -> u64 {
    let maze = build_maze(input);
    *bfs(&maze).values().max().unwrap()
}

pub fn part_b(input: &str) -> u64 {
    let maze = build_maze(input);
    bfs(&maze).values().filter(|c| c >= &&1000).count() as u64
}

#[test]
fn test_sample() {
    assert_eq!(part_a("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 18);
    assert_eq!(
        part_a("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"),
        23
    );
    assert_eq!(
        part_a("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"),
        31
    );
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(include_str!("input.txt")), 4406);
}

#[test]
fn test_part_b() {
    assert_eq!(part_b(include_str!("input.txt")), 8468);
}
