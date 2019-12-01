use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_tiles_per_square(input: &str) -> HashMap<(u64, u64), Vec<u64>> {
    let mut set_tiles: HashMap<(u64, u64), Vec<u64>> = HashMap::new();
    let line_re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    for line in input.split('\n').filter(|line| line != &"") {
        let captures = line_re.captures(line).expect("Invalid line");

        let id = captures[1].parse::<u64>().expect("Invalid number");
        let start_y = captures[2].parse::<u64>().expect("Invalid number");
        let start_x = captures[3].parse::<u64>().expect("Invalid number");
        let height = captures[4].parse::<u64>().expect("Invalid number");
        let width = captures[5].parse::<u64>().expect("Invalid number");

        for y in start_y..start_y + height {
            for x in start_x..start_x + width {
                set_tiles.entry((x, y)).or_insert(Vec::new()).push(id);
            }
        }
    }

    set_tiles
}

pub fn part_a(input: &str) -> u64 {
    get_tiles_per_square(input).iter().fold(
        0,
        |memo, item| if item.1.len() > 1 { memo + 1 } else { memo },
    )
}

pub fn part_b(input: &str) -> Option<u64> {
    let mut overlaps: HashSet<u64> = HashSet::new();
    let mut all_ids: HashSet<u64> = HashSet::new();

    for (_, tiles) in get_tiles_per_square(input) {
        for tile in &tiles {
            if tiles.len() > 1 {
                overlaps.insert(*tile);
            }
            all_ids.insert(*tile);
        }
    }

    let candidates: Vec<&u64> = all_ids.difference(&overlaps).collect();
    if candidates.len() == 1 {
        Some(*candidates[0])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"),
            4
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 103482);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"),
            Some(3)
        )
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), Some(686));
    }
}
