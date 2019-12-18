use std::collections::{BTreeSet, HashMap, VecDeque};

pub fn part_a(input: &str) -> usize {
    let maze: Vec<Vec<char>> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.chars().collect())
        .collect();

    let h = maze.len();
    let w = maze[0].len();

    let mut start: (i64, i64) = (0, 0);
    let mut all_keys: BTreeSet<char> = BTreeSet::new();
    for x in 0..w {
        for y in 0..h {
            if maze[y][x] == '@' {
                start = (x as i64, y as i64);
            }
            if maze[y][x] != '.' && maze[y][x] != '#' && maze[y][x].is_ascii_lowercase() {
                all_keys.insert(maze[y][x]);
            }
        }
    }
    eprintln!("{:?}", all_keys.len());

    // gbest(&maze, start, &mut Default::default(), all_keys.len())
    let h = maze.len();
    let w = maze[0].len();
    let mut seen = HashMap::new();
    let mut visit = VecDeque::new();
    visit.push_back((start, 0, BTreeSet::new()));
    let mut longest_chain = 0;

    while let Some((pos, i, keys)) = visit.pop_front() {
        if pos.0 < 0 || pos.1 < 0 || pos.0 > w as i64 || pos.1 > h as i64 {
            continue;
        }
        if seen.contains_key(&(pos, keys.clone())) {
            continue;
        }
        seen.insert((pos, keys.clone()), i);

        let c = maze[pos.1 as usize][pos.0 as usize];
        if c.is_ascii_lowercase() {
            let mut nkeys = keys.clone();
            if keys.len() > longest_chain {
                eprintln!(">> {}", keys.len());
                longest_chain = keys.len();
            }
            nkeys.insert(c);
            seen.insert((pos, nkeys.clone()), i);
            visit.push_back(((pos.0 + 1, pos.1), i + 1, nkeys.clone()));
            visit.push_back(((pos.0 - 1, pos.1), i + 1, nkeys.clone()));
            visit.push_back(((pos.0, pos.1 + 1), i + 1, nkeys.clone()));
            visit.push_back(((pos.0, pos.1 - 1), i + 1, nkeys));
        } else if c == '.' || c == '@' || keys.contains(&c.to_ascii_lowercase()) {
            visit.push_back(((pos.0 + 1, pos.1), i + 1, keys.clone()));
            visit.push_back(((pos.0 - 1, pos.1), i + 1, keys.clone()));
            visit.push_back(((pos.0, pos.1 + 1), i + 1, keys.clone()));
            visit.push_back(((pos.0, pos.1 - 1), i + 1, keys.clone()));
        }
    }

    let mut bs = std::usize::MAX;
    let l = all_keys.len();
    for (k, v) in seen {
        if k.1.len() == l {
            eprintln!(">>> {}", v);
            bs = bs.min(v);
        }
    }

    bs

    // let mut best = std::usize::MAX;
    // // if keys.len() == 2 {
    // //     eprintln!("{:?}", key_locations);
    // //     panic!();
    // // }
    // for (key, dist, loc) in &key_locations {
    //     keys.insert(*key);
    //     let score = gbest(maze, *loc, keys, num_keys) + dist;
    //     keys.remove(key);

    //     best = best.min(score);
    // }

    // if best > std::usize::MAX / 2 {
    //     panic!();
    // }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("12\n"), 2);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 4270);
    }
}
