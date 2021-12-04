use std::collections::{HashMap, HashSet};

pub fn part_a(input: &str) -> i32 {
    let mut lines = input.trim().split('\n');

    let calls: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut called: Vec<HashSet<(i32, i32)>> = Vec::new();

    while let Some(_) = lines.next() {
        let mut board: HashMap<i32, (i32, i32)> = HashMap::new();
        for y in 0..5 {
            for (x, num) in lines.next().unwrap().split(' ').enumerate() {
                let num: i32 = num.parse().unwrap();
                board.insert(num, (y, x as i32));
            }
        }
        boards.push(board);
        called.push(HashSet::new());
    }

    for call in calls {
        for (i, board) in boards.iter_mut().enumerate() {
            if let Some((y, x)) = board.get_mut(&call) {
                called.get_mut(i).unwrap().insert((*y, *x));
            }
        }

        for (i, called) in called.iter().enumerate() {
            let mut bingo = false;
            // columns
            for y in 0..5 {
                let mut col_filled = true;
                for x in 0..5 {
                    if !called.contains(&(y, x)) {
                        col_filled = false;
                        break;
                    }
                }
                if col_filled {
                    bingo = true;
                }
            }
            // rows
            for x in 0..5 {
                let mut col_filled = true;
                for y in 0..5 {
                    if !called.contains(&(y, x)) {
                        col_filled = false;
                        break;
                    }
                }
                if col_filled {
                    bingo = true;
                }
            }

            if bingo {
                let mut score = 0;
                for (num, (y, x)) in boards[i].iter() {
                    if !called.contains(&(*y, *x)) {
                        score += num;
                    }
                }
                return score * call;
            }
        }
    }

    0
}

pub fn part_b(input: &str) -> i32 {
    let mut lines = input.trim().split('\n');

    let calls: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut called: Vec<HashSet<(i32, i32)>> = Vec::new();

    while let Some(_) = lines.next() {
        let mut board: HashMap<i32, (i32, i32)> = HashMap::new();
        for y in 0..5 {
            for (x, num) in lines.next().unwrap().split(' ').enumerate() {
                let num: i32 = num.parse().unwrap();
                board.insert(num, (y, x as i32));
            }
        }
        boards.push(board);
        called.push(HashSet::new());
    }

    for call in calls {
        for (i, board) in boards.iter_mut().enumerate() {
            if let Some((y, x)) = board.get_mut(&call) {
                called.get_mut(i).unwrap().insert((*y, *x));
            }
        }

        let mut is_to_remove = Vec::new();
        for (i, called) in called.iter().enumerate() {
            let mut bingo = false;
            // columns
            for y in 0..5 {
                let mut col_filled = true;
                for x in 0..5 {
                    if !called.contains(&(y, x)) {
                        col_filled = false;
                        break;
                    }
                }
                if col_filled {
                    bingo = true;
                }
            }
            // rows
            for x in 0..5 {
                let mut col_filled = true;
                for y in 0..5 {
                    if !called.contains(&(y, x)) {
                        col_filled = false;
                        break;
                    }
                }
                if col_filled {
                    bingo = true;
                }
            }

            if bingo {
                if boards.len() == 1 {
                    let mut score = 0;
                    for (num, (y, x)) in boards[i].iter() {
                        if !called.contains(&(*y, *x)) {
                            score += num;
                        }
                    }
                    return score * call;
                } else {
                    is_to_remove.push(i);
                }
            }
        }

        is_to_remove.reverse();

        for i in is_to_remove {
            called.remove(i);
            boards.remove(i);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 4662);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 12080);
    }
}
