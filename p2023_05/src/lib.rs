use std::collections::BTreeMap;

struct Mapping {
    src: usize,
    dest: usize,
    size: usize,
}

pub fn part_a(input: &str) -> usize {
    let mut steps = Vec::new();

    let mut lines = input.trim().split('\n');
    let seeds = lines.next().unwrap().split_once(": ").unwrap().1;
    let seeds = seeds
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    lines.next();

    while let Some(map_type) = lines.next() {
        let _map_type = map_type.split_once(" map:").unwrap().0;
        let mut these_mappings = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mapping = line
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            these_mappings.push(Mapping {
                dest: mapping[0],
                src: mapping[1],
                size: mapping[2],
            });
        }
        steps.push(these_mappings);
    }

    let mut options = seeds;
    for step in &steps {
        options = options
            .iter()
            .map(|option| {
                for mapping in step {
                    if *option >= mapping.src && *option < mapping.src + mapping.size {
                        return mapping.dest + (option - mapping.src);
                    }
                }
                return *option;
            })
            .collect();
    }

    *options.iter().min().unwrap()
}

pub fn part_b(input: &str) -> usize {
    let mut steps = Vec::new();

    let mut lines = input.trim().split('\n');
    let seeds = lines.next().unwrap().split_once(": ").unwrap().1;
    let seeds = seeds
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<_>>();
    lines.next();

    while let Some(map_type) = lines.next() {
        let _map_type = map_type.split_once(" map:").unwrap().0;
        let mut these_mappings = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mapping = line
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            these_mappings.push(Mapping {
                dest: mapping[0],
                src: mapping[1],
                size: mapping[2],
            });
        }
        steps.push(these_mappings);
    }

    let mut options = seeds;
    for step in &steps {
        let mut next_options = vec![];
        let mut pts = BTreeMap::new();

        for (start, option_size) in &options {
            let end = start + option_size - 1; // inclusive

            pts.insert(*start, true);
            pts.insert(end, false);

            for Mapping {
                dest,
                src: map_start,
                size: map_size,
            } in step
            {
                let map_end = map_start + map_size - 1; // inclusive
                let overlap_start = *map_start.max(start);
                // eprintln!("Considering {}-{} vs {}-{}", start, end, map_start, map_end);
                if overlap_start > map_end || overlap_start > end {
                    continue;
                }
                let overlap_end = map_end.min(end);
                if overlap_end < *map_start || overlap_end < *start {
                    continue;
                }
                // eprintln!(
                //     " Get us {}-{}",
                //     dest + (overlap_start - map_start),
                //     overlap_start + overlap_end - overlap_start + 1 - 1
                // );
                next_options.push((
                    dest + (overlap_start - map_start),
                    overlap_end - overlap_start + 1,
                ));
                pts.insert(overlap_start, false);
                if overlap_end + 1 <= end {
                    pts.insert(overlap_end + 1, true);
                }
            }
        }
        // eprintln!(" {:?}", pts);
        let mut on = None;
        for (position, change_to) in &pts {
            if let Some(on) = on {
                if !*change_to {
                    // eprintln!(" adding {}-{}", on, position - on);
                    next_options.push((on, position - on));
                }
            }
            on = if *change_to { Some(*position) } else { None };
        }
        options = next_options;
    }

    *options.iter().map(|(a, _)| a).min().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            35
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 88151870);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            46
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2008785);
    }
}
