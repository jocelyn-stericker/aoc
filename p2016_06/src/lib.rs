use std::collections::HashMap;

pub fn part_a(input: &str) -> String {
    let mut secret = String::default();
    let lines: Vec<&str> = input.trim().split('\n').collect();
    for i in 0..lines[0].len() {
        let mut freq = HashMap::new();
        for line in &lines {
            *freq
                .entry(line.chars().skip(i).next().unwrap())
                .or_insert(0) += 1;
        }
        secret.push(*freq.iter().max_by_key(|f| f.1).unwrap().0)
    }
    secret
}

pub fn part_b(input: &str) -> String {
    let mut secret = String::default();
    let lines: Vec<&str> = input.trim().split('\n').collect();
    for i in 0..lines[0].len() {
        let mut freq = HashMap::new();
        for line in &lines {
            *freq
                .entry(line.chars().skip(i).next().unwrap())
                .or_insert(0) += 1;
        }
        secret.push(*freq.iter().min_by_key(|f| f.1).unwrap().0)
    }
    secret
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a( "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar\n"), "easter");
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), "asvcbhvg"); //5:49
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_b( "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar\n"), "advent");
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), "odqnikqv"); //7:00
    }
}
