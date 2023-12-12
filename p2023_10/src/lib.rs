use std::collections::{HashMap, HashSet, VecDeque};

fn parse_graph(input: &str) -> ((i64, i64), HashMap<(i64, i64), Vec<(i64, i64)>>) {
    let mut map: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    let mut start = None;
    for (y, line) in input.trim().split('\n').enumerate() {
        let y = y as i64;
        for (x, char) in line.chars().enumerate() {
            let x = x as i64;
            match char {
                '|' => {
                    map.insert((x, y), vec![(x, y - 1), (x, y + 1)]);
                }
                '-' => {
                    map.insert((x, y), vec![(x - 1, y), (x + 1, y)]);
                }
                'L' => {
                    map.insert((x, y), vec![(x, y - 1), (x + 1, y)]);
                }
                'J' => {
                    map.insert((x, y), vec![(x, y - 1), (x - 1, y)]);
                }
                '7' => {
                    map.insert((x, y), vec![(x, y + 1), (x - 1, y)]);
                }
                'F' => {
                    map.insert((x, y), vec![(x, y + 1), (x + 1, y)]);
                }
                '.' => {}
                'S' => {
                    start = Some((x, y));
                }
                _ => {
                    panic!();
                }
            }
        }
    }
    let start = start.unwrap();
    let mut start_options = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if let Some(options) = map.get(&(start.0 + dx, start.1 + dy)) {
                if options.iter().any(|(x, y)| *x == start.0 && *y == start.1) {
                    start_options.push((start.0 + dx, start.1 + dy));
                }
            }
        }
    }
    map.insert(start, start_options);

    (start, map)
}

pub fn part_a(input: &str) -> i64 {
    let (start, map) = parse_graph(input);

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if visited.contains_key(&pos) {
            continue;
        }
        visited.insert(pos, dist);
        for next in map.get(&pos).unwrap() {
            queue.push_back((*next, dist + 1));
        }
    }
    *visited.values().max().unwrap_or(&0)
}

pub fn part_b(input: &str) -> i64 {
    let (start, map) = parse_graph(input);

    let mut queue = VecDeque::new();
    queue.push_back((start, vec![]));

    let mut mainloop = vec![];
    while let Some((pos, path)) = queue.pop_front() {
        if pos == start && path.len() > mainloop.len() {
            mainloop = path.clone();
        }
        if path.contains(&pos) {
            continue;
        }
        let mut path = path.clone();
        path.push(pos);
        for next in map.get(&pos).unwrap() {
            queue.push_back((*next, path.clone()));
        }
    }


    // filter map to hide anything not in the loop
    let map = map
        .into_iter()
        .filter(|m| mainloop.contains(&m.0))
        .collect::<HashMap<_, _>>();

    let min_x = mainloop.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = mainloop.iter().map(|(x, _)| x).max().unwrap() + 1;
    let min_y = mainloop.iter().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = mainloop.iter().map(|(_, y)| y).max().unwrap() + 1;

    // we're now doing a flood fill at the points between the walls,
    // (1, 1) is the point bordering (0, 0), (1, 0), (0, 1), and (1, 1)
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let mut outside: HashSet<(i64, i64)> = HashSet::new();
    queue.push_back((min_x, min_y));
    while let Some((x, y)) = queue.pop_front() {
        if x < min_x || x > max_x || y < min_y || y > max_y {
            continue;
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        // up
        match map.get(&(x - 1, y - 1)) {
            Some(up_left) if up_left.iter().any(|(ox, oy)| *ox == x && *oy == y - 1) => {}
            _ => {
                outside.insert((x - 1, y - 1));
                outside.insert((x, y - 1));
                queue.push_back((x, y - 1));
            }
        }
        // down
        match map.get(&(x - 1, y)) {
            Some(down_left) if down_left.iter().any(|(ox, oy)| *ox == x && *oy == y) => {}
            _ => {
                outside.insert((x - 1, y));
                outside.insert((x, y));
                queue.push_back((x, y + 1));
            }
        }
        // left
        match map.get(&(x - 1, y - 1)) {
            Some(up_left) if up_left.iter().any(|(ox, oy)| *ox == x - 1 && *oy == y) => {}
            _ => {
                outside.insert((x - 1, y - 1));
                outside.insert((x - 1, y));
                queue.push_back((x - 1, y));
            }
        }
        // right
        match map.get(&(x, y)) {
            Some(down_right) if down_right.iter().any(|(ox, oy)| *ox == x && *oy == y - 1) => {}
            _ => {
                outside.insert((x, y));
                outside.insert((x, y - 1));
                queue.push_back((x + 1, y));
            }
        }
    }

    let mut inside: HashSet<(i64, i64)> = HashSet::new();
    for y in (min_y + 1)..max_y {
        for x in (min_x + 1)..max_x {
            if !outside.contains(&(x, y)) && !mainloop.contains(&(x, y)) {
                inside.insert((x, y));
            }
        }
    }
    inside.len() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                ".....
.S-7.
.|.|.
.L-J.
.....\n"
            ),
            4
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 6738);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            4
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            super::part_b(
                "...........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
            ),
            4
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            super::part_b(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            super::part_b(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        );
    }

    #[test]
    fn example6() {
        assert_eq!(
            super::part_b(
                "........
S-----7
|.....|
|F--7.|
|L-7L7|
L--JLLJ"
            ),
            6
        );
    }

    #[test]
    fn example7() {
        assert_eq!(
            super::part_b(
                "........
S-----7
|.....|
|..F7.|
|.FJL7|
L-J.LLJ"
            ),
            9
        );
    }

    #[test]
    fn part_b() {
        // 632 is too high
        assert_eq!(super::part_b(include_str!("input.txt")), 579);
    }
}
