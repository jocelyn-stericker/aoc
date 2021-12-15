use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

pub fn part_a(input: &str) -> i32 {
    let mut map = HashMap::new();
    let mut best = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((y, x), c.to_digit(10).unwrap() as i32);
            best.insert((y, x), i32::MAX);
            max_x = x;
            max_y = y;
        }
    }
    let mut visit = VecDeque::new();
    visit.push_back(((0, 0), 0));
    while let Some(((y, x), cost)) = visit.pop_front() {
        if cost < best[&(y, x)] {
            best.insert((y, x), cost);
            for (dy, dx) in [(1isize, 0), (-1, 0), (0, 1), (0, -1)] {
                let y = (y as isize) + dy;
                let x = (x as isize) + dx;
                if y >= 0 && x >= 0 && y <= max_y as isize && x <= max_x as isize {
                    visit.push_back((
                        (y as usize, x as usize),
                        cost + map[&(y as usize, x as usize)],
                    ));
                }
            }
        }
    }
    best[&(max_y, max_x)]
}

fn wrap(i: i32) -> i32 {
    let i = i % 10;
    if i == 0 {
        1
    } else {
        i
    }
}

pub fn part_b(input: &str) -> i32 {
    let mut map = HashMap::new();
    let mut best = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((y, x), c.to_digit(10).unwrap() as i32);
            best.insert((y, x), i32::MAX);
            max_x = x;
            max_y = y;
        }
    }
    let tile_width = max_x + 1;
    let tile_height = max_y + 1;

    for y_tile in 0..5 {
        for x_tile in 0..5 {
            if y_tile == 0 && x_tile == 0 {
                continue;
            }
            for y in 0..=max_y {
                for x in 0..=max_x {
                    if x_tile == 0 {
                        map.insert(
                            (y + y_tile * tile_height, x + x_tile * tile_width),
                            wrap(
                                map[&(y + (y_tile - 1) * tile_height, x + x_tile * tile_width)] + 1,
                            ),
                        );
                    } else {
                        map.insert(
                            (y + y_tile * tile_height, x + x_tile * tile_width),
                            wrap(
                                map[&(y + y_tile * tile_height, x + (x_tile - 1) * tile_width)] + 1,
                            ),
                        );
                    }

                    best.insert(
                        (y + y_tile * tile_height, x + x_tile * tile_width),
                        i32::MAX,
                    );
                }
            }
        }
    }

    let max_y = tile_height * 5 - 1;
    let max_x = tile_width * 5 - 1;

    let mut visit = BinaryHeap::new();
    visit.push((Reverse(0), (0, 0)));
    while let Some((Reverse(cost), (y, x))) = visit.pop() {
        if cost < best[&(y, x)] {
            best.insert((y, x), cost);
            for (dy, dx) in [(1isize, 0), (-1, 0), (0, 1), (0, -1)] {
                let y = (y as isize) + dy;
                let x = (x as isize) + dx;
                if y >= 0 && x >= 0 && y <= max_y as isize && x <= max_x as isize {
                    visit.push((
                        Reverse(cost + map[&(y as usize, x as usize)]),
                        (y as usize, x as usize),
                    ));
                }
            }
        }
    }
    best[&(max_y, max_x)]
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            ),
            40
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 609);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            ),
            315
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2925);
    }
}
