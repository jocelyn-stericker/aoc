use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut pairs = HashMap::new();
    pairs.insert('(', ')');
    pairs.insert('[', ']');
    pairs.insert('{', '}');
    pairs.insert('<', '>');

    let mut points = HashMap::new();
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);

    let mut score = 0;

    for line in input.trim().split('\n') {
        let mut stack = Vec::new();
        for c in line.chars() {
            if pairs.get(&c).is_some() {
                stack.push(c);
            } else if let Some(c2) = stack.pop() {
                let expected = pairs[&c2];
                if c != expected {
                    // eprintln!("Invalid {}, expected {} but found {}", line, expected, c);
                    score += points[&c];
                    break;
                }
            } else {
                break;
            }
        }
    }
    score
}

pub fn part_b(input: &str) -> i64 {
    let mut pairs = HashMap::new();
    pairs.insert('(', ')');
    pairs.insert('[', ']');
    pairs.insert('{', '}');
    pairs.insert('<', '>');

    let mut points = HashMap::new();
    points.insert(')', 1);
    points.insert(']', 2);
    points.insert('}', 3);
    points.insert('>', 4);

    let mut scores = Vec::new();

    for line in input.trim().split('\n') {
        let mut stack = Vec::new();
        let mut corrupt = false;
        for c in line.chars() {
            if pairs.get(&c).is_some() {
                stack.push(c);
            } else if let Some(c2) = stack.pop() {
                let expected = pairs[&c2];
                if c != expected {
                    corrupt = true;
                    break;
                }
            } else {
                eprintln!("Closing characted unexpected");
                panic!();
            }
        }
        if !corrupt {
            let mut completion = String::new();

            let mut this_score = 0;
            stack.reverse();
            for s in stack {
                this_score *= 5;
                completion.push(pairs[&s]);
                this_score += points[&pairs[&s]];
            }
            eprintln!("Completing {} with {}", line, completion);
            scores.push(this_score);
        }
    }
    assert!(scores.len() % 2 == 1);
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part_a(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            ),
            26397
        );
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 323691);
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part_b(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            ),
            288957
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2858785164);
    }
}
