use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
enum Tile {
    Wall,
    Space,
    Goal(i32),
}

fn shortest(
    at: i32,
    remaining: &mut HashSet<i32>,
    distances: &HashMap<(i32, i32), i32>,
    return_to: Option<i32>,
) -> i32 {
    let mut best = i32::MAX;
    if remaining.len() == 1 {
        if let Some(return_to) = return_to {
            return distances[&(at, return_to)];
        } else {
            return 0;
        }
    }
    remaining.remove(&at);
    for d in remaining.clone() {
        best = best.min(distances[&(at, d)] + shortest(d, remaining, distances, return_to));
    }
    remaining.insert(at);
    best
}

pub fn part_a(input: &str, return_to: Option<i32>) -> i32 {
    let mut world = HashMap::new();
    let mut digits = HashMap::new();
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            world.insert(
                (y as i32, x as i32),
                match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Space,
                    d => {
                        let d = d.to_digit(10).unwrap() as i32;
                        digits.insert(d, (y as i32, x as i32));
                        Tile::Goal(d)
                    }
                },
            );
        }
    }
    let mut distances = HashMap::new();
    for (d, (y, x)) in digits.iter() {
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back((*y, *x, 0));
        visited.insert((*y, *x));

        while let Some((y, x, i)) = to_visit.pop_front() {
            visited.insert((y, x));
            for (dy, dx) in [(-1_i32, 0_i32), (0, -1), (1, 0), (0, 1)] {
                if visited.contains(&(y + dy, x + dx)) {
                    continue;
                }
                visited.insert((y + dy, x + dx));
                match world.get(&(y + dy, x + dx)) {
                    None | Some(Tile::Wall) => {}
                    Some(Tile::Space) => {
                        to_visit.push_back((y + dy, x + dx, i + 1));
                    }
                    Some(Tile::Goal(other_digit)) => {
                        to_visit.push_back((y + dy, x + dx, i + 1));
                        distances.insert((*d, *other_digit), i + 1);
                    }
                };
            }
        }
    }

    let mut to_visit: HashSet<_> = digits.keys().copied().collect();
    dbg!(&to_visit);

    shortest(0, &mut to_visit, &distances, return_to)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "###########
#0.1.....2#
#.#######.#
#4.......3#
###########",
                None
            ),
            14
        );
    }

    #[test]
    fn part_a() {
        //457 is too low
        assert_eq!(super::part_a(include_str!("input.txt"), None), 464);
    }

    #[test]
    fn part_b() {
        //457 is too low
        assert_eq!(super::part_a(include_str!("input.txt"), Some(0)), 652);
    }
}
