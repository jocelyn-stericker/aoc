pub fn part_a(row: usize, col: usize) -> u64 {
    let mut x = 1;
    let mut y = 1;
    let mut v: u64 = 20151125;
    loop {
        if y == row && x == col {
            return v;
        }
        v *= 252533;
        v %= 33554393;

        if y == 1 {
            y = x + 1;
            x = 1;
        } else {
            x += 1;
            y -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(2981, 3075), 9132360);
    }
}
