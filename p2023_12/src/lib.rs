use std::collections::HashMap;

fn count_arrangements<'a> (
    damaged: &'a [char],
    lengths: &'a [i64],
    memo: &mut HashMap<(&'a [char], &'a [i64]), i64>,
) -> i64 {
    if let Some(count) = memo.get(&(damaged, lengths)) {
        return *count;
    }

    let mut count = 0;
    if damaged.len() == 0 {
        return if lengths.len() == 0 { 1 } else { 0 };
    }
    if damaged[0] == '.' || damaged[0] == '?' {
        // Place '.'
        count += count_arrangements(&damaged[1..], lengths, memo);
    }
    if (damaged[0] == '?' || damaged[0] == '#') && lengths.len() > 0 {
        // Place '#'
        let length = lengths[0];
        let mut ok = true;
        for i in 0..length {
            if *damaged.get(i as usize).unwrap_or(&'.') == '.' {
                ok = false;
                break;
            }
        }
        if *damaged.get(length as usize).unwrap_or(&'.') == '#' {
            ok = false;
        }
        // eprintln!("is ok? {} {:?} {:?}", ok, damaged, lengths);
        if ok {
            count += count_arrangements(
                &damaged[((length as usize + 1).min(damaged.len()))..],
                &lengths[1..],
                memo,
            );
        }
    }
    memo.insert((damaged, lengths), count);
    count
}

pub fn part_a(input: &str) -> i64 {
    let mut count = 0;
    for line in input.trim().split('\n') {
        let (damaged, lengths) = line.split_once(" ").unwrap();
        let damaged = damaged.chars().collect::<Vec<_>>();
        let lengths = lengths
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        count += count_arrangements(&damaged, &lengths, &mut HashMap::new());
    }
    count
}

pub fn part_b(input: &str) -> i64 {
    let mut count = 0;
    for line in input.trim().split('\n') {
        let (damaged, lengths) = line.split_once(" ").unwrap();
        let damaged_t = damaged.chars().collect::<Vec<_>>();
        let lengths_t = lengths
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut damaged = Vec::new();
        let mut lengths = Vec::new();
        for i in 0..5 {
            damaged.extend(damaged_t.iter());
            if i < 4 {
                damaged.push('?');
            }
            lengths.extend(lengths_t.iter());
        }
        count += count_arrangements(&damaged, &lengths, &mut HashMap::new());
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn example0() {
        assert_eq!(super::part_a("?###???????? 3,2,1\n"), 10);
    }

    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1\n"
            ),
            21
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1\n"
            ),
            525152
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 7939);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 850504257483930);
    }
}
