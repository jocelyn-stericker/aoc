// use std::collections::HashSet;

fn get_loop_size(l: i64) -> usize {
    let sn = 7;
    let mut val = 1;
    for i in 1.. {
        val *= sn;
        val %= 20201227;

        if val == l {
            return i;
        }
    }

    panic!();
}

fn encrypt(sn: i64, l: usize) -> i64 {
    let mut val = 1;
    for _ in 0..l {
        val *= sn;
        val %= 20201227;
    }

    val
}

pub fn part_a(input: &str) -> i64 {
    let mut x = input.trim().split('\n').map(|l| l.parse::<i64>().unwrap());
    let card_pub = x.next().unwrap();
    let door_pub = x.next().unwrap();

    let card_loop_size = get_loop_size(card_pub);
    let door_loop_size = get_loop_size(door_pub);

    let ek = encrypt(door_pub, card_loop_size);
    let ek2 = encrypt(card_pub, door_loop_size);

    assert!(ek == ek2);
    ek
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 6011069);
    }
}
