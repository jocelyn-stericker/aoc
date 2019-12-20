use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    Wall,
    Space,
    Portal(char, char),
}

fn left(hm: &HashMap<(usize, usize), Cell>, x: usize, y: usize) -> Option<(usize, usize, &Cell)> {
    if x == 0 {
        return None;
    }
    hm.get(&(x - 1, y)).map(|v| (x - 1, y, v))
}

fn up(hm: &HashMap<(usize, usize), Cell>, x: usize, y: usize) -> Option<(usize, usize, &Cell)> {
    if y == 0 {
        return None;
    }
    hm.get(&(x, y - 1)).map(|v| (x, y - 1, v))
}

fn right(hm: &HashMap<(usize, usize), Cell>, x: usize, y: usize) -> Option<(usize, usize, &Cell)> {
    hm.get(&(x + 1, y)).map(|v| (x + 1, y, v))
}

fn down(hm: &HashMap<(usize, usize), Cell>, x: usize, y: usize) -> Option<(usize, usize, &Cell)> {
    hm.get(&(x, y + 1)).map(|v| (x, y + 1, v))
}

pub fn part_a(input: &str) -> usize {
    let raw_maze: Vec<Vec<char>> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut cells: HashMap<(usize, usize), Cell> = HashMap::new();
    let mut jump: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut tmp_portals: HashMap<(char, char), (usize, usize)> = HashMap::new();

    for (y, row) in raw_maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_ascii_uppercase() {
                if let Some(next) = raw_maze[y].get(x + 1) {
                    if next.is_ascii_uppercase() {
                        if cell > next {
                            if raw_maze[y].get(x + 2).filter(|c| **c != ' ').is_some() {
                                cells.insert((x + 1, y), Cell::Portal(*next, *cell));
                            } else {
                                cells.insert((x, y), Cell::Portal(*next, *cell));
                            }
                        } else {
                            if raw_maze[y].get(x + 2).filter(|c| **c != ' ').is_some() {
                                cells.insert((x + 1, y), Cell::Portal(*cell, *next));
                            } else {
                                cells.insert((x, y), Cell::Portal(*cell, *next));
                            }
                        }
                    }
                }
                if let Some(next) = raw_maze.get(y + 1).map(|row| row[x]) {
                    if next.is_ascii_uppercase() {
                        if *cell > next {
                            if raw_maze
                                .get(y + 2)
                                .map(|row| row[x])
                                .filter(|c| *c != ' ')
                                .is_some()
                            {
                                cells.insert((x, y + 1), Cell::Portal(next, *cell));
                            } else {
                                cells.insert((x, y), Cell::Portal(next, *cell));
                            }
                        } else {
                            if raw_maze
                                .get(y + 2)
                                .map(|row| row[x])
                                .filter(|c| *c != ' ')
                                .is_some()
                            {
                                cells.insert((x, y + 1), Cell::Portal(*cell, next));
                            } else {
                                cells.insert((x, y), Cell::Portal(*cell, next));
                            }
                        }
                    }
                }
                if let Some(Cell::Portal(a, b)) = cells.get(&(x, y)) {
                    if let Some(other) = tmp_portals.remove(&(*a, *b)) {
                        jump.insert((x, y), other);
                        jump.insert(other, (x, y));
                    } else {
                        tmp_portals.insert((*a, *b), (x, y));
                    }
                }
            } else if *cell == '.' {
                cells.insert((x, y), Cell::Space);
            } else if *cell == '#' {
                cells.insert((x, y), Cell::Wall);
            }
        }
    }
    // for (y, row) in raw_maze.iter().enumerate() {
    //     for (x, _cell) in row.iter().enumerate() {
    //         if let Some(cell) = cells.get(&(x, y)) {
    //             match cell {
    //                 Cell::Wall => eprint!("#"),
    //                 Cell::Space => eprint!("."),
    //                 Cell::Portal(o, t) => eprint!("{}", t),
    //             }
    //         } else {
    //             eprint!(" ");
    //         }
    //     }
    //     eprintln!();
    // }

    for (y, row) in raw_maze.iter().enumerate() {
        for (x, _cell) in row.iter().enumerate() {
            if let Some(cell) = cells.get(&(x, y)) {
                match cell {
                    Cell::Wall => eprint!("#"),
                    Cell::Space => eprint!("."),
                    Cell::Portal(o, t) => eprint!("{}", t),
                }
            } else {
                eprint!(" ");
            }
        }
        eprintln!();
    }

    // eprintln!("{:?} \n{:?}", jump, jump.get(&(43, 1)));
    // eprintln!("{:?}", tmp_portals);
    let start = tmp_portals.remove(&('A', 'A')).unwrap();
    tmp_portals.remove(&('Z', 'Z')).unwrap();
    assert!(tmp_portals.is_empty());
    std::mem::drop(tmp_portals);

    let mut best: HashMap<(usize, usize), usize> = HashMap::new();
    let mut next: VecDeque<(usize, usize, usize)> = VecDeque::new();
    next.push_back((start.0, start.1, 0));

    while let Some((x, y, score)) = next.pop_front() {
        if let Some(best_score) = best.get(&(x, y)) {
            if *best_score <= score {
                continue;
            }
        }
        // eprintln!("At {:?}", (x, y, score));
        best.insert((x, y), score);
        for others in &[
            left(&cells, x, y),
            up(&cells, x, y),
            right(&cells, x, y),
            down(&cells, x, y),
        ] {
            if let Some(other) = others {
                // eprintln!("Got {:?}", other);
                match other {
                    (_, _, Cell::Wall) => {}
                    (x, y, Cell::Space) => {
                        next.push_back((*x, *y, score + 1));
                    }
                    (x, y, Cell::Portal(a, b)) => {
                        if *a == 'Z' && *b == 'Z' {
                            // -1 for the initial walk out of the portal.
                            return score - 1;
                        } else if *a != 'A' || *b != 'A' {
                            let other = jump.get(&(*x, *y)).unwrap();
                            // Costs 1 to jump, if we incr here that takes 2.
                            // eprintln!("PORTAL {:?} -> {:?}", (x, y), other);
                            next.push_front((other.0, other.1, score));
                        }
                    }
                }
            }
        }
    }

    panic!();
}

