pub fn part_a(input: &str) -> usize {
    let goal: usize = input.trim().parse().unwrap();

    let mut max_score = 0;
    for i in 1.. {
        let i = i * 10;
        let mut score = 10;
        for factor in 2..=(i / 2) {
            if i % factor == 0 {
                score += factor * 10;
            }
        }
        score += i * 10;

        if score > max_score {
            let mut factors = Vec::new();
            for factor in 2..=(i / 2) {
                if i % factor == 0 {
                    factors.push(factor);
                }
            }
            max_score = score;
            eprintln!(
                "{} {} {} {} {:?}",
                i,
                score,
                goal,
                (score as f64) / (goal as f64) * 100.0,
                &factors
            );
        }
        if score >= goal {
            return i;
        }
    }

    panic!();
}

pub fn part_b(input: &str) -> usize {
    let goal: usize = input.trim().parse().unwrap();

    let mut max_score = 0;
    for i in 1.. {
        let i = i * 10;
        let mut score = 0;
        for factor in (i / 50)..=(i / 2) {
            if factor > 0 && i % factor == 0 && i <= factor * 50 {
                score += factor * 11;
            }
        }
        score += i * 11;

        if score > max_score {
            let mut factors = Vec::new();
            for factor in (i / 50)..=(i / 2) {
                if factor > 0 && i % factor == 0 && i <= factor * 50 {
                    factors.push(factor);
                }
            }
            max_score = score;
            eprintln!(
                "{} {} {} {} {:?}",
                i,
                score,
                goal,
                (score as f64) / (goal as f64) * 100.0,
                &factors
            );
        }
        if score >= goal {
            return i;
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 665280);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 705600);
    }
}
