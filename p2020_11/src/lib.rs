use std::collections::HashSet;

pub fn part_a(input: &str) -> usize {
    let mut walls = HashSet::new();
    let mut occupied = HashSet::new();

    let mut max_x = 0i64;
    let mut max_y = 0i64;
    for (y, line) in input.trim().split('\n').enumerate() {
        max_y = (y as i64).max(max_y);
        for (x, c) in line.chars().enumerate() {
            max_x = (x as i64).max(max_x);
            if c == '.' {
                walls.insert((x as i64, y as i64));
            }
        }
    }

    loop {
        let prev_occupied = occupied.clone();

        for y in 0i64..=max_y {
            for x in 0i64..=max_x {
                if !walls.contains(&(x, y)) {
                    if !prev_occupied.contains(&(x, y)) {
                        let mut ok = true;
                        for i in (-1i64)..=1 {
                            for j in (-1i64)..=1 {
                                if (i != 0 || j != 0) && prev_occupied.contains(&(x + i, y + j)) {
                                    ok = false;
                                }
                            }
                        }
                        if ok {
                            occupied.insert((x, y));
                        }
                    } else {
                        let mut count = 0;
                        for i in (-1i64)..=1 {
                            for j in (-1i64)..=1 {
                                if (i != 0 || j != 0) && prev_occupied.contains(&(x + i, y + j)) {
                                    count += 1;
                                }
                            }
                        }
                        if count >= 4 {
                            occupied.remove(&(x, y));
                        }
                    }
                }
            }
        }

        if prev_occupied == occupied {
            break;
        }
    }

    occupied.len()
}

pub fn part_b(input: &str) -> usize {
    let mut walls = Vec::new();
    let mut occupied = Vec::new();

    let mut max_x = 0i64;
    let mut max_y = 0i64;
    for (y, line) in input.trim().split('\n').enumerate() {
        max_y = (y as i64).max(max_y);
        for (x, c) in line.chars().enumerate() {
            max_x = (x as i64).max(max_x);
            walls.push(c == '.');
            occupied.push(false);
        }
    }

    loop {
        let prev_occupied = occupied.clone();

        for y in 0i64..=max_y {
            for x in 0i64..=max_x {
                if !walls[(y * (max_x + 1) + x) as usize] {
                    if !prev_occupied[(y * (max_x + 1) + x) as usize] {
                        let mut ok = true;
                        'a: for i in (-1i64)..=1 {
                            for j in (-1i64)..=1 {
                                if i != 0 || j != 0 {
                                    for k in 1..100 {
                                        if x + i * k < 0
                                            || y + j * k < 0
                                            || y + j * k > max_y
                                            || x + i * k > max_x
                                        {
                                            continue;
                                        }

                                        if prev_occupied
                                            [((x + i * k) + (max_x + 1) * (y + j * k)) as usize]
                                        {
                                            ok = false;
                                            break 'a;
                                        }

                                        if !walls
                                            [((x + i * k) + (max_x + 1) * (y + j * k)) as usize]
                                        {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        if ok {
                            occupied[(y * (max_x + 1) + x) as usize] = true;
                        }
                    } else {
                        let mut count = 0;
                        'b: for i in (-1i64)..=1 {
                            for j in (-1i64)..=1 {
                                if i != 0 || j != 0 {
                                    for k in 1..100 {
                                        if x + i * k < 0
                                            || y + j * k < 0
                                            || y + j * k > max_y
                                            || x + i * k > max_x
                                        {
                                            continue;
                                        }

                                        if prev_occupied
                                            [((x + i * k) + (max_x + 1) * (y + j * k)) as usize]
                                        {
                                            count += 1;
                                            if count >= 5 {
                                                break 'b;
                                            }
                                        }

                                        if !walls
                                            [((x + i * k) + (max_x + 1) * (y + j * k)) as usize]
                                        {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        if count >= 5 {
                            occupied[(y * (max_x + 1) + x) as usize] = false;
                        }
                    }
                }
            }
        }

        if prev_occupied == occupied {
            break;
        }
    }

    occupied.iter().map(|b| if *b { 1 } else { 0 }).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2243);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2027);
    }
}
