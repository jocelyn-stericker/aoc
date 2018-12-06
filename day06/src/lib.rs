use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pt(i32, i32);

fn get_pts(input: &str) -> Vec<Pt> {
    input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| {
            let line: Vec<i32> = line
                .split(", ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            Pt(line[0], line[1])
        })
        .collect()
}

fn get_floodfill(input: &str) -> Vec<i32> {
    let pts = get_pts(input);

    let min_x = pts.iter().min_by_key(|p| p.0).unwrap().0;
    let min_y = pts.iter().min_by_key(|p| p.1).unwrap().1;
    let max_x = pts.iter().max_by_key(|p| p.0).unwrap().0;
    let max_y = pts.iter().max_by_key(|p| p.1).unwrap().1;

    let mut out_of_bounds: HashSet<i32> = HashSet::new();
    let mut color: HashMap<Pt, (u32, HashSet<i32>)> = HashMap::new();
    let mut next_generation: HashMap<Pt, HashSet<i32>> = HashMap::new();

    for (i, pt) in pts.iter().enumerate() {
        let mut set = HashSet::new();
        set.insert(i as i32);
        color.insert(*pt, (0, set.clone()));
        next_generation.insert(*pt, set);
    }

    let directions = vec![Pt(-1, 0), Pt(1, 0), Pt(0, -1), Pt(0, 1)];

    for generation in 1.. {
        let mut did_something = false;
        let this_generation = next_generation.clone();
        next_generation.clear();
        for (pt, ids) in this_generation.iter() {
            if ids.iter().len() != 1 {
                continue;
            }

            let i = ids.iter().next().unwrap();

            for direction in &directions {
                let npt = Pt(pt.0 + direction.0, pt.1 + direction.1);
                if npt.0 < min_x || npt.0 > max_x || npt.1 < min_y || npt.1 > max_y {
                    if !out_of_bounds.contains(i) {
                        did_something = true;
                        out_of_bounds.insert(*i);
                    }
                } else {
                    let entry = color
                        .entry(npt)
                        .or_insert_with(|| (generation, HashSet::new()));
                    if generation != entry.0 {
                        continue;
                    }

                    if !entry.1.contains(i) {
                        did_something = true;
                        entry.1.insert(*i);
                    }

                    // Also process this next iteration.
                    next_generation
                        .entry(npt)
                        .or_insert_with(HashSet::new)
                        .insert(*i);
                }
            }
        }

        if !did_something {
            break;
        }
    }

    color
        .iter()
        .filter_map(|(_pt, (_gen, ids))| {
            if ids.len() != 1 {
                None
            } else {
                let id = ids.iter().next().unwrap();
                if out_of_bounds.contains(id) {
                    None
                } else {
                    Some(*id)
                }
            }
        })
        .collect()
}

pub fn part_a(input: &str) -> u64 {
    *get_floodfill(input)
        .iter()
        .fold(HashMap::<i32, u64>::new(), |mut memo, id| {
            *memo.entry(*id).or_insert(0) += 1;

            memo
        })
        .iter()
        .max_by_key(|m| m.1)
        .unwrap()
        .1
}

pub fn part_b(input: &str, max_distance: i32) -> usize {
    let pts = get_pts(input);

    let extra_bounds = max_distance / (pts.len() as i32) + 1;

    let min_x = pts.iter().min_by_key(|p| p.0).unwrap().0;
    let min_y = pts.iter().min_by_key(|p| p.1).unwrap().1;
    let max_x = pts.iter().max_by_key(|p| p.0).unwrap().0;
    let max_y = pts.iter().max_by_key(|p| p.1).unwrap().1;

    let mut safe_pts = vec![];

    for x in (min_x - extra_bounds)..=(max_x + extra_bounds) {
        for y in (min_y - extra_bounds)..=(max_y + extra_bounds) {
            let mut score = 0;
            for pt in &pts {
                score += (pt.0 - x).abs() + (pt.1 - y).abs();
            }

            if score < max_distance {
                safe_pts.push(Pt(x, y));
            }
        }
    }

    safe_pts.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a("1, 1\n1, 6\n\n8, 3\n3, 4\n5, 5\n8, 9\n\n"),
            17
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 5975);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b("1, 1\n1, 6\n\n8, 3\n3, 4\n5, 5\n8, 9\n\n", 32),
            16
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt"), 10000), 38670);
    }
}
