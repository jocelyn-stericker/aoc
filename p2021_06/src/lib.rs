use std::collections::HashMap;

pub fn num_fishes(mut state: i64, days: usize, memo: &mut HashMap<(i64, usize), usize>) -> usize {
    if let Some(result) = memo.get(&(state, days)) {
        return *result;
    }

    let orig_state = state;
    let mut count = 1;
    for d in 0..days {
        state -= 1;
        if state == -1 {
            state = 6;
            count += num_fishes(8, days - d - 1, memo)
        }
    }

    memo.insert((orig_state, days), count);
    count
}

pub fn part_a(input: &str, days: usize) -> usize {
    let fish: Vec<i64> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut count = 0;
    let mut memo = HashMap::new();
    for fish in &fish {
        let count_from_fish = num_fishes(*fish, days, &mut memo);
        count += count_from_fish;
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("3,4,3,1,2\n", 80), 5934);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt"), 80), 352195);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_a(include_str!("input.txt"), 256), 1600306001288);
    }
}
