use std::collections::{HashSet, HashMap};

pub fn part_a(input: &str) -> i64 {
    let mut total = 0;
    for line in input.trim().split('\n') {
        let (_card_id, card) = line.split_once(":").unwrap();
        let card = card.trim();
        let (win, mine) = card.split_once(" | ").unwrap();
        let win = win
            .trim()
            .split(" ")
            .filter(|w| !w.is_empty())
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<HashSet<_>>();
        let mut pts = 0;
        for mine in mine
            .trim()
            .split(" ")
            .filter(|w| !w.is_empty())
            .map(|num| num.parse::<i64>().unwrap())
        {
            if win.contains(&mine) {
                if pts == 0 {
                    pts = 1;
                } else {
                    pts *= 2;
                }
            }
        }
        total += pts;
    }
    total
}

pub fn part_b(input: &str) -> usize {
    let mut total = 0;
    let mut counts = HashMap::new();

    for (line_id, line) in input.trim().split('\n').enumerate() {
        let copies = *counts.get(&line_id).unwrap_or(&1);

        let (_card_id, card) = line.split_once(":").unwrap();
        let card = card.trim();
        let (win, mine) = card.split_once(" | ").unwrap();
        let win = win
            .trim()
            .split(" ")
            .filter(|w| !w.is_empty())
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<HashSet<_>>();
        let mut lines = 0;
        for mine in mine
            .trim()
            .split(" ")
            .filter(|w| !w.is_empty())
            .map(|num| num.parse::<i64>().unwrap())
        {
            if win.contains(&mine) {
                lines += 1;
            }
        }

        for i in (line_id+1)..=(line_id+lines) {
            *counts.entry(i).or_insert(1) += copies;
        }


        total += copies;
    }

    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            13
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 21568);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 11827296);
    }
}
