use modexp::{modexp, BigInt};
use modinverse::modinverse;
use num_integer::Integer;
use num_traits::cast::{FromPrimitive, ToPrimitive};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Act {
    Cut(isize),
    Rev,
    Deal(usize),
}

pub fn part_a(input: &str, times: isize) -> usize {
    let mut deck: VecDeque<usize> = (0..10007).collect();
    let script: Vec<Act> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| {
            if line == "deal into new stack" {
                return Act::Rev;
            }
            if line.starts_with("deal with increment") {
                return Act::Deal(line.split(' ').last().unwrap().parse::<usize>().unwrap());
            }
            if line.starts_with("cut") {
                return Act::Cut(line.split(' ').last().unwrap().parse::<isize>().unwrap());
            }

            eprintln!("{}", line);

            panic!();
        })
        .collect();

    for _ in 0..times {
        for act in &script {
            eprintln!("{:?}", act);
            match act {
                Act::Rev => {
                    deck = deck.into_iter().rev().collect();
                }
                Act::Cut(mut p) => {
                    while p > 0 {
                        let x = deck.pop_front().unwrap();
                        deck.push_back(x);
                        p -= 1;
                    }
                    while p < 0 {
                        let x = deck.pop_back().unwrap();
                        deck.push_front(x);
                        p += 1;
                    }
                }
                Act::Deal(d) => {
                    let other = deck.clone();
                    for i in 0..10007 {
                        deck[i] = 10000000;
                    }
                    let mut pos = 0;
                    for i in 0..10007 {
                        deck[pos] = other[i];
                        pos = (pos + d) % 10007;
                    }
                    for i in 0..10007 {
                        if deck[i] == 10000000 {
                            eprintln!("Bad {}", i);
                            panic!();
                        }
                    }
                }
            }

            for i in 0..10007 {
                if deck[i] == 2019 {
                    eprintln!("@ {}", i);
                }
            }
        }
    }

    // eprintln!("{:?}", deck);

    for i in 0..10007 {
        if deck[i] == 2019 {
            return i;
        }
    }
    panic!();
}

/*
 *
    for act in &script {
        match act {
            Act::Rev => {
                pos = deck_size - 1 - pos;
            }
            Act::Cut(p) => {
                // We're *undoing* it.
                pos = ((pos + *p) + deck_size) % deck_size;
            }
            Act::Deal(d) => {
                let di = modinverse(*d as isize, deck_size).unwrap();
                let np = (pos * di) % (deck_size);
                pos = np;
            }
        }
    }
*/

pub fn part_b(input: &str) -> isize {
    let deck_size: isize = 119315717514047;
    // let deck_size: isize = 10007;
    let script: Vec<Act> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| {
            if line == "deal into new stack" {
                return Act::Rev;
            }
            if line.starts_with("deal with increment") {
                return Act::Deal(line.split(' ').last().unwrap().parse::<usize>().unwrap());
            }
            if line.starts_with("cut") {
                return Act::Cut(line.split(' ').last().unwrap().parse::<isize>().unwrap());
            }

            eprintln!("{}", line);

            panic!();
        })
        .collect();

    let yolo = 101741582076661isize;
    // let yolo = 2isize;

    let mut inc: i128 = 1;
    let mut start: i128 = 0;
    for act in &script {
        match act {
            Act::Rev => {
                // to undo: pos = deck_size - 1 - pos;
                inc *= -1;
                start += inc;
            }
            Act::Cut(p) => {
                let p = *p as i128;
                // to undo: pos = ((pos + *p) + deck_size) % deck_size;
                start = start + inc * p;
            }
            Act::Deal(d) => {
                let d = *d as i128;
                // to undo:
                // let di = modinverse(*d as isize, deck_size).unwrap();
                // let np = (pos * di) % (deck_size);
                // pos = np;
                let di = modinverse(d as i128, deck_size as i128).unwrap();
                inc = (inc * di) % (deck_size as i128);
            }
        }
        inc = (inc + (deck_size as i128)) % (deck_size as i128);
        start = (start + (deck_size as i128)) % (deck_size as i128);

        inc = (inc + (deck_size as i128)) % (deck_size as i128);
        start = (start + (deck_size as i128)) % (deck_size as i128);
    }

    eprintln!("{} {}", start, inc);

    let ans: BigInt = BigInt::from_i64(start as i64).unwrap()
        * (1 - modexp(inc as isize, yolo, deck_size))
        * (modexp((1 - inc) as isize, deck_size - 1 - 1, deck_size))
        + 2020 * modexp(inc as isize, yolo, deck_size);

    eprintln!(
        "{:?}",
        ans.mod_floor(&BigInt::from_i64(deck_size as i64).unwrap())
    );

    ans.mod_floor(&BigInt::from_i64(deck_size as i64).unwrap())
        .to_i64()
        .unwrap() as isize
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt"), 1), 6326);
        assert_eq!(super::part_a(include_str!("input.txt"), 2), 6538);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 40522432670594);
    }
}
