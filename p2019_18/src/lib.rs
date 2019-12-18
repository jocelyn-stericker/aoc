use std::collections::{BTreeSet, HashMap, VecDeque};

pub fn part_b(input: &str) -> usize {
    let maze: Vec<Vec<char>> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.chars().collect())
        .collect();

    let h = maze.len();
    let w = maze[0].len();

    let mut starts: Vec<(i64, i64)> = Vec::new();
    let mut all_keys: BTreeSet<char> = BTreeSet::new();
    for x in 0..w {
        for y in 0..h {
            if maze[y][x] == '@' {
                starts.push((x as i64, y as i64));
            }
            if maze[y][x] != '.' && maze[y][x] != '#' && maze[y][x].is_ascii_lowercase() {
                all_keys.insert(maze[y][x]);
            }
        }
    }
    eprintln!("{:?}", all_keys.len());

    let h = maze.len();
    let mut multiverses = 0;
    let w = maze[0].len();
    let mut seen = HashMap::new();
    let mut visit = VecDeque::new();
    let mut visited: Vec<Vec<((i64, i64), usize, usize, usize, BTreeSet<char>)>> = vec![vec![]];
    for (i, s) in starts.into_iter().enumerate() {
        visited[0].push((s, i, 0, 0, BTreeSet::new()));
        visit.push_back((s, i, 0, 0, BTreeSet::new()));
    }
    let mut longest_chain = 0;

    while let Some((pos, universe, multiverse, i, keys)) = visit.pop_front() {
        while i >= visited.len() {
            visited.push(vec![]);
        }

        if pos.0 < 0 || pos.1 < 0 || pos.0 > w as i64 || pos.1 > h as i64 {
            continue;
        }
        if seen.contains_key(&(pos, keys.clone())) {
            continue;
        }
        seen.insert((pos, keys.clone()), i);

        let c = maze[pos.1 as usize][pos.0 as usize];
        // eprintln!("{:?} {:?} {:?}", pos, universe, c);
        if c.is_ascii_lowercase() && !keys.contains(&c) {
            let mut nkeys = keys.clone();
            if keys.len() > longest_chain {
                longest_chain = keys.len();
            }
            nkeys.insert(c);
            seen.insert((pos, nkeys.clone()), i);
            eprintln!(
                "Got key in mv {}: {} (i = {}) {:?} {:?} -> {}",
                multiverse,
                c,
                i,
                pos,
                nkeys,
                multiverses + 1
            );
            multiverses += 1;
            visited[i].push(((pos.0, pos.1), universe, multiverses, i, nkeys.clone()));

            let mut to_start = VecDeque::new();
            for t in 0..i {
                for to_visit in &visited[t] {
                    if to_visit.1 != universe && to_visit.2 == multiverse {
                        eprintln!(
                            "Sharing with universe {} {} {:?}",
                            to_visit.1, i, to_visit.0
                        );
                        to_start.push_back((to_visit.0, to_visit.1, multiverses, i, nkeys.clone()));
                    }
                }
            }
            visit.push_back((
                (pos.0 + 1, pos.1),
                universe,
                multiverses,
                i + 1,
                nkeys.clone(),
            ));
            visit.push_back((
                (pos.0 - 1, pos.1),
                universe,
                multiverses,
                i + 1,
                nkeys.clone(),
            ));
            visit.push_back((
                (pos.0, pos.1 + 1),
                universe,
                multiverses,
                i + 1,
                nkeys.clone(),
            ));
            visit.push_back(((pos.0, pos.1 - 1), universe, multiverses, i + 1, nkeys));
            for new in to_start.into_iter() {
                visit.push_back((
                    ((new.0).0 + 1, (new.0).1),
                    new.1,
                    new.2,
                    new.3 + 1,
                    new.4.clone(),
                ));
                visit.push_back((
                    ((new.0).0 - 1, (new.0).1),
                    new.1,
                    new.2,
                    new.3 + 1,
                    new.4.clone(),
                ));
                visit.push_back((
                    ((new.0).0, (new.0).1 - 1),
                    new.1,
                    new.2,
                    new.3 + 1,
                    new.4.clone(),
                ));
                visit.push_back((
                    ((new.0).0, (new.0).1 + 1),
                    new.1,
                    new.2,
                    new.3 + 1,
                    new.4.clone(),
                ));
                visited[i].push((((new.0).0, (new.0).1), new.1, new.2, new.3, new.4.clone()));
            }
        } else if c == '.' || c == '@' || keys.contains(&c.to_ascii_lowercase()) {
            visit.push_back((
                (pos.0 + 1, pos.1),
                universe,
                multiverse,
                i + 1,
                keys.clone(),
            ));
            visit.push_back((
                (pos.0 - 1, pos.1),
                universe,
                multiverse,
                i + 1,
                keys.clone(),
            ));
            visit.push_back((
                (pos.0, pos.1 + 1),
                universe,
                multiverse,
                i + 1,
                keys.clone(),
            ));
            visit.push_back((
                (pos.0, pos.1 - 1),
                universe,
                multiverse,
                i + 1,
                keys.clone(),
            ));
        }
    }

    let mut bs = std::usize::MAX;
    let l = all_keys.len();
    for (k, v) in seen {
        if k.1.len() == l {
            bs = bs.min(v);
        }
    }

    bs
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_b() {
        // 1136 too low.
        assert_eq!(super::part_b(include_str!("input.txt")), 1982);
    }
}