pub fn part_b(input: &str) -> usize {
    let raw_maze: Vec<Vec<char>> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut cells: HashMap<(usize, usize), Cell> = HashMap::new();
    let mut jump: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut tmp_portals: HashMap<(char, char), (usize, usize)> = HashMap::new();

    let rows = raw_maze.len();
    let cols = raw_maze[0].len();

    for (y, row) in raw_maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_ascii_uppercase() {
                if let Some(next) = raw_maze[y].get(x + 1) {
                    if next.is_ascii_uppercase() {
                        if cell > next {
                            if raw_maze[y].get(x + 2).filter(|c| **c != ' ').is_some() {
                                cells.insert((x + 1, y), Cell::Portal(*next, *cell));
                            } else {
                                cells.insert((x, y), Cell::Portal(*next, *cell));
                            }
                        } else {
                            if raw_maze[y].get(x + 2).filter(|c| **c != ' ').is_some() {
                                cells.insert((x + 1, y), Cell::Portal(*cell, *next));
                            } else {
                                cells.insert((x, y), Cell::Portal(*cell, *next));
                            }
                        }
                    }
                }
                if let Some(next) = raw_maze.get(y + 1).map(|row| row[x]) {
                    if next.is_ascii_uppercase() {
                        if *cell > next {
                            if raw_maze
                                .get(y + 2)
                                .map(|row| row[x])
                                .filter(|c| *c != ' ')
                                .is_some()
                            {
                                cells.insert((x, y + 1), Cell::Portal(next, *cell));
                            } else {
                                cells.insert((x, y), Cell::Portal(next, *cell));
                            }
                        } else {
                            if raw_maze
                                .get(y + 2)
                                .map(|row| row[x])
                                .filter(|c| *c != ' ')
                                .is_some()
                            {
                                cells.insert((x, y + 1), Cell::Portal(*cell, next));
                            } else {
                                cells.insert((x, y), Cell::Portal(*cell, next));
                            }
                        }
                    }
                }
                if let Some(Cell::Portal(a, b)) = cells.get(&(x, y)) {
                    if let Some(other) = tmp_portals.remove(&(*a, *b)) {
                        jump.insert((x, y), other);
                        jump.insert(other, (x, y));
                    } else {
                        tmp_portals.insert((*a, *b), (x, y));
                    }
                }
            } else if *cell == '.' {
                cells.insert((x, y), Cell::Space);
            } else if *cell == '#' {
                cells.insert((x, y), Cell::Wall);
            }
        }
    }
    // for (y, row) in raw_maze.iter().enumerate() {
    //     for (x, _cell) in row.iter().enumerate() {
    //         if let Some(cell) = cells.get(&(x, y)) {
    //             match cell {
    //                 Cell::Wall => eprint!("#"),
    //                 Cell::Space => eprint!("."),
    //                 Cell::Portal(o, t) => eprint!("{}", t),
    //             }
    //         } else {
    //             eprint!(" ");
    //         }
    //     }
    //     eprintln!();
    // }

    for (y, row) in raw_maze.iter().enumerate() {
        for (x, _cell) in row.iter().enumerate() {
            if let Some(cell) = cells.get(&(x, y)) {
                match cell {
                    Cell::Wall => eprint!("#"),
                    Cell::Space => eprint!("."),
                    Cell::Portal(o, t) => eprint!("{}", t),
                }
            } else {
                eprint!(" ");
            }
        }
        eprintln!();
    }

    // eprintln!("{:?} \n{:?}", jump, jump.get(&(43, 1)));
    // eprintln!("{:?}", tmp_portals);
    let start = tmp_portals.remove(&('A', 'A')).unwrap();
    tmp_portals.remove(&('Z', 'Z')).unwrap();
    assert!(tmp_portals.is_empty());
    std::mem::drop(tmp_portals);

    let mut best: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut next: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
    next.push_back((start.0, start.1, 0, 0));

    while let Some((x, y, score, lvl)) = next.pop_front() {
        if let Some(best_score) = best.get(&(x, y, lvl)) {
            if *best_score <= score {
                continue;
            }
        }
        // eprintln!("At {:?}", (x, y, score));
        best.insert((x, y, lvl), score);
        for others in &[
            left(&cells, x, y),
            up(&cells, x, y),
            right(&cells, x, y),
            down(&cells, x, y),
        ] {
            if let Some(other) = others {
                // eprintln!("Got {:?}", other);
                match other {
                    (_, _, Cell::Wall) => {}
                    (x, y, Cell::Space) => {
                        next.push_back((*x, *y, score + 1, lvl));
                    }
                    (x, y, Cell::Portal(a, b)) => {
                        if *a == 'Z' && *b == 'Z' {
                            // -1 for the initial walk out of the portal.
                            if lvl == 0 {
                                return score - 1;
                            }
                        } else if *a != 'A' || *b != 'A' && lvl != 0 {
                            let other = jump.get(&(*x, *y)).unwrap();
                            // Costs 1 to jump, if we incr here that takes 2.
                            // eprintln!("PORTAL {:?} -> {:?} ({} {}) {}", (x, y), other, a, b, score);
                            if *x <= 2 || *y <= 2 || *x >= cols - 3 || *y >= rows - 3 {
                                if lvl > 0 {
                                    // eprintln!("Level out {} ", lvl - 1);
                                    next.push_front((other.0, other.1, score, lvl - 1));
                                }
                            } else {
                                // eprintln!("Level in -> {}", lvl + 1);
                                next.push_front((other.0, other.1, score, lvl + 1));
                            }
                        }
                    }
                }
            }
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 642);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 7492);
    }
}
