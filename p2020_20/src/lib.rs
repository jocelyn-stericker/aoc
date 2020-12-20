use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Sub {
    img: Vec<Vec<char>>,
    tile: i64,
}

impl Sub {
    fn hash_top(&self) -> i64 {
        let mut sum = 0;
        let mut t = 1;
        let h = self.img.len();
        for i in 0..h {
            sum += t * if self.img[0][i] == '#' { 1 } else { 0 };

            t *= 2;
        }
        sum
    }

    fn hash_bottom(&self) -> i64 {
        let mut sum = 0;
        let mut t = 1;
        let h = self.img.len();
        for i in 0..h {
            sum += t * if self.img[h - 1][i] == '#' { 1 } else { 0 };

            t *= 2;
        }
        sum
    }

    fn hash_left(&self) -> i64 {
        let mut sum = 0;
        let mut t = 1;
        let h = self.img.len();
        for i in 0..h {
            sum += t * if self.img[i][0] == '#' { 1 } else { 0 };

            t *= 2;
        }
        sum
    }

    fn hash_right(&self) -> i64 {
        let mut sum = 0;
        let mut t = 1;
        let h = self.img.len();
        for i in 0..h {
            sum += t * if self.img[i][h - 1] == '#' { 1 } else { 0 };

            t *= 2;
        }
        sum
    }
}

fn rots(img: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let h = img.len();

    let mut n = img.clone();
    for y in 0..h {
        for x in 0..h {
            n[y][x] = img[x][h - y - 1];
        }
    }

    let mut m = img.clone();
    for y in 0..h {
        for x in 0..h {
            m[y][x] = n[x][h - y - 1];
        }
    }

    let mut o = img.clone();
    for y in 0..h {
        for x in 0..h {
            o[y][x] = m[x][h - y - 1];
        }
    }

    vec![img, n, m, o]
}

fn transforms(img: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut t = vec![];
    let h = img.len();

    for img in rots(img).into_iter() {
        // flip x
        let mut n = img.clone();
        for y in 0..h {
            for x in 0..h {
                n[y][x] = img[y][h - x - 1];
            }
        }
        t.push(n);

        // flip y
        let mut n = img.clone();
        for y in 0..h {
            for x in 0..h {
                n[y][x] = img[h - y - 1][x];
            }
        }
        t.push(n);

        // normal
        t.push(img);
    }

    t.sort();
    t.dedup();
    t
}

fn solve(
    so_far: &mut Vec<Sub>,
    available: &mut HashMap<i64, Vec<Sub>>,
    above_left: &HashMap<(i64, i64), Vec<Sub>>,
    top: &HashMap<i64, Vec<Sub>>,
    left: &HashMap<i64, Vec<Sub>>,
    h: i64,
) -> Option<i64> {
    if so_far.len() == (h * h) as usize {
        return Some(
            so_far[0].tile
                * so_far[(h - 1) as usize].tile
                * so_far[(h * (h - 1)) as usize].tile
                * so_far[(h * h - 1) as usize].tile,
        );
    }

    let p_above = (so_far.len() as i64) - h;
    let p_left = if (so_far.len() as i64) % h == 0 {
        -1
    } else {
        (so_far.len() as i64) - 1
    };

    let p_above = if p_above >= 0 {
        Some(so_far[p_above as usize].hash_bottom())
    } else {
        None
    };
    let p_left = if p_left >= 0 {
        Some(so_far[p_left as usize].hash_right())
    } else {
        None
    };

    match (p_above, p_left) {
        (None, None) => {
            let k: Vec<i64> = available.keys().cloned().collect();
            for option in k.into_iter() {
                let removed = available.remove(&option).unwrap();

                for rotation in &removed {
                    so_far.push(rotation.clone());
                    if let Some(x) = solve(so_far, available, above_left, top, left, h) {
                        return Some(x);
                    }
                    so_far.pop();
                }

                available.insert(option, removed);
            }
        }
        (Some(p_above), None) => {
            if let Some(rotations) = top.get(&p_above) {
                for rotation in rotations {
                    if let Some(removed) = available.remove(&rotation.tile) {
                        so_far.push(rotation.clone());
                        if let Some(x) = solve(so_far, available, above_left, top, left, h) {
                            return Some(x);
                        }
                        so_far.pop();

                        available.insert(rotation.tile, removed);
                    }
                }
            }
        }
        (None, Some(p_left)) => {
            if let Some(rotations) = left.get(&p_left) {
                for rotation in rotations {
                    let mut ok = true;
                    if let Some(p_above) = p_above {
                        if p_above != rotation.hash_top() {
                            ok = false;
                        }
                    }
                    if ok {
                        if let Some(removed) = available.remove(&rotation.tile) {
                            so_far.push(rotation.clone());
                            if let Some(x) = solve(so_far, available, above_left, top, left, h) {
                                return Some(x);
                            }
                            so_far.pop();

                            available.insert(rotation.tile, removed);
                        }
                    }
                }
            }
        }

        (Some(p_above), Some(p_left)) => {
            if let Some(rotations) = above_left.get(&(p_above, p_left)) {
                for rotation in rotations {
                    if let Some(removed) = available.remove(&rotation.tile) {
                        so_far.push(rotation.clone());
                        if let Some(x) = solve(so_far, available, above_left, top, left, h) {
                            return Some(x);
                        }
                        so_far.pop();

                        available.insert(rotation.tile, removed);
                    }
                }
            }
        }
    };

    None
}

