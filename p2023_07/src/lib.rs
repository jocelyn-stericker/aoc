use std::collections::HashMap;

#[derive(PartialOrd, Ord, Eq, PartialEq)]
enum Kind {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn part_a(input: &str) -> i64 {
    let mut values = HashMap::new();
    for (i, card) in [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ]
    .iter()
    .rev()
    .enumerate()
    {
        values.insert(card, i as i64);
    }

    let mut hands = Vec::new();

    for line in input.trim().split('\n') {
        let (hand, pts) = line.split_once(" ").unwrap();
        let pts = pts.parse::<i64>().unwrap();
        let hand = hand.chars().map(|c| values[&c]).collect::<Vec<_>>();

        let mut counts = HashMap::new();
        for card in hand.iter() {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }

        let kind = if counts.len() == 1 {
            Kind::FiveOfAKind
        } else if counts.len() == 2 {
            if counts.values().any(|&count| count == 4) {
                Kind::FourOfAKind
            } else {
                Kind::FullHouse
            }
        } else if counts.len() == 3 {
            if counts.values().any(|&count| count == 3) {
                Kind::ThreeOfAKind
            } else {
                Kind::TwoPair
            }
        } else if counts.len() == 4 {
            Kind::OnePair
        } else if counts.len() == 5 {
            Kind::HighCard
        } else {
            unreachable!();
        };

        hands.push((kind, hand, pts))
    }

    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, pts))| pts * (i as i64 + 1))
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let mut values = HashMap::new();
    for (i, card) in [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ]
    .iter()
    .rev()
    .enumerate()
    {
        values.insert(card, i as i64);
    }

    let mut hands = Vec::new();

    for line in input.trim().split('\n') {
        let (hand, pts) = line.split_once(" ").unwrap();
        let pts = pts.parse::<i64>().unwrap();
        let hand = hand.chars().map(|c| values[&c]).collect::<Vec<_>>();

        let mut best_kind = Kind::HighCard;
        for jokers_are_now in values.values() {
            let mut counts = HashMap::new();
            for card in hand.iter() {
                let card = if *card == values[&'J'] { jokers_are_now } else { card };
                let count = counts.entry(card).or_insert(0);
                *count += 1;
            }

            let kind = if counts.len() == 1 {
                Kind::FiveOfAKind
            } else if counts.len() == 2 {
                if counts.values().any(|&count| count == 4) {
                    Kind::FourOfAKind
                } else {
                    Kind::FullHouse
                }
            } else if counts.len() == 3 {
                if counts.values().any(|&count| count == 3) {
                    Kind::ThreeOfAKind
                } else {
                    Kind::TwoPair
                }
            } else if counts.len() == 4 {
                Kind::OnePair
            } else if counts.len() == 5 {
                Kind::HighCard
            } else {
                unreachable!();
            };

            best_kind = kind.max(best_kind);
        }

        hands.push((best_kind, hand, pts))
    }

    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, pts))| pts * (i as i64 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483\n"
            ),
            6440
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 248569531);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483\n"
            ),
            5905
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 250382098);
    }
}
