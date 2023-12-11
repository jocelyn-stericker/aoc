use std::collections::VecDeque;

pub fn part_a(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.trim().split('\n') {
        let mut tree = vec![line
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()];
        while tree.last().unwrap().iter().any(|i| *i != 0) {
            let mut next = vec![];
            let prev = tree.last().unwrap();
            for j in 1..prev.len() {
                next.push(prev[j] - prev[j - 1]);
            }
            tree.push(next);
        }

        // extrapolate
        tree.last_mut().unwrap().push(0);
        for i in (0..tree.len() - 1).rev() {
            let next = *tree[i].last().unwrap() + *tree[i + 1].last().unwrap();
            tree[i].push(next);
        }

        let secret = *tree[0].last().unwrap();
        sum += secret;
    }
    sum
}

pub fn part_b(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.trim().split('\n') {
        let mut tree = vec![line
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<VecDeque<i64>>()];
        while tree.last().unwrap().iter().any(|i| *i != 0) {
            let mut next = VecDeque::new();
            let prev = tree.last().unwrap();
            for j in 1..prev.len() {
                next.push_back(prev[j] - prev[j - 1]);
            }
            tree.push(next);
        }

        // extrapolate
        tree.last_mut().unwrap().push_front(0);
        for i in (0..tree.len() - 1).rev() {
            let next = tree[i][0] - tree[i + 1][0];
            tree[i].push_front(next);
        }

        let secret = tree[0][0];
        sum += secret;
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45\n"
            ),
            114
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1953784198);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45\n"
            ),
            2
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 957);
    }
}
