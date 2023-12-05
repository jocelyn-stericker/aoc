use std::collections::HashMap;

pub fn part_a(input: &str) -> u32 {
    let mut total = 0;
    let mut rows: i64 = 0;
    let mut cols: i64 = 0;
    let grid = input
        .trim()
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    rows = rows.max(y as i64 + 1) as i64;
                    cols = cols.max(x as i64 + 1) as i64;

                    ((x as i64, y as i64), c)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    for y in 0..rows {
        for x in 0..cols {
            if grid[&(x, y)].is_digit(10) && !grid.get(&(x + 1, y)).unwrap_or(&' ').is_digit(10) {
                let mut mult = 1;
                let mut sum = 0;
                let mut start_x = 0;
                for i in (-1..=x).rev() {
                    start_x = i;
                    if let Some(digit) = grid.get(&(i, y)).and_then(|char| char.to_digit(10)) {
                        sum += mult * digit;
                        mult *= 10;
                    } else {
                        break;
                    }
                }
                let mut ok = false;
                'a: for j in (y - 1)..=(y + 1) {
                    for i in start_x..=(x + 1) {
                        if j == y && i > start_x && i <= x {
                            // digit!
                            continue;
                        }
                        if grid
                            .get(&(i, j))
                            .map(|char| *char != '.' && !char.is_digit(10))
                            .unwrap_or(false)
                        {
                            ok = true;
                            break 'a;
                        }
                    }
                }
                if ok {
                    total += sum;
                }
            }
        }
    }

    total
}

pub fn part_b(input: &str) -> i64 {
    let mut total: i64 = 0;
    let mut rows: i64 = 0;
    let mut cols: i64 = 0;
    let grid = input
        .trim()
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    rows = rows.max(y as i64 + 1) as i64;
                    cols = cols.max(x as i64 + 1) as i64;

                    ((x as i64, y as i64), c)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<_, _>>();
        let mut gears = HashMap::new();

    for y in 0..rows {
        for x in 0..cols {
            if grid[&(x, y)].is_digit(10) && !grid.get(&(x + 1, y)).unwrap_or(&' ').is_digit(10) {
                let mut mult: i64 = 1;
                let mut sum: i64 = 0;
                let mut start_x = 0;
                for i in (-1..=x).rev() {
                    start_x = i;
                    if let Some(digit) = grid.get(&(i, y)).and_then(|char| char.to_digit(10)) {
                        sum += mult * (digit as i64);
                        mult *= 10;
                    } else {
                        break;
                    }
                }
                for j in (y - 1)..=(y + 1) {
                    for i in start_x..=(x + 1) {
                        if j == y && i > start_x && i <= x {
                            // digit!
                            continue;
                        }
                        if grid
                            .get(&(i, j))
                            .map(|char| *char == '*')
                            .unwrap_or(false)
                        {
                            gears.entry((i, j)).or_insert(Vec::new()).push(sum);
                        }
                    }
                }
            }
        }
    }

    for (_, gear) in gears {
        if gear.len() == 2 {
            total += gear[0] * gear[1];
        }
    }
    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 532445);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            467835
        );
    }

    #[test]
        fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 79842967);
    }
}
