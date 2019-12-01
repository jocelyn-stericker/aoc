use linked_list::{Cursor, LinkedList};
use std::collections::HashMap;

pub fn next_circular<T: Copy>(cursor: &mut Cursor<T>) -> T {
    if let Some(t) = cursor.next() {
        *t
    } else {
        *cursor.next().unwrap()
    }
}

pub fn prev_circular<T: Copy>(cursor: &mut Cursor<T>) -> T {
    if let Some(t) = cursor.prev() {
        *t
    } else {
        *cursor.prev().unwrap()
    }
}

pub fn part_a(players: u32, last_marble: u32) -> u32 {
    let mut scores: HashMap<u32, u32> = HashMap::new();

    let mut ll: LinkedList<u32> = LinkedList::new();
    let mut cursor = ll.cursor();

    let mut player = 0;
    cursor.insert(0);

    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            for _ in 0..7 {
                prev_circular(&mut cursor);
            }
            *scores.entry(player).or_insert(0) += marble + cursor.remove().unwrap();
        } else {
            next_circular(&mut cursor);
            next_circular(&mut cursor);
            cursor.insert(marble);
        }

        player = (player + 1) % players;
    }

    *scores.values().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a(9, 3), 0);
        assert_eq!(super::part_a(9, 10), 0);
        assert_eq!(super::part_a(9, 25), 32);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_a(10, 1618), 8317);
    }

    #[test]
    fn example3() {
        assert_eq!(super::part_a(13, 7999), 146373);
    }

    #[test]
    fn example4() {
        assert_eq!(super::part_a(17, 1104), 2764);
    }

    #[test]
    fn example5() {
        assert_eq!(super::part_a(21, 6111), 54718);
    }

    #[test]
    fn example6() {
        assert_eq!(super::part_a(30, 5807), 37305);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(462, 71938), 398371);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_a(462, 71938 * 100), 3212830280);
    }
}
