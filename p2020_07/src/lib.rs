use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_a(input: &str) -> i64 {
    let mut can_contain: HashMap<_, Vec<_>> = HashMap::new();
    let mut pairs: HashSet<(String, String)> = HashSet::new();
    let mut q = VecDeque::new();
    for line in input.trim().split('\n') {
        let mut columns = line.split(',');
        let outer: &str = columns.next().unwrap();
        for inner in columns.map(|c| c.chars().skip(2).collect::<String>()) {
            q.push_back((outer.to_owned(), inner.to_owned()));
            pairs.insert((outer.to_owned(), inner.to_owned()));
            can_contain.entry(outer.to_owned()).or_default().push(inner);
        }
    }
    while let Some((outer, inner)) = q.pop_front() {
        for subinner in can_contain.get(&inner).cloned().unwrap_or_default() {
            if !pairs.contains(&(outer.clone(), subinner.clone())) {
                q.push_back((outer.to_owned(), subinner.to_owned()));
                pairs.insert((outer.to_owned(), subinner.to_owned()));
                can_contain
                    .entry(outer.to_owned())
                    .or_default()
                    .push(subinner.clone());
            }
        }
    }
    let mut sum = 0;
    for (_outer, inner) in pairs {
        if inner == "shiny gold" {
            sum += 1;
        }
    }
    sum
}

fn nest(
    outer: &str,
    memo: &mut HashMap<String, usize>,
    can_contain: &HashMap<String, Vec<String>>,
) -> usize {
    if let Some(x) = memo.get(outer) {
        return *x;
    }
    let mut sum = 0;
    if let Some(contain) = can_contain.get(outer) {
        for inner in contain {
            sum += nest(inner, memo, can_contain);
        }
    } else {
    }
    sum += 1;
    memo.insert(outer.to_string(), sum);
    sum
}

pub fn part_b(input: &str) -> usize {
    let mut can_contain: HashMap<_, Vec<_>> = HashMap::new();

    for line in input.trim().split('\n') {
        let mut columns = line.split(',');
        let outer: &str = columns.next().unwrap();
        for (count, inner) in columns.map(|c| {
            let count = c
                .chars()
                .next()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
            (count, c.chars().skip(2).collect::<String>())
        }) {
            for _ in 0..count {
                can_contain
                    .entry(outer.to_owned())
                    .or_default()
                    .push(inner.clone());
            }
        }
    }

    let mut memo: HashMap<String, usize> = HashMap::new();

    nest("shiny gold", &mut memo, &can_contain) - 1
}

#[cfg(test)]
mod tests {

    #[test]
    fn example1() {
        assert_eq!(super::part_b("light red,1 bright white,2 muted yellow\ndark orange,3 bright white,4 muted yellow\nbright white,1 shiny gold\nmuted yellow,2 shiny gold,9 faded blue\nshiny gold,1 dark olive,2 vibrant plum\ndark olive,3 faded blue,4 dotted black\nvibrant plum,5 faded blue,6 dotted black\nfaded blue\ndotted black\n"), 32);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 289);
    }

    #[test]
    fn part_b() {
        // 22137
        assert_eq!(super::part_b(include_str!("input.txt")), 30055);
    }
}
