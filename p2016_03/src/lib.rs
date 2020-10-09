pub fn part_a(input: &str) -> i64 {
    let mut possible = 0;
    for mut line in input.trim().split('\n').map(|l| {
        l.split(' ')
            .map(|l| l.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }) {
        line.sort_unstable();

        if line[0] + line[1] > line[2] {
            possible += 1;
        }
    }

    possible
}

pub fn part_b(input: &str) -> i64 {
    let mut possible = 0;
    let lines = input
        .trim()
        .split('\n')
        .map(|l| {
            l.split(' ')
                .map(|l| l.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();

    for i in 0..=2 {
        for j in 0..(lines.len() / 3) {
            let mut line = vec![lines[j * 3][i], lines[j * 3 + 1][i], lines[j * 3 + 2][i]];
            line.sort_unstable();

            if line[0] + line[1] > line[2] {
                possible += 1;
            }
        }
    }

    possible
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a("5 10 25\n"), 0);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1050); // 4:55
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 1921); // 9:16
    }
}