fn draw(d: &[Sub], h: usize) -> String {
    let dh = d[0].img.len();

    let mut s = String::new();

    for y in 0..h * dh {
        let dy = y / dh;
        let sy = y % dh;
        if sy == 0 || sy == dh - 1 {
            continue;
        }
        for x in 0..h * dh {
            let dx = x / dh;
            let sx = x % dh;

            if sx == 0 || sx == dh - 1 {
                continue;
            }
            s.push(d[dy * h + dx].img[sy][sx]);
        }
        s.push('\n');
    }

    s
}

pub fn part_a_and_b(input: &str) -> (i64, i64) {
    // let mut sub = HashMap::new();
    let mut available = HashMap::new();
    let mut top: HashMap<i64, Vec<Sub>> = HashMap::new();
    let mut left: HashMap<i64, Vec<Sub>> = HashMap::new();
    let mut above_left: HashMap<(i64, i64), Vec<Sub>> = HashMap::new();

    for line in input.trim().split("\n\n") {
        let mut parts = line.split(":\n");
        let tile = parts
            .next()
            .unwrap()
            .split("Tile ")
            .nth(1)
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let img: Vec<Vec<char>> = parts
            .next()
            .unwrap()
            .split('\n')
            .map(|m| m.chars().collect())
            .collect();

        let mut all = Vec::new();
        for img in transforms(img.clone()) {
            let sub = Sub { img, tile };

            top.entry(sub.hash_top()).or_default().push(sub.clone());
            left.entry(sub.hash_left()).or_default().push(sub.clone());
            above_left
                .entry((sub.hash_top(), sub.hash_left()))
                .or_default()
                .push(sub.clone());
            all.push(sub);
        }

        available.insert(tile, all);
    }

    let mut so_far = Vec::new();
    let h = (available.len() as f64).sqrt() as _;
    let s = solve(&mut so_far, &mut available, &above_left, &top, &left, h).unwrap();

    let monstors = draw(&so_far, h as usize);

    let map: Vec<Vec<char>> = monstors
        .trim()
        .split('\n')
        .map(|c| c.chars().collect())
        .collect();

    let sea_monstor = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let mut b = 0;
    'a: for map in transforms(map) {
        let mut count = 0;
        let mut map2 = map.clone();
        for y in 0..map.len() {
            for x in 0..map.len() {
                let mut is_monstor = true;
                'f: for (dy, l) in sea_monstor.iter().enumerate() {
                    for (dx, c) in l.chars().enumerate() {
                        if y + dy >= map.len()
                            || x + dx >= map.len()
                            || c == '#' && map[y + dy][x + dx] != '#'
                        {
                            is_monstor = false;
                            break 'f;
                        }
                    }
                }
                if is_monstor {
                    for (dy, l) in sea_monstor.iter().enumerate() {
                        for (dx, c) in l.chars().enumerate() {
                            if c == '#' {
                                map2[y + dy][x + dx] = 'X';
                            }
                        }
                    }
                    count = 1;
                }
            }
        }

        if count > 0 {
            let mut count = 0;
            for y in 0..map2.len() {
                for x in 0..map2.len() {
                    if map2[y][x] == '#' {
                        count += 1;
                    }
                }
            }
            b = count;
            break 'a;
        }
    }
    (s, b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        assert_eq!(
            super::part_a_and_b(include_str!("sample.txt")),
            (20899048083289, 273)
        );
    }

    #[test]
    fn part_a_and_b() {
        assert_eq!(
            super::part_a_and_b(include_str!("input.txt")),
            (83775126454273, 1993)
        );
    }
}
