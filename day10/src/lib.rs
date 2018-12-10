use regex::Regex;
use std::mem;

fn bounding_box(input: &Vec<((i64, i64), (i64, i64))>) -> ((i64, i64), (i64, i64)) {
    (
        (
            input.iter().map(|i| (i.0).0).min().unwrap(),
            input.iter().map(|i| (i.0).1).min().unwrap(),
        ),
        (
            input.iter().map(|i| (i.0).0).max().unwrap(),
            input.iter().map(|i| (i.0).1).max().unwrap(),
        ),
    )
}

fn area(bounding: &((i64, i64), (i64, i64))) -> i64 {
    ((bounding.1).0 - (bounding.0).0) * ((bounding.1).1 - (bounding.0).1)
}

pub fn part_a(input: &str) -> String {
    let line_re = Regex::new(r"^position=<\s*(?P<dx>[-0-9]+),\s*(?P<dy>[-0-9]+)> velocity=<\s*(?P<vx>[-0-9]+),\s*(?P<vy>[-0-9]+)>").unwrap();

    let mut pts: Vec<((i64, i64), (i64, i64))> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| {
            let line = line_re.captures(line).expect("Invalid line");
            let (dx, dy) = (
                line.name("dx").unwrap().as_str().parse::<i64>().unwrap(),
                line.name("dy").unwrap().as_str().parse::<i64>().unwrap(),
            );
            let (vx, vy) = (
                line.name("vx").unwrap().as_str().parse::<i64>().unwrap(),
                line.name("vy").unwrap().as_str().parse::<i64>().unwrap(),
            );

            ((dx, dy), (vx, vy))
        })
        .collect();

    let mut bounding = bounding_box(&pts);

    for time in 0.. {
        let mut new_pts = pts
            .iter()
            .map(|pt| (((pt.0).0 + (pt.1).0, (pt.0).1 + (pt.1).1), pt.1))
            .collect();
        let mut new_bounding = bounding_box(&new_pts);

        if area(&bounding) < area(&new_bounding) {
            // The previous points are the answer!
            let offset = bounding.0;
            let max_x = (bounding.1).0 - offset.0;
            let max_y = (bounding.1).1 - offset.1;

            let mut answer: Vec<Vec<char>> = (0..=max_y)
                .map(|_| ((0..=max_x).map(|_| ' ').collect()))
                .collect();

            for pt in &pts {
                answer[((pt.0).1 - offset.1) as usize][((pt.0).0 - offset.0) as usize] = '#'
            }

            for line in &answer {
                let s: String = line.iter().collect();
                println!("{}", s);
            }
            println!("{}", time);
            return "".to_owned();
        }

        mem::swap(&mut bounding, &mut new_bounding);
        mem::swap(&mut pts, &mut new_pts);
    }

    "".to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a(include_str!("sample.txt")), "");
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), "");
    }
}
