use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut sea = HashMap::new();
    let mut height = 0;
    let mut width = 0;

    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '>' | 'v' => {
                    sea.insert((y, x), c);
                }
                _ => panic!(),
            }
            width = x + 1;
        }
        height = y + 1;
    }

    let mut did_move = true;
    let mut i = 0;
    while did_move {
        // for y in 0..height {
        //     for x in 0..width {
        //         eprint!(
        //             "{}",
        //             match sea.get(&(y, x)) {
        //                 None => '.',
        //                 Some(c) => *c,
        //             }
        //         );
        //     }
        //     eprintln!();
        // }
        // eprintln!();

        did_move = false;
        let mut next_sea = HashMap::new();
        for ((y, x), c) in sea.iter() {
            if *c == 'v' || sea.contains_key(&(*y, (*x + 1) % width)) {
                next_sea.insert((*y, *x), *c);
            } else {
                next_sea.insert((*y, (*x + 1) % width), *c);
                did_move = true;
            }
        }

        sea = next_sea;
        let mut next_sea = HashMap::new();
        for ((y, x), c) in sea.iter() {
            if *c == '>' || sea.contains_key(&((*y + 1) % height, *x)) {
                next_sea.insert((*y, *x), *c);
            } else {
                next_sea.insert(((*y + 1) % height, *x), *c);
                did_move = true;
            }
        }

        sea = next_sea;

        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>\n"
            ),
            58
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 429);
    }
}
